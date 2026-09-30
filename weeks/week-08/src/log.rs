#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogEvent {
    pub kind: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EventLog {
    pub events: Vec<LogEvent>,
}

impl EventLog {
    /// Create an empty event log.
    pub fn new() -> Self {
        // Steps:
        // 1. Return an `EventLog` with an empty `events` vector.
        Self { events: Vec::new() }
    }

    /// Record one structured event.
    pub fn record(&mut self, kind: &str, message: &str) {
        // Steps:
        // 1. Convert `kind` and `message` into owned strings.
        // 2. Push a `LogEvent` into `self.events`.
        self.events.push(LogEvent {
            kind: kind.to_string(),
            message: message.to_string(),
        });
    }

    /// Return true when any event has the requested kind.
    pub fn contains_kind(&self, kind: &str) -> bool {
        // Steps:
        // 1. Iterate over events.
        // 2. Return true if any event kind equals `kind`.
        self.events.iter().any(|event| event.kind == kind)
    }

    /// Return all event messages in order.
    pub fn messages(&self) -> Vec<String> {
        // Steps:
        // 1. Clone each event message.
        // 2. Preserve event order.
        // 3. Return the vector.
        self.events
            .iter()
            .map(|event| event.message.clone())
            .collect()
    }
}
