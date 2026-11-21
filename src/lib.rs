// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-software-center: mint software center component model.
//!
//! Real implementation: centers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Center {
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
pub enum MintSoftwareCenterError {
    EmptyCenterName,
    DuplicateCenter(String),
    EmptyEventSubject,
}

impl fmt::Display for MintSoftwareCenterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCenterName => write!(f, "center name must not be empty"),
            Self::DuplicateCenter(id) => write!(f, "duplicate center {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintSoftwareCenterError {}

pub struct MintSoftwareCenter {
    pub name: String,
    centers: BTreeMap<String, Center>,
    events: BTreeMap<String, Event>,
}

impl MintSoftwareCenter {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            centers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_center(&mut self, item: Center) -> Result<(), MintSoftwareCenterError> {
        if item.name.trim().is_empty() {
            return Err(MintSoftwareCenterError::EmptyCenterName);
        }
        let id = item.id.clone();
        if self.centers.contains_key(&id) {
            return Err(MintSoftwareCenterError::DuplicateCenter(id.clone()));
        }
        self.centers.insert(id, item);
        Ok(())
    }

    pub fn center_count(&self) -> usize {
        self.centers.len()
    }

    pub fn get_center(&self, id: &str) -> Option<&Center> {
        self.centers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintSoftwareCenterError> {
        if event.subject.trim().is_empty() {
            return Err(MintSoftwareCenterError::EmptyEventSubject);
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

    fn sample_center(id: &str, name: &str) -> Center {
        Center {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MintSoftwareCenter::new("MintSoftwareCenter");
        assert_eq!(m.name, "MintSoftwareCenter");
        assert_eq!(m.center_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_center() {
        let mut m = MintSoftwareCenter::new("MintSoftwareCenter");
        m.add_center(sample_center("a1", "Alpha")).unwrap();
        assert_eq!(m.center_count(), 1);
        assert!(m.get_center("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MintSoftwareCenter::new("MintSoftwareCenter");
        assert!(m.add_center(sample_center("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MintSoftwareCenter::new("MintSoftwareCenter");
        m.add_center(sample_center("x", "One")).unwrap();
        assert!(m.add_center(sample_center("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MintSoftwareCenter::new("MintSoftwareCenter");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
