#[derive(Debug, PartialEq, Eq, Clone, Copy)] // Add traits for comparison, copy, print
pub enum TrainType {
    Unknown,
    Wayfreight,
    BoxMove,
    Manifest,
    Passenger
}

impl Default for TrainType {
    fn default() -> Self {
        TrainType::Unknown
    }
}

impl TrainType {
    pub fn new(code: char) -> Self {
        if code == 'W' {
            TrainType::Wayfreight
        } else if code == 'B' {
            TrainType::BoxMove
        } else if code == 'M' {
            TrainType::Manifest
        } else if code == 'P' {
            TrainType::Passenger
        } else {
            TrainType::Unknown
        }
    }
}

use std::fmt;                                                                   
impl fmt::Display for TrainType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrainType::Wayfreight => write!(f, "<#TrainType Wayfreight (W)>"),
            TrainType::BoxMove  => write!(f, "<#TrainType BoxMove (B)>"),
            TrainType::Manifest => write!(f, "<#TrainType Manifest (M)>"),
            TrainType::Passenger => write!(f, "<#TrainType Passenger (P)>"),
            TrainType::Unknown => write!(f, "<#TrainType Unknown>"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)] // Add traits for comparison, copy, print
pub enum Stop {
    StationStop(u8),
    IndustryStop(usize),
}

impl Stop {
    pub fn newStationStop(index: u8) -> Self {
        Stop::StationStop(index)
    }
    pub fn newIndustryStop(index: usize) -> Self {
        Stop::IndustryStop(index)
    }

}

impl fmt::Display for Stop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stop::StationStop(station) => 
                write!(f, "<#StationStop ({})>", station),
            Stop::IndustryStop(industry) =>
                write!(f, "<#IndustryStop ({})>", industry),
        }
    }
}

#[derive(Debug, Default, Clone)] 
pub struct Train {
    orders: Vec<String>,
    stops: Vec<Stop>,
    name: String,
    divList: String,
    carTypes: String,
    description: String,
    shift: u8,
    maxcars: usize,
    maxclear: u8,
    maxweight: u8,
    maxlength: usize,
    onduty: u32,
    print: bool,
    done: bool,
    traintype: TrainType,
}

impl Train {
    pub fn new(name: String, divList: String,carTypes: String,
               description: String, shift: u8, maxcars: usize, maxclear: u8, 
               maxweight: u8, maxlength: usize, onduty: u32, print: bool, 
               done: bool, traintype: TrainType) -> Self {
        Self {orders: Vec::new(), stops: Vec::new(), name: name, 
              divList: divList, carTypes: carTypes, description: description, 
              shift: shift, maxcars: maxcars, maxclear: maxclear, 
              maxweight: maxweight, maxlength: maxlength, onduty: onduty, 
              print: print, done: done, traintype: traintype }
     }
     pub fn Name(&self) -> String {
        self.name.clone()
     }
     pub fn DivisionList(&self) -> String {
        self.divList.clone()
     }
     pub fn CarTypes(&self) -> String {
        self.carTypes.clone()
     }
     pub fn Description(&self) -> String {
        self.description.clone()
     }
     pub fn Shift(&self) -> u8 {
        self.shift
     }
     pub fn SetShift(&mut self, newshift: u8) {
        self.shift = newshift;
     }
     pub fn MaxCars(&self) -> usize {
        self.maxcars
     }
     pub fn MaxClear(&self) -> u8 {
        self.maxclear
     }
     pub fn MaxWeight(&self) -> u8 {
        self.maxweight
     }
     pub fn SetMaxWeight(&mut self, newmaxweight: u8) {
        self.maxweight = newmaxweight
     }
     pub fn MaxLength(&self) -> usize{
        self.maxlength
     }
     pub fn SetMaxLength(&mut self, newmaxlength: usize) {
        self.maxlength = newmaxlength;
     }
     pub fn OnDuty(&self) -> u32 {
        self.onduty
     }
     pub fn Print(&self) -> bool {
        self.print
     }
     pub fn SetPrint(&mut self, flag: bool) {
        self.print = flag;
     }
     pub fn Done(&self) -> bool {
        self.done
     }
     pub fn SetDone(&mut self, flag: bool) {
        self.done = flag;
     }
     pub fn Type(&self) -> TrainType {
        self.traintype
     }
     pub fn NumberOfOrders(&self) -> usize {
        self.orders.len()
     }
     pub fn Order(&self, i: usize) -> Option<String> {
        if i < self.orders.len() {
            Some(self.orders[i].clone())
        } else {
            None
        }
     }
     pub fn AddOrder(&mut self, order: String) {
        self.orders.push(order);
     }
     pub fn NumberOfStops(&self) -> usize {
        self.stops.len()
     }
     pub fn Stop(&self, i: usize) -> Option<&Stop> {
        if i < self.stops.len() {
            Some(&self.stops[i])
        } else {
            None
        }
     }
     pub fn AddStop(&mut self, stop: &Stop) {
        self.stops.push(stop.clone());
     }
}

impl fmt::Display for Train {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<#Train {} (Type: {}, {} stops)>", self.name, self.Type(), 
                self.NumberOfStops())
    }
}
