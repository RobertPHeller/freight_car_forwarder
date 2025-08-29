#[derive(Debug, Default, Clone)]
pub struct Owner {
    initials: String,
    name: String,
    comment: String,
}

impl Owner {
    pub fn new(initials: String, name: String, comment: String) -> Self {
        Self {initials: initials, name: name, comment: comment}
    }
    pub fn Initials(&self) -> String {self.initials.clone()}
    pub fn Name(&self) -> String {self.name.clone()}
    pub fn Comment(&self) -> String {self.comment.clone()}
}

use std::fmt;

impl fmt::Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
