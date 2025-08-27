pub use crate::division::Division;
pub use crate::station::Station;
pub use crate::train::Train;
pub use crate::industry::Industry;
pub use crate::owner::Owner;
pub use crate::car::Car;
use std::collections::HashMap;
//use std::io;
use std::io::prelude::*;
//use std::error::Error;
use std::io::BufReader;
use std::fs::File;
//use std::path::PathBuf;
use std::fs;


#[derive(Debug)]
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
    divisions: Vec<Division>,
    stations: Vec<Station>,
    trains: Vec<Train>,
    trainIndex: HashMap<String, &'system Train>,
    industries: Vec<Industry>,
    carTypesOrder: Vec<u8>,
    carTypes: Vec<u8>,
    //carGroups: Vec<Vec<u8>>,
    owners: Vec<Owner>,
    cars: Vec<Car>,
    //switchList: SwitchList,
    sessionNumber: u32,
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


impl System<'_> {
    pub fn SystemName(&self) -> String {
        self.systemName.clone()
    }
    pub fn SystemFile(&self) -> String {
        self.systemFile.clone()
    }
    pub fn IndustriesFile(&self) -> String {
        self.industriesFile.clone()
    }
    pub fn TrainsFile(&self) -> String {
        self.trainsFile.clone()
    }
    pub fn OrdersFile(&self) -> String {
        self.ordersFile.clone()
    }
    pub fn OwnersFile(&self) -> String {
        self.ownersFile.clone()
    }
    pub fn CarTypesFile(&self) -> String {
        self.carTypesFile.clone()
    }
    pub fn CarsFile(&self) -> String {
        self.carsFile.clone()
    }
    pub fn StatsFile(&self) -> String {
        self.statsFile.clone()
    }
    pub fn NumberOfDivisions(&self) -> usize {
        self.divisions.len()
    }
    pub fn NumberOfStations(&self) -> usize {
        self.stations.len()
    }
    pub fn NumberOfTrains(&self) -> usize {
        self.trains.len()
    }
    pub fn NumberOfIndustries(&self) -> usize {
        self.industries.len()
    }
    pub fn NumberOfCars(&self) -> usize {
        self.cars.len()
    }
    pub fn SessionNumber(&self) -> u32 {
        self.sessionNumber
    }
    pub fn ShiftNumber(&self) -> u8 {
        self.shiftNumber
    }
    pub fn TotalShifts(&self) -> u8 {
        self.totalShifts
    }
    pub fn NextShift(&mut self) -> u8 {
        self.shiftNumber = self.shiftNumber + 1;
        self.totalShifts = self.totalShifts + 1;
        if self.shiftNumber > 3 {
            self.sessionNumber = self.sessionNumber + 1;
            self.shiftNumber = 1;
        }
        self.shiftNumber
    }
    pub fn TotalCars(&self) -> usize {
        self.cars.len()
    }
    pub fn StatsPeriod(&self) -> u32 {
        self.statsPeriod
    }
    pub fn CarsMoved(&self) -> u32 {
        self.carsMoved
    }
    pub fn CarsAtDest(&self) -> u32 {
        self.carsAtDest
    }
    pub fn CarsNotMoved(&self) -> u32 {
        self.carsNotMoved
    }
    pub fn CarsMovedOnce(&self) -> u32 {
        self.carsMovedOnce
    }
    pub fn CarsMovedTwice(&self) -> u32 {
        self.carsMovedTwice
    }
    pub fn CarsMovedThree(&self) -> u32 {
        self.carsMovedThree
    }
    pub fn CarsMovedMore(&self) -> u32 {
        self.carsMovedMore
    }
    pub fn CarMovements(&self) -> u32 {
        self.carMovements
    }
    pub fn CarsInTransit(&self) -> u32 {
        self.carsInTransit
    }
    pub fn CarsAtWorkBench(&self) -> u32 {
        self.carsAtWorkBench
    }
    pub fn CarsAtDest_CarsInTransit(&self) -> u32 {
        self.carsAtDest_carsInTransit
    }
    pub fn PrintYards(&self) -> bool {
        self.printYards
    }
    pub fn SetPrintYards(&mut self,flag: bool) {
        self.printYards = flag;
    }
    pub fn PrintAlpha(&self) -> bool {
        self.printAlpha
    }
    pub fn SetPrintAlpha(&mut self,flag: bool) {
        self.printAlpha = flag;
    }
    pub fn PrintAtwice(&self) -> bool {
        self.printAtwice
    }
    pub fn SetPrintAtwice(&mut self,flag: bool) {
        self.printAtwice = flag;
    }
    pub fn PrintList(&self) -> bool {
        self.printList
    }
    pub fn SetPrintList(&mut self,flag: bool) {
        self.printList = flag;
    }
    pub fn PrintLtwice(&self) -> bool {
        self.printLtwice
    }
    pub fn SetPrintLtwice(&mut self,flag: bool) {
        self.printLtwice = flag;
    }
    pub fn PrintDispatch(&self) -> bool {
        self.printDispatch
    }
    pub fn SetPrintDispatch(&mut self,flag: bool) {
        self.printDispatch = flag;
    }
    pub fn Printem(&self) -> bool {
        self.printem
    }
    pub fn SetPrintem(&mut self,flag: bool) {
        self.printem = flag;
    }
    
    fn SkipCommentsReadLine(reader: &mut BufReader<File>) -> 
            std::io::Result<String> {
        let mut buffer = String::new();
        loop {
            reader.read_line(&mut buffer)?;
            buffer = buffer.trim().to_string();
            if buffer.len() > 0 && !buffer.starts_with("'") {
                break;
            }
         }
         Ok(buffer)
     }
                

    pub fn new(systemfile: String) -> Self {
        let systemfilePath = fs::canonicalize(systemfile)
                .expect("Path not found");
        //let systemDirectory = systemfilePath.with_file_name("");
        let f = File::open(systemfilePath.to_str().unwrap())
                .expect("Cannot open file");
        let mut reader = BufReader::new(f);
        let systemname = Self::SkipCommentsReadLine(&mut reader).expect("Read error");
        let industriesfile = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
        let trainsfile = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
        let ordersfile = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
        let ownersfile = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
        let cartypesfile = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
        let carsfile = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
        let statsfile = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
        let divisions: Vec<Division> = Vec::new();
        let stations: Vec<Station> = Vec::new();
        let trains: Vec<Train> = Vec::new();
        let trainindex: HashMap<String, &Train> = HashMap::new();
        let industries: Vec<Industry> = Vec::new();
        let cartypesorder: Vec<u8> = Vec::new();
        let cartypes: Vec<u8> = Vec::new();
        let owners: Vec<Owner> = Vec::new();
        let cars: Vec<Car> = Vec::new();
        let sessionNumber: u32 = 0;
        let shiftnumber: u8 = 1;
        let totalshifts: u8 = 0;
        let ranalltrains: u32 = 0;
        let totalpickups: u32 = 0;
        let totalloads: u32 = 0;
        let totaltons: u32 = 0;
        let totalrevenuetons: u32 = 0;
        let trainPrintOK: bool = false;
        let wayFreight: bool = false;
        let deliver: bool = false;
        let trainlength: u32 = 0;
        let numbercars: u32 = 0;
        let traintons: u32 = 0;
        let trainloads: u32 = 0;
        let trainempties: u32 = 0;
        let trainlongest: u32 = 0;
        let statsperiod: u32 = 0;
        let carsmoved: u32 = 0;
        let carsatdest: u32 = 0;
        let carsnotmoved: u32 = 0;
        let carsmovedonce: u32 = 0;
        let carsmovedtwice: u32 = 0;
        let carsmovedthree: u32 = 0;
        let carsmovedmore: u32 = 0;
        let carmovements: u32 = 0;
        let carsintransit: u32 = 0;
        let carsatworkbench: u32 = 0;
        let carsatdest_carsInTransit: u32 = 0;
        let printYards: bool = false;
        let printAlpha: bool = false;
        let printAtwice: bool = false;
        let printList: bool = false;
        let printLtwice: bool = false;
        let printDispatch: bool = false;
        let printem: bool = false;
        let messageBuffer: String = String::from("");
        Self {systemFile: systemfilePath.to_str().unwrap().to_string(), 
              systemName: systemname.clone(), 
              industriesFile: industriesfile.to_str().unwrap().to_string(), 
              trainsFile: trainsfile.to_str().unwrap().to_string(), 
              ordersFile: ordersfile.to_str().unwrap().to_string(), 
              ownersFile: ownersfile.to_str().unwrap().to_string(), 
              carTypesFile: cartypesfile.to_str().unwrap().to_string(), 
              carsFile: carsfile.to_str().unwrap().to_string(), 
              statsFile: statsfile.to_str().unwrap().to_string(), 
              divisions: divisions, stations: stations, 
              trains: trains, trainIndex: trainindex, industries: industries, 
              carTypesOrder: cartypesorder, carTypes: cartypes, owners: owners,
              cars: cars, sessionNumber: sessionNumber, 
              shiftNumber: shiftnumber, totalShifts: totalshifts, 
              ranAllTrains: ranalltrains, totalPickups: totalpickups, 
              totalLoads: totalloads, totalTons: totaltons, 
              totalRevenueTons: totalrevenuetons, trainPrintOK: trainPrintOK, 
              wayFreight: wayFreight, deliver: deliver, 
              trainLength: trainlength, numberCars: numbercars, 
              trainTons: traintons, trainLoads: trainloads, 
              trainEmpties: trainempties, trainLongest: trainlongest, 
              statsPeriod: statsperiod, carsMoved: carsmoved, 
              carsAtDest: carsatdest, carsNotMoved: carsnotmoved, 
              carsMovedOnce: carsmovedonce, carsMovedTwice: carsmovedtwice, 
              carsMovedThree: carsmovedthree, carsMovedMore: carsmovedmore, 
              carMovements: carmovements, carsInTransit: carsintransit, 
              carsAtWorkBench: carsatworkbench, 
              carsAtDest_carsInTransit: carsatdest_carsInTransit, 
              printYards: printYards, printAlpha: printAlpha, 
              printAtwice: printAtwice, printList: printList, 
              printLtwice: printLtwice, printDispatch:printDispatch, 
              printem: printem, messageBuffer: messageBuffer }
    }
    
}
