use std::collections::HashMap;

pub(crate) struct SectionViolations {
    inner: HashMap<String, Vec<String>>,
}

impl SectionViolations {
    pub(crate) fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub(crate) fn upsert(&mut self, cell_group_descriptor: &str, violation: &str) {
        if let Some(violations) = self.inner.get_mut(cell_group_descriptor) {
            violations.push(violation.to_string());
        } else {
            self.inner.insert(
                cell_group_descriptor.to_string(),
                vec![violation.to_string()],
            );
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
