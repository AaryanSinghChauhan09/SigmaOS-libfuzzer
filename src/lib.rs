// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-libfuzzer: libfuzzer component model.
//!
//! Real implementation: libfuzzer_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibfuzzerItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibfuzzerError {
    EmptyLibfuzzerItemName,
    DuplicateLibfuzzerItem(String),
    EmptyEventSubject,
}

impl fmt::Display for LibfuzzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLibfuzzerItemName => write!(f, "libfuzzer_item name must not be empty"),
            Self::DuplicateLibfuzzerItem(id) => write!(f, "duplicate libfuzzer_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LibfuzzerError {}

pub struct Libfuzzer {
    pub name: String,
    libfuzzer_items: BTreeMap<String, LibfuzzerItem>,
    events: BTreeMap<String, Event>,
}

impl Libfuzzer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            libfuzzer_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_libfuzzer_item(&mut self, item: LibfuzzerItem) -> Result<(), LibfuzzerError> {
        if item.name.trim().is_empty() {
            return Err(LibfuzzerError::EmptyLibfuzzerItemName);
        }
        let id = item.id.clone();
        if self.libfuzzer_items.contains_key(&id) {
            return Err(LibfuzzerError::DuplicateLibfuzzerItem(id.clone()));
        }
        self.libfuzzer_items.insert(id, item);
        Ok(())
    }

    pub fn libfuzzer_item_count(&self) -> usize {
        self.libfuzzer_items.len()
    }

    pub fn get_libfuzzer_item(&self, id: &str) -> Option<&LibfuzzerItem> {
        self.libfuzzer_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LibfuzzerError> {
        if event.subject.trim().is_empty() {
            return Err(LibfuzzerError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_libfuzzer_item(id: &str, name: &str) -> LibfuzzerItem {
        LibfuzzerItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Libfuzzer::new("Libfuzzer");
        assert_eq!(m.name, "Libfuzzer");
        assert_eq!(m.libfuzzer_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_libfuzzer_item() {
        let mut m = Libfuzzer::new("Libfuzzer");
        m.add_libfuzzer_item(sample_libfuzzer_item("a1", "Alpha")).unwrap();
        assert_eq!(m.libfuzzer_item_count(), 1);
        assert!(m.get_libfuzzer_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Libfuzzer::new("Libfuzzer");
        assert!(m.add_libfuzzer_item(sample_libfuzzer_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Libfuzzer::new("Libfuzzer");
        m.add_libfuzzer_item(sample_libfuzzer_item("x", "One")).unwrap();
        assert!(m.add_libfuzzer_item(sample_libfuzzer_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Libfuzzer::new("Libfuzzer");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
