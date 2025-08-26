//use crate::Owner::*;
//use crate::Train::*;
//use crate::Industry::*;

#[derive(Debug)]
pub struct Car {
//    owner: &Owner::Owner,
//    lasttrain: &Train::Train,
//    prevtrain: &Train::Train,
//    location: &Industry::Industry,
//    destination: &Industry::Industry,
    marks: String,
    number: String,
    divisions: String,
    length: u8,
    plate: u8,
    weightclass: u8,
    ltwt: u8,
    ldlmt: u8,
    trips: u32,
    moves: u32,
    assignments: u32,
    loadedP: bool,
    mirrorP: bool,
    fixedP: bool,
    doneP: bool,
    peek: bool,
    tmpStatus: bool,
    cartype: u8,
}
