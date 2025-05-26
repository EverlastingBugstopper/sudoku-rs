use std::collections::BTreeMap;

pub(crate) struct SectionViolations {
    inner: BTreeMap<String, Vec<String>>,
}

impl SectionViolations {
    pub(crate) fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub(crate) fn upsert(&mut self, cell_group_descriptor: String, violation: String) {
        if let Some(violations) = self.inner.get_mut(&cell_group_descriptor) {
            violations.push(violation);
        } else {
            self.inner.insert(cell_group_descriptor, vec![violation]);
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub(crate) fn result(&self) -> Vec<String> {
        let mut result = Vec::new();
        for (cell_group_descriptor, violations) in &self.inner {
            result.push(format!(
                "{cell_group_descriptor}: {}",
                violations.join(", ")
            ));
        }
        result
    }
}
