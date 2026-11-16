// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-mint-command-processor: mint command processor component model.
//!
//! Real implementation: processors, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Processor {
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
pub enum MintCommandProcessorError {
    EmptyProcessorName,
    DuplicateProcessor(String),
    EmptyEventSubject,
}

impl fmt::Display for MintCommandProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProcessorName => write!(f, "processor name must not be empty"),
            Self::DuplicateProcessor(id) => write!(f, "duplicate processor {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintCommandProcessorError {}

pub struct MintCommandProcessor {
    pub name: String,
    processors: BTreeMap<String, Processor>,
    events: BTreeMap<String, Event>,
}

impl MintCommandProcessor {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            processors: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_processor(&mut self, item: Processor) -> Result<(), MintCommandProcessorError> {
        if item.name.trim().is_empty() {
            return Err(MintCommandProcessorError::EmptyProcessorName);
        }
        let id = item.id.clone();
        if self.processors.contains_key(&id) {
            return Err(MintCommandProcessorError::DuplicateProcessor(id.clone()));
        }
        self.processors.insert(id, item);
        Ok(())
    }

    pub fn processor_count(&self) -> usize {
        self.processors.len()
    }

    pub fn get_processor(&self, id: &str) -> Option<&Processor> {
        self.processors.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintCommandProcessorError> {
        if event.subject.trim().is_empty() {
            return Err(MintCommandProcessorError::EmptyEventSubject);
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

    fn sample_processor(id: &str, name: &str) -> Processor {
        Processor {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MintCommandProcessor::new("MintCommandProcessor");
        assert_eq!(m.name, "MintCommandProcessor");
        assert_eq!(m.processor_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_processor() {
        let mut m = MintCommandProcessor::new("MintCommandProcessor");
        m.add_processor(sample_processor("a1", "Alpha")).unwrap();
        assert_eq!(m.processor_count(), 1);
        assert!(m.get_processor("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MintCommandProcessor::new("MintCommandProcessor");
        assert!(m.add_processor(sample_processor("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MintCommandProcessor::new("MintCommandProcessor");
        m.add_processor(sample_processor("x", "One")).unwrap();
        assert!(m.add_processor(sample_processor("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MintCommandProcessor::new("MintCommandProcessor");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
