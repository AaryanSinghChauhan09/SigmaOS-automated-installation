// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-automated-installation: automated installation component model.
//!
//! Real implementation: installations, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Installation {
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
pub enum AutomatedInstallationError {
    EmptyInstallationName,
    DuplicateInstallation(String),
    EmptyEventSubject,
}

impl fmt::Display for AutomatedInstallationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInstallationName => write!(f, "installation name must not be empty"),
            Self::DuplicateInstallation(id) => write!(f, "duplicate installation {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AutomatedInstallationError {}

pub struct AutomatedInstallation {
    pub name: String,
    installations: BTreeMap<String, Installation>,
    events: BTreeMap<String, Event>,
}

impl AutomatedInstallation {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            installations: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_installation(&mut self, item: Installation) -> Result<(), AutomatedInstallationError> {
        if item.name.trim().is_empty() {
            return Err(AutomatedInstallationError::EmptyInstallationName);
        }
        let id = item.id.clone();
        if self.installations.contains_key(&id) {
            return Err(AutomatedInstallationError::DuplicateInstallation(id.clone()));
        }
        self.installations.insert(id, item);
        Ok(())
    }

    pub fn installation_count(&self) -> usize {
        self.installations.len()
    }

    pub fn get_installation(&self, id: &str) -> Option<&Installation> {
        self.installations.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AutomatedInstallationError> {
        if event.subject.trim().is_empty() {
            return Err(AutomatedInstallationError::EmptyEventSubject);
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

    fn sample_installation(id: &str, name: &str) -> Installation {
        Installation {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = AutomatedInstallation::new("AutomatedInstallation");
        assert_eq!(m.name, "AutomatedInstallation");
        assert_eq!(m.installation_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_installation() {
        let mut m = AutomatedInstallation::new("AutomatedInstallation");
        m.add_installation(sample_installation("a1", "Alpha")).unwrap();
        assert_eq!(m.installation_count(), 1);
        assert!(m.get_installation("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = AutomatedInstallation::new("AutomatedInstallation");
        assert!(m.add_installation(sample_installation("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = AutomatedInstallation::new("AutomatedInstallation");
        m.add_installation(sample_installation("x", "One")).unwrap();
        assert!(m.add_installation(sample_installation("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = AutomatedInstallation::new("AutomatedInstallation");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
