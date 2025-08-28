//use crate::station::Station;
//use crate::industry::Industry;

#[derive(Default, Debug, Clone)]
pub struct Division {
    name: String,
    //stations: Vec<&'system Station>,
    //home: &'system Industry,
    home_index: usize,
    symbol: char,
    area: char,
}

impl Division {
    pub fn new(name: String, home_index: usize, symbol: char, area: char) -> Self {
        Self {name: name,
              home_index: home_index,
              symbol: symbol,
              area: area,
         }
    }
    pub fn Name(&self) -> String {self.name.clone()}
    pub fn Home(&self) -> usize {self.home_index}
    pub fn Symbol(&self) -> char {self.symbol}
    pub fn Area(&self) -> char {self.area}
}

use std::fmt;
impl fmt::Display for Division {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<#Division {} (Symbol: {})>", self.name, self.symbol)
    }
}

