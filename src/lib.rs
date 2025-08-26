pub mod Station;
//use crate::Station::*;
pub mod Division;
//use crate::Division::*;
pub mod Train;
//use crate::Train::*;
pub mod Industry;
//use crate::Industry::*;
pub mod CarType;
//use crate::CarType::*;
pub mod Owner;
//use crate::Owner::*;
pub mod Car;
//use crate::Car::*;
pub mod SwitchList;
//use crate::SwitchList::*;

pub struct System<'system> {
    systemFile: String,
    systemName: String,
    industriesFile: String,
    trainsFile: String,
    ordersFile: String,
    ownersFile: String,
    carTypesFile: String,
    carsFile: String,
    statsFile: String,
    divisions: Vec<Division::Division<'system>>,
    stations: Vec<Station::Station<'system>>,
    trains: Vec<Train::Train>,
    trainIndex: Vec<String>,
    industries: Vec<Industry::Industry>,
    carTypesOrder: Vec<u8>,
    carTypes: Vec<u8>,
    //carGroups: Vec<Vec<u8>>,
    owners: Vec<Owner::Owner>,
    cars: Vec<Car::Car>,
    //switchList: SwitchList,
    shiftNumber: u8,
    totalShifts: u8,
    ranAllTrains: u32,
    totalPickups: u32,
    totalLoads: u32,
    totalTons: u32,
    totalRevenueTons: u32,
    trainPrintOK: bool,
    wayFreight: bool,
    deliver: bool,
    trainLength: u32,
    numberCars: u32,
    trainTons: u32,
    trainLoads: u32,
    trainEmpties: u32,
    trainLongest: u32,
    //curDiv &Division::Division;
    //originYard: &Industry::Industry,
    //trainLastLocation: &Industry::Industry,
    //carDest: &Industry::Industry,
    statsPeriod: u32,
    carsMoved: u32,
    carsAtDest: u32,
    carsNotMoved: u32,
    carsMovedOnce: u32,
    carsMovedTwice: u32,
    carsMovedThree: u32,
    carsMovedMore: u32,
    carMovements: u32,
    carsInTransit: u32,
    carsAtWorkBench: u32,
    carsAtDest_carsInTransit: u32,
    printYards: bool,
    printAlpha: bool,
    printAtwice: bool,
    printList: bool,
    printLtwice: bool,
    printDispatch: bool,
    printem: bool,
    messageBuffer: String,
    //whitespace: String,
    //indScrapYard: &Industry::Industry,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
