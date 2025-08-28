
#[derive(Default, Debug, Clone)]
pub struct Station {
    name: String,
    comment: String,
    division_index: u8,
}

impl Station {
    pub fn new(name: String, comment: String, division_index: u8) -> Self {
        Self {name: name, comment: comment, division_index: division_index}
    }
    pub fn Name(&self) -> String {self.name.clone()}
    pub fn Comment(&self) -> String {self.comment.clone()}
    pub fn DivisionIndex(&self) -> u8 {self.division_index}
}

use std::fmt;
impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<#Station {}>", self.name)
    }
}
