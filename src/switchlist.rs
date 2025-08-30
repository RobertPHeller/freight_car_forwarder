use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)] // Add traits for comparison, copy, print
pub enum StationOrIndustry {
    StationStop(u8),
    IndustryStop(usize),
    None,
}

impl fmt::Display for StationOrIndustry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StationOrIndustry::StationStop(station) =>
                    write!(f, "<#Station ({})>", station),
            StationOrIndustry::IndustryStop(industry) =>
                    write!(f, "<#Industry ({})>", industry),
            StationOrIndustry::None =>
                    write!(f, "<#None>"),
        }
    }
}

impl StationOrIndustry {
    pub fn newStationStop(index: u8) -> Self {
        StationOrIndustry::StationStop(index)
    }
    pub fn newIndustryStop(index: usize) -> Self {
        StationOrIndustry::IndustryStop(index)
    }
}

impl Default for StationOrIndustry {
    fn default() -> Self {
        StationOrIndustry::None
    }
}

#[derive(Debug, Clone)]
pub struct SwitchListElement {
    pickLoc: usize,
    pickCar: usize,
    pickTrain: usize,
    lastTrain: usize,
    dropStop: StationOrIndustry,
}

impl fmt::Display for SwitchListElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<#SwitchListElement>")
    }
}

impl Default for SwitchListElement {
    fn default() -> Self {
        Self {pickLoc: 0, pickCar: 0, pickTrain: 0, lastTrain: 0, 
              dropStop: StationOrIndustry::None }
    }
}

impl SwitchListElement {
    pub fn newStation(pickLoc: usize, pickCar: usize, pickTrain: usize, 
                      lastTrain: usize, station: u8) -> Self {
        Self {pickLoc: pickLoc, pickCar: pickCar, pickTrain: pickTrain,
              lastTrain: lastTrain, 
              dropStop: StationOrIndustry::newStationStop(station) }
    }
    pub fn newIndustry(pickLoc: usize, pickCar: usize, pickTrain: usize, 
                      lastTrain: usize, industry: usize) -> Self {
        Self {pickLoc: pickLoc, pickCar: pickCar, pickTrain: pickTrain,
              lastTrain: lastTrain, 
              dropStop: StationOrIndustry::newIndustryStop(industry) }
    }
    pub fn PickLocation(&self) -> usize {self.pickLoc}
    pub fn PickCar(&self) -> usize {self.pickCar}
    pub fn PickTrain(&self) -> usize {self.pickTrain}
    pub fn LastTrain(&self) -> usize {self.lastTrain}
    pub fn DropStopIndustry(&self) -> Option<usize> {
        if self.pickTrain == 0 {return None;}
        match self.dropStop {
            StationOrIndustry::StationStop(station) => None,
            StationOrIndustry::IndustryStop(industry) => Some(industry),
            StationOrIndustry::None => None,
        }
    }
    pub fn DropStopStation(&self) -> Option<u8> {
        if self.pickTrain == 0 {return None;}
        match self.dropStop {
            StationOrIndustry::StationStop(station) => Some(station),
            StationOrIndustry::IndustryStop(industry) => None,
            StationOrIndustry::None => None, 
        }
    }
}
