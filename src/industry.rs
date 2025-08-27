//use crate::Car::*;
//use crate::Station::*;

#[derive(Debug)]
pub struct Industry {
//    cars: Vec<&Car::Car>,
//    station: &Station::Station,
//    mirror: &Industry,
    name: String,
    loadTypes: String,
    emptyTypes: String,
    divisionControlList: String,
    trackLen: u32,
    assignLen: u32,
    priority: u8,
    plate: u8,
    weightclass: u8,
    maxCarLen: u8,
    carsNum: u8,
    carsLen: u8,
    statsLen: u32,
    usedLen: u32,
    remLen: u32,
    reload: bool,
    indtype: u8,
    hazard: u8,
}
