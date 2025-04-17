use std::fmt;

use super::cell::Cell;

#[derive(Clone, Copy)]
pub struct Row {
    inner: [Cell; 9],
}

impl Row {
    pub fn empty() -> Self {
        Self {
            inner: [Cell::empty(); 9],
        }
    }

    pub fn set(&mut self, cell: Cell, col_idx: usize) {
        self.inner[col_idx] = cell;
    }

    pub fn cell(&self, col_idx: usize) -> &Cell {
        &self.inner[col_idx]
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "┃")?;
        for (cell_idx, cell_contents) in self.inner.iter().enumerate() {
            let separator = match cell_idx {
                2 | 5 | 8 => '┃',
                _ => '│',
            };
            write!(f, " {} {}", cell_contents, separator)?;
        }
        writeln!(f)?;
        Ok(())
    }
}
