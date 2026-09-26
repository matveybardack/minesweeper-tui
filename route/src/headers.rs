use std::path::Iter;

/// HTTP-headers - pairs of "key"- "value"
/// case-insensitive
#[derive(Debug, Clone, Default)]
pub struct Headers {
    entries: Vec<(String, String)>,
}

impl Headers {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Add/Replace header.
    pub fn insert(&mut self, name: impl Into<String>, value: impl Into<String>) {
        let name = name.into();
        let value = value.into();
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|(n, _)| n.eq_ignore_ascii_case(&name))
        {
            entry.1 = value;
        } else {
            self.entries.push((name, value));
        }
    }

    /// Returns value by key
    pub fn get(&self, name: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    /// iterator
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries.iter().map(|(n, v)| (n.as_str(), v.as_str()))
    }

    /// Mutable iterator
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (String, String)> {
        self.entries
            .iter_mut()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
    }
}
