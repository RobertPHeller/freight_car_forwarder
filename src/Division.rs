//use crate::Station::*;
//use crate::Industry::*;

#[derive(Debug)]
pub struct Division<'system> {
    name: String,
//    stations: Vec<&Station::Station>,
//    home: &'system Industry<'system>::Industry,
    symbol: u8,
    area: u8,
}
