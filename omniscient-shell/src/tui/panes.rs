//! Pane definitions for the dashboard

pub struct Pane {
    pub name: String,
    pub visible: bool,
}

impl Pane {
    pub fn new(name: impl Into<String>) -> Self {
        Pane {
            name: name.into(),
            visible: true,
        }
    }
}
