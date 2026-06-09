//! Session manager types and state

use std::collections::HashMap;
use std::time::Instant;
use chromiumoxide::Page;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SessionKind { Tab, Context, Browser }

pub struct Session {
    pub page: Page,
    pub kind: SessionKind,
    pub url: String,
    pub active_at: Instant,
}

pub struct State {
    pub sessions: HashMap<String, Session>
}
