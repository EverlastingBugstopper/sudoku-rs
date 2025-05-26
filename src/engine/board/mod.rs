use std::{fmt, str::FromStr};

mod cell;
mod number;
mod row;
mod violations;

use violations::SectionViolations;

use {cell::Cell, row::Row};

type CellGroup<'a> = Vec<&'a Cell>;

#[derive(Debug)]
enum CellGroupKind {
    Row,
    Region,
    Column,
}

pub struct Board {
    inner: [Row; 9],
}

impl Board {
    pub fn empty() -> Self {
        Self {
            inner: [Row::empty(); 9],
        }
    }

    pub fn set(&mut self, cell: Cell, row_idx: usize, col_idx: usize) {
        self.inner[row_idx].set(cell, col_idx);
    }

    pub fn check_solution(&self) -> Result<(), Vec<String>> {
        let mut section_violations = SectionViolations::new();

        for (cell_group_kind, cell_groups) in [
            (CellGroupKind::Row, self.rows()),
            (CellGroupKind::Column, self.columns()),
            (CellGroupKind::Region, self.regions()),
        ] {
            for (cell_group_idx, cell_group) in cell_groups.iter().enumerate() {
                let cell_group_descriptor = format!("{cell_group_kind:?} {}", cell_group_idx + 1);
                let mut needed_values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
                for cell in cell_group {
                    if let Some(value) = cell.value() {
                        if let Some(needed_value_idx) = needed_values
                            .iter()
                            .position(|needed_value| needed_value == &value)
                        {
                            needed_values.remove(needed_value_idx);
                        } else {
                            section_violations.upsert(
                                cell_group_descriptor.clone(),
                                format!("multiple {value}s"),
                            );
                        }
                    }
                }
                for unused_value in &needed_values {
                    section_violations
                        .upsert(cell_group_descriptor.clone(), format!("no {unused_value}s"));
                }
            }
        }

        if section_violations.is_empty() {
            Ok(())
        } else {
            let result = section_violations.result();
            Err(result)
        }
    }

    fn write_top_border(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "┏━━━┯━━━┯━━━┳━━━┯━━━┯━━━┳━━━┯━━━┯━━━┓")
    }

    fn write_bottom_border(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "┗━━━┷━━━┷━━━┻━━━┷━━━┷━━━┻━━━┷━━━┷━━━┛")
    }

    fn write_thin_inner_border(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨")
    }

    fn write_thick_inner_border(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫")
    }

    fn rows(&self) -> Vec<CellGroup> {
        (0..=8).map(|row_idx| self.row(row_idx)).collect()
    }

    fn columns(&self) -> Vec<CellGroup> {
        (0..=8).map(|col_idx| self.column(col_idx)).collect()
    }

    fn regions(&self) -> Vec<CellGroup> {
        (0..=8).map(|region_idx| self.region(region_idx)).collect()
    }

    fn row(&self, row_idx: usize) -> CellGroup {
        (0..=8)
            .map(|col_idx| self.inner[row_idx].cell(col_idx))
            .collect()
    }

    fn column(&self, col_idx: usize) -> CellGroup {
        (0..=8)
            .map(|row_idx| self.inner[row_idx].cell(col_idx))
            .collect()
    }

    fn region(&self, region_idx: usize) -> CellGroup {
        let (row_range, col_range) = match region_idx {
            0 => (0..=2, 0..=2),
            1 => (0..=2, 3..=5),
            2 => (0..=2, 6..=8),
            3 => (3..=5, 0..=2),
            4 => (3..=5, 3..=5),
            5 => (3..=5, 6..=8),
            6 => (6..=8, 0..=2),
            7 => (6..=8, 3..=5),
            8 => (6..=8, 6..=8),
            other => panic!(
                "Region {} does not exist, index must be 0 through 8.",
                other
            ),
        };

        row_range
            .flat_map(|row_idx| {
                col_range
                    .clone()
                    .map(move |col_idx| self.inner[row_idx].cell(col_idx))
            })
            .collect()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_top_border(f)?;
        for (row_idx, row_contents) in self.inner.iter().enumerate() {
            write!(f, "{}", row_contents)?;
            match row_idx {
                2 | 5 => self.write_thick_inner_border(f),
                8 => self.write_bottom_border(f),
                _ => self.write_thin_inner_border(f),
            }?;
        }
        Ok(())
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<&str> = input.split('\n').collect();
        if rows.last().map(|s| s.trim()) == Some("") {
            rows.pop();
        }
        let num_rows = rows.len();
        if num_rows != 9 {
            return Err(format!("Input contains {} rows, expected 9.", num_rows));
        }

        let mut board = Board::empty();

        for (row_idx, row_contents) in rows.iter().enumerate() {
            let cells: Vec<&str> = row_contents.split(",").collect();
            let num_cells = cells.len();
            if num_cells != 9 {
                return Err(format!(
                    "Row {} contained {} cells, expected 9.",
                    row_idx + 1,
                    num_cells
                ));
            }

            for (cell_idx, cell_contents) in cells.iter().enumerate() {
                let cell = Cell::from_str(cell_contents)?;
                board.set(cell, row_idx, cell_idx)
            }
        }

        Ok(board)
    }
}
