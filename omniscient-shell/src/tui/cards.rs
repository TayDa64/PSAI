//! Card widgets for displaying content

pub struct Card {
    pub title: String,
    pub content: String,
}

impl Card {
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Card {
            title: title.into(),
            content: content.into(),
        }
    }
}
