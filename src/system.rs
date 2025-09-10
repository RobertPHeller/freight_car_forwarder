// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-09-02 15:15:09
//  Last Modified : <250910.1552>
//
//  Description	
//
//  Notes
//
//  History
//	
/////////////////////////////////////////////////////////////////////////////
//    Copyright (C) 2025  Robert Heller D/B/A Deepwoods Software
//			51 Locke Hill Road
//			Wendell, MA 01379-9728
//
//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program; if not, write to the Free Software
//    Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
// 
//
//////////////////////////////////////////////////////////////////////////////
pub use crate::division::*;
pub use crate::station::*;
pub use crate::train::*;
pub use crate::industry::*;
pub use crate::owner::*;
pub use crate::car::*;
pub use crate::cartype::*;
pub use crate::switchlist::*;
pub use crate::fcfprintpdf::*;
use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::io::*;
//use std::io::prelude::*;
//use std::error::Error;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;
use std::path::PathBuf;
use std::fs;
use rand::prelude::*;
use std::sync::Arc;

/// Scrap yard index (cars marked as scrap).
const IND_SCRAP_YARD: usize = 999;
/// RIP track (aka workbench) cars taken out of service for repairs.
const IND_RIP_TRACK: usize = 1;

/// This is the main Freight Car Forwarder struct.
///
/// It implements all of the basic data and algorithms used in the the 
/// Freight Car Forwarder system.
///
/// This class includes code to load a model railroad "system"
/// (divisions, stations, industries, cars, and trains) along with code to
/// assign cars to trains, run trains, generate yard switch lists, and
/// various reports.  Basically everything you need run realistic trains
/// on a layout.
///
/// This is my third port of Tim O'Connors Freight Car Forwarding system,
/// originally written in QBasic for use with the North Shore Model RR
/// Club "Chesapeake System".
///
/// Author: Robert Heller <heller@deepsoft.com>
///
///
#[derive(Debug)]
pub struct System {
    /// Full pathname of the system file.
    systemFile: String,             
    /// The system name.
    systemName: String,             
    /// Full pathname of the industries file.
    industriesFile: String,         
    /// Full pathname of the trains file.
    trainsFile: String,             
    /// Full pathname of the train orders file. 
    ordersFile: String,             
    /// Full pathname of the car owners file.
    ownersFile: String,             
    /// Full pathname of the car types file.
    carTypesFile: String,           
    /// Full pathname of the cars file. 
    carsFile: String,               
    /// Full pathname of the stats file.
    statsFile: String,              
    /// Division map. 
    divisions: HashMap<u8, Division>, 
    /// Max Division
    maxDivision: u8,
    /// Station map.
    stations: HashMap<u8, Station>, 
    /// Max station
    maxStation: u8,
    /// Train map.
    trains: Arc<HashMap<usize, Train>>,
    /// Max train number
    maxTrain: usize,
    /// Train name map.
    trainIndex: HashMap<String, usize>, 
    /// Industries map.
    industries: Arc<HashMap<usize, IndustryFile>>, 
    /// Max industry.
    maxIndustry: usize,
    /// Car type order vector.
    carTypesOrder: Vec<char>,       
    /// Car type map.
    carTypes: HashMap<char, CarType>, 
    /// Car group vector.
    carGroups: Vec<CarGroup>,       
    /// Car owner map.
    owners: HashMap<String, Owner>, 
    /// Car vector. 
    cars: Vec<Car>,                 
    /// Switch lists.
    switchList: SwitchList,         
    /// Current session number. 
    sessionNumber: u32,             
    /// Current shift number.
    shiftNumber: u8,                
    /// The total number of shifts.
    totalShifts: u32,               
    /// The ran all trains flag.
    ranAllTrains: u32,              
    /// The total number of pickups. 
    totalPickups: u32,              
    /// The total number of loads. 
    totalLoads: u32,                
    /// The total number of tons.
    totalTons: u32,                 
    /// The total number of revenue tons. 
    totalRevenueTons: u32,          
    /// Train print flag. 
    trainPrintOK: bool,             
    ///  Way freight flag.
    wayFreight: bool,               
    /// Deliver flag.
    deliver: bool,                  
    /// Train length.
    trainLength: u32,               
    /// The number of cars on a train.
    numberCars: u32,                
    /// The number of tons on a train.
    trainTons: u32,                 
    /// The number of loads on a train.
    trainLoads: u32,                
    /// The number of empties on a train.
    trainEmpties: u32,              
    /// The longest a train has been.
    trainLongest: u32,              
    /// Current division. 
    curDivIndex: u8,                
    /// Origin Yard. 
    originYardIndex: usize,         
    /// A trains last location.
    trainLastLocationIndex: usize,  
    /// A temporary for a car's location.
    carDestIndex: usize,            
    /// The current stats period.
    statsPeriod: u32,               
    /// The number of cars moved. 
    carsMoved: u32,                 
    /// The number of cars at their destinations. 
    carsAtDest: u32,                
    /// The number of cars not moved. 
    carsNotMoved: u32,              
    /// The number of cars moved one time.
    carsMovedOnce: u32,             
    /// The number of cars moved two times.
    carsMovedTwice: u32,            
    /// The number of cars moved three times.
    carsMovedThree: u32,            
    /// The number of cars moved more then three times.
    carsMovedMore: u32,             
    /// The number of cars movements.
    carMovements: u32,              
    /// The number of cars in transit.
    carsInTransit: u32,             
    /// The number of cars at the workbench.
    carsAtWorkBench: u32,           
    ///  The number of cars at their destinations and still in transit.
    carsAtDest_carsInTransit: u32,  
    /// Flag for printing yard switch lists.
    printYards: bool,               
    /// Flag for printing alphabetical lists.
    printAlpha: bool,               
    /// Flag for printing a second copy of alphabetical lists.
    printAtwice: bool,              
    /// Flag for printing train switch lists.
    printList: bool,                
    /// Flag for printing a second copy of train switch lists.
    printLtwice: bool,              
    /// Flag for printing a dispatcher's report.
    printDispatch: bool,            
    /// Flag for printing train movements.
    printem: bool,                  
    //messageBuffer: String,
    //whitespace: String,
    //indScrapYard: &Industry::Industry,
}

use std::fmt;
impl fmt::Display for System {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "<#System {}>", self.systemName)
  }
}

///  Types of car type reports.

pub enum CarTypeReport {
    /// Report on all car types.
    All,    
    /// Report on one type. 
    Type,
    /// Report summary.
    Summary,
}

///  Types of location report.
pub enum CarLocationType {
    /// Report by industry. 
    INDUSTRY,
    ///  Report by station.
    STATION,
    /// Report by division.
    DIVISION,
    /// Report on all locations. 
    ALL,
}

impl System {
    /// Return the system name.  This is read from the system file.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the system name.
    pub fn SystemName(&self) -> String {
        self.systemName.clone()
    }
    /// Return the system file's full path name.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the system file name.
    pub fn SystemFile(&self) -> String {
        self.systemFile.clone()
    }
    /// Return the industry file's full path name.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the industry file name.
    pub fn IndustriesFile(&self) -> String {
        self.industriesFile.clone()
    }
    /// Return the trains file's full path name.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the trains file name.
    pub fn TrainsFile(&self) -> String {
        self.trainsFile.clone()
    }
    /// Return the train orders file's full path name.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the train orders  file name.
    pub fn OrdersFile(&self) -> String {
        self.ordersFile.clone()
    }
    /// Return the owners file's full path name.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the owners file name.
    pub fn OwnersFile(&self) -> String {
        self.ownersFile.clone()
    }
    /// Return the car types file's full path name.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the car types file name.
    pub fn CarTypesFile(&self) -> String {
        self.carTypesFile.clone()
    }
    /// Return the cars file's full path name.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the cars file name.
    pub fn CarsFile(&self) -> String {
        self.carsFile.clone()
    }
    /// Return the statistics file's full path name.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the statistics file name.
    pub fn StatsFile(&self) -> String {
        self.statsFile.clone()
    }
    /// Return the number of divisions loaded.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the number of divisions.
    pub fn NumberOfDivisions(&self) -> usize {
        self.divisions.len()
    }
    /// Find a division by its index.  Returns either a reference to the 
    /// division or None.
    ///
    /// ## Parameters:
    /// - i The division index to look for.
    ///
    /// __Returns__ A reference to the division struct or None.
    pub fn DivisionByIndex(&self, i: u8) -> Option<&Division> {
        self.divisions.get(&i)
    }
    /// Find a division by its symbol. Returns either a reference to the
    /// to the division or None.
    ///
    /// ## Parameters:
    /// - symbol The division symbol to look for.
    ///
    /// __Returns__ A reference to the division struct or None.
    pub fn FindDivisionBySymbol(&self, symbol: char) -> Option<&Division> {
        for (id, div) in self.divisions.iter() {
            if div.Symbol() == symbol {
                return Some(div);
            }
        }
        None
    }
    /// Find a division index by its symbol. Returns either a reference to the
    /// to the division or None.
    ///
    /// ## Parameters:
    /// - symbol The division symbol to look for.
    ///
    /// __Returns__ A division index or None.
    pub fn FindDivisionIndexBySymbol(&self, symbol: char) -> Option<u8> {
        for (id, div) in self.divisions.iter() {
            if div.Symbol() == symbol {
                return Some(*id);
            }
        }
        None
    }
    /// List train names
    ///
    /// ## Parameters:
    /// - all List all trains reguarless of shift or only list trains this shift.
    /// - trainType List only this type or all types is None.
    ///
    /// __Returns__ nothing
    pub fn ListTrainNames(&self, all: bool, trainType: Option<TrainType>) {
        let mut TrainCount = 0;
        println!("{}",self.SystemName());
        //eprintln!("*** in System::ListTrainNames(): self.shiftNumber is {}",self.shiftNumber);
        for train in self.trains.values() {
            let trainName = train.Name();
            let trainShift = train.Shift();
            //eprintln!("*** in System::ListTrainNames(): trainName is {}, trainShift is {}, train.Type() is {}",trainName,trainShift,train.Type());
            if !all && trainShift != self.shiftNumber {continue;}
            if trainType.is_some() &&
               trainType.unwrap() != train.Type() {continue;}
            if train.Type() == TrainType::BoxMove {continue;}
            TrainCount += 1;
            let mut buffer = trainName.clone();
            while buffer.len() < 11 {buffer.push(' ');}
            buffer += format!("<{}>", trainShift).as_str();
            while buffer.len() < 20 {buffer.push(' ');}
            print!("{}",buffer);
            if (TrainCount % 4) == 0 {println!("");}
        }
        if (TrainCount % 4) != 0 {println!("");}
        println!("\nTotal Trains: {}\n", TrainCount);
    }
    /// Return an iterator into the divisions.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ An interator into the divisions.
    pub fn DivisionIter(&self) ->  Iter<'_, u8, Division> {
        self.divisions.iter()
    }
    /// Return the number of stations.
    ///
    /// ## Parameters:
    /// None.
    /// 
    /// __Returns__ the number of stations.
    pub fn NumberOfStations(&self) -> usize {
        self.stations.len()
    }
    /// Return a station by index.
    ///
    /// ## Parameters:
    /// - i the station index.
    ///
    /// __Returns__ a reference to a station or None.
    pub fn StationByIndex(&self, i: u8) -> Option<&Station> {
        self.stations.get(&i)
    }
    /// Return a station by name.
    ///
    /// ## Parameters:
    /// - name the name of the station.
    ///
    /// __Returns__ a reference to a station or None.
    pub fn FindStationByName(&self, name: String) -> Option<&Station> {
        for (id, sta) in self.stations.iter() {
            if sta.Name() == name {
                return Some(sta);
            }
        }
        None
    }
    /// Return an iterator into the stations.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ An interator into the stations.
    pub fn StationIter(&self) ->  Iter<'_, u8, Station> {
        self.stations.iter()
    }
    /// Return the number of trains.
    ///
    /// ## Parameters:
    /// None.
    /// 
    /// __Returns__ the number of trains.
    pub fn NumberOfTrains(&self) -> usize {
        self.trains.len()
    }
    /// Return a train by index.
    ///
    /// ## Parameters:
    /// - i the train index.
    ///
    /// __Returns__ a reference to a train or None.
    pub fn TrainByIndex(&self, i: usize) -> Option<&Train> {
        self.trains.get(&i)
    }
    /// Return a train by name.
    ///
    /// ## Parameters:
    /// - name the name of the train.
    ///
    /// __Returns__ a reference to a train or None.
    pub fn TrainByName(&self, name: String) -> Option<&Train> {
        let result = self.trainIndex.get(&name);
        if result == None {
            return None;
        }
        self.trains.get(result.unwrap())
    }
    /// Return an iterator into the trains.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ An interator into the trains.
    pub fn TrainIter(&self) ->  Iter<'_, usize, Train> {
        self.trains.iter()
    }
    /// Return the number of industries.
    ///
    /// ## Parameters:
    /// None.
    /// 
    /// __Returns__ the number of industries.
    pub fn NumberOfIndustries(&self) -> usize {
        self.industries.len()
    }
    
    /// Return an industry by index.
    ///
    /// ## Parameters:
    /// - i the industry index.
    ///
    /// __Returns__ a reference to an industry or None.
    pub fn IndustryByIndex(&self, i: usize) -> Option<&IndustryFile> {
        self.industries.get(&i)
    }
    /// Return an industry by name.
    ///
    /// ## Parameters:
    /// - name the industry name.
    ///
    /// __Returns__ a  reference to an industry or None.
    pub fn FindIndustryByName(&self, name: String) -> Option<&IndustryFile> {
        for (id, ind) in self.industries.iter() {
            if ind.Name() == name {
                return Some(ind);
            }
        }
        None
    }
    /// Return an iterator into the industries.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ An interator into the industries.
    pub fn IndustryIter(&self) ->  Iter<'_, usize, IndustryFile> {
        self.industries.iter()
    }
    /// Return the number of cars.
    ///
    /// ## Parameters:
    /// None.
    /// 
    /// __Returns__ the number of cars.
    pub fn NumberOfCars(&self) -> usize {
        self.cars.len()
    }
    /// Return the session number.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the session number.
    pub fn SessionNumber(&self) -> u32 {
        self.sessionNumber
    }
    /// Return the shift number.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the shift number.
    pub fn ShiftNumber(&self) -> u8 {
        self.shiftNumber
    }
    /// Return the total shifts.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the total shifts.
    pub fn TotalShifts(&self) -> u32 {
        self.totalShifts
    }
    /// Advance the shift number.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the next shift number.
    pub fn NextShift(&mut self) -> u8 {
        self.shiftNumber = self.shiftNumber + 1;
        self.totalShifts = self.totalShifts + 1;
        if self.shiftNumber > 3 {
            self.sessionNumber = self.sessionNumber + 1;
            self.shiftNumber = 1;
        }
        self.shiftNumber
    }
    /// Return the total number of cars.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the total number of cars.
    pub fn TotalCars(&self) -> usize {
        self.cars.len()
    }
    /// Return the statistics period.
    ///
    /// ## Parameters:
    /// None. 
    ///
    /// __Returns__ the statistics period.
    pub fn StatsPeriod(&self) -> u32 {
        self.statsPeriod
    }
    /// Return the number of cars moved. 
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the number of cars moved.
    pub fn CarsMoved(&self) -> u32 {
        self.carsMoved
    }
    /// Return the number of cars at their destinations. 
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the number of cars at their destinations.
    pub fn CarsAtDest(&self) -> u32 {
        self.carsAtDest
    }
    /// Return the number of cars not moved. 
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the number of cars not moved.
    pub fn CarsNotMoved(&self) -> u32 {
        self.carsNotMoved
    }
    /// Return the number of cars moved once. 
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the number of cars moved once.
    pub fn CarsMovedOnce(&self) -> u32 {
        self.carsMovedOnce
    }
    /// Return the number of cars moved twice. 
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the number of cars moved twice.
    pub fn CarsMovedTwice(&self) -> u32 {
        self.carsMovedTwice
    }
    /// Return the number of cars moved three times. 
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the number of cars moved three times.
    pub fn CarsMovedThree(&self) -> u32 {
        self.carsMovedThree
    }
    /// Return the number of cars moved more than three times. 
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the number of cars moved more than three times.
    pub fn CarsMovedMore(&self) -> u32 {
        self.carsMovedMore
    }
    /// Return the number of movements. 
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the number of movements.
    pub fn CarMovements(&self) -> u32 {
        self.carMovements
    }
    /// Return the number of cars in transit. 
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the number of cars in transit.
    pub fn CarsInTransit(&self) -> u32 {
        self.carsInTransit
    }
    /// Return the number of cars on the workbench. 
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the number of cars on the workbench.
    pub fn CarsAtWorkBench(&self) -> u32 {
        self.carsAtWorkBench
    }
    /// Return the number of cars in transit at at their destination. 
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the number of cars at at their destination.
    pub fn CarsAtDest_CarsInTransit(&self) -> u32 {
        self.carsAtDest_carsInTransit
    }
    /// Return the print yards flag.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the print yards flag. 
    pub fn PrintYards(&self) -> bool {
        self.printYards
    }
    /// Set the print yards flag.
    ///
    /// ## Parameters:
    /// - flag the new flag value.
    ///
    /// __Returns__ nothing.
    pub fn SetPrintYards(&mut self,flag: bool) {
        self.printYards = flag;
    }
    /// Return the print alpha flag.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the print alpha flag. 
    pub fn PrintAlpha(&self) -> bool {
        self.printAlpha
    }
    /// Set the print alpha flag.
    ///
    /// ## Parameters:
    /// - flag the new flag value.
    ///
    /// __Returns__ nothing.
    pub fn SetPrintAlpha(&mut self,flag: bool) {
        self.printAlpha = flag;
    }
    /// Return the print alpha twice flag.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the print alpha twice flag. 
    pub fn PrintAtwice(&self) -> bool {
        self.printAtwice
    }
    /// Set the print alpha twice flag.
    ///
    /// ## Parameters:
    /// - flag the new flag value.
    ///
    /// __Returns__ nothing.
    pub fn SetPrintAtwice(&mut self,flag: bool) {
        self.printAtwice = flag;
    }
    /// Return the print list flag.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the print list flag. 
    pub fn PrintList(&self) -> bool {
        self.printList
    }
    /// Set the print list flag.
    ///
    /// ## Parameters:
    /// - flag the new flag value.
    ///
    /// __Returns__ nothing.
    pub fn SetPrintList(&mut self,flag: bool) {
        self.printList = flag;
    }
    /// Return the print list twice flag.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the print list twice flag. 
    pub fn PrintLtwice(&self) -> bool {
        self.printLtwice
    }
    /// Set the print list twice flag.
    ///
    /// ## Parameters:
    /// - flag the new flag value.
    ///
    /// __Returns__ nothing.
    pub fn SetPrintLtwice(&mut self,flag: bool) {
        self.printLtwice = flag;
    }
    /// Return the print dispatcher flag.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the print dispatcher flag. 
    pub fn PrintDispatch(&self) -> bool {
        self.printDispatch
    }
    /// Set the print dispatcher flag.
    ///
    /// ## Parameters:
    /// - flag the new flag value.
    ///
    /// __Returns__ nothing.
    pub fn SetPrintDispatch(&mut self,flag: bool) {
        self.printDispatch = flag;
    }
    /// Return the printem flag.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the printem flag. 
    pub fn Printem(&self) -> bool {
        self.printem
    }
    /// Set the printem flag.
    ///
    /// ## Parameters:
    /// - flag the new flag value.
    ///
    /// __Returns__ nothing.
    pub fn SetPrintem(&mut self,flag: bool) {
        self.printem = flag;
    }
    /// Utility to get a line after skipping any intervening comments.
    ///
    /// Read the next line from a datafile, skipping any intervening comment 
    /// lines. Save the line of text and return the line if successfull.  If 
    /// EOF reached, return an error.
    /// ## Parameters:
    /// - reader The input buffer to read from.
    ///
    /// __Returns__ the read line or an Err    
    fn SkipCommentsReadLine(reader: &mut BufReader<File>) -> 
            std::io::Result<String> {
        let mut buffer = String::new();
        loop {
            buffer.clear();
            let result = reader.read_line(&mut buffer);
            //println!("In SkipCommentsReadLine(): result is {:?}",result);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            if buffer.len() == 0 && result.unwrap() == 0 {
                return Err(Error::new(ErrorKind::Other,"EOF"));
            }
            buffer = buffer.trim().to_string();
            //println!("In SkipCommentsReadLine: buffer is {}",buffer);
            if buffer.len() > 0 && !buffer.starts_with("'") {
                break;
            }
        }
        //println!("Returning from SkipCommentsReadLine");
        Ok(buffer)
    }
    /// Read in the division map.
    ///
    ///    Basically, a division has a numeric identifier, a symbolic name, and
    ///    a "home" -- which can be a YARD or an INDUSTRY.
    ///
    ///    The purpose of a division is that cars destined for industries are
    ///    routed --> to the industry's station --> to the station's division
    ///    --> to the division's home. It's just a way of clumping industries
    ///    together into a logical unit.
    ///  
    ///    - \#          Numeric identifier
    ///    - Symbol     Symbolic alphanumeric identifier (A-Z a-z 0-9)
    ///    - Home       Numeric Home yard of the division
    ///    - Area       Symbolic alphanumeric Area identifier
    ///    - Name       Text name of the division
    /// ## Parameters:
    /// - reader The input buffer to read from.
    ///
    /// __Returns__ the number of divisions read or an Err
    fn ReadDivisions(&mut self, reader: &mut BufReader<File>) ->
            std::io::Result<u8> {
        let mut count: u8 = 0;
        let buffer = Self::SkipCommentsReadLine(reader)
                    .expect("Read Error");
        let temp = buffer.split_once("=").unwrap();
        // The file starts with a mac count of divisions.  Originally used to
        // dimension the QBASIC arrays, but with C++ and rust that is not 
        // needed except as a termination check.
        if temp.0.trim() != "Divisions" {
            return Err(Error::new(ErrorKind::Other,"Missing Divisions = "));
        }
        let divcount: u8 = temp.1.trim()
                .parse::<u8>()
                .expect("Syntax error");
        self.maxDivision = divcount;
        //println!("divcount is {}",divcount);
        loop {
            let line = Self::SkipCommentsReadLine(reader)
                    .expect("Read Error");
            // A -1 on a line by itself terminates the Division data,
            if line == "-1" || count >= divcount {
                break;
            }
            // Each line is a comma separated list of 5 values
            let items: Vec<_> = line.split(",").collect();
            //println!("items is {:?}",items);
            let div = Division::new(items[4].to_string(),
                                    items[2].trim().parse::<usize>()
                                        .expect("Syntax error"),
                                    items[1].chars().next().unwrap(),
                                    items[3].chars().next().unwrap());
             let div_index = items[0].trim().parse::<u8>()
                    .expect("Syntax error");
             self.divisions.insert(div_index,div);
             count = count + 1;
        }
        Ok(count)
    }
    /// Read in the station map.
    ///
    /// Allocate memory for stations, and read in definitions
    ///
    ///    Basically, a station has a symbolic name, and is based in a "division".
    ///    This means that freight cars destined for an industry at this station
    ///    are usually routed to the "division yard" (see below) first. Then the
    ///    wayfreight (or boxmove) takes the car from the yard to the station and
    ///    then to the industry.
    ///
    ///    Note you are free to create several "stations" with the same name, and
    ///    yet with different "divisions". The purpose of this flexibility is to
    ///    allow you to serve industries on your layout in a flexible manner - so
    ///    the same physical "layout station" may be represented by several of the
    ///    "logical stations" in the database.
    ///
    ///    Another trick is to define "trailing point" sidings in one direction as
    ///    one station, and then trailing point sidings in the opposite direction
    ///    as another station (with the same name, I mean). Then an "out and back"
    ///    wayfreight can then be set up to serve only trailing point sidings, as
    ///    it travels out, turns, and returns thru the same area.
    /// ## Parameters:
    /// - reader The input buffer to read from.
    ///
    /// __Returns__ the number of stations read or an Err
    fn ReadStations(&mut self,reader: &mut BufReader<File>) ->
            std::io::Result<u8> {
        let mut count: u8 = 0;
        let buffer = Self::SkipCommentsReadLine(reader)
                    .expect("Read Error");
        let temp  = buffer.split_once("=").unwrap();
        if temp.0.trim() != "Stations" {
             return Err(Error::new(ErrorKind::Other,"Missing Stations = "));
        }
        let stacount: u8 = temp.1.trim()
                .parse::<u8>()
                .expect("Syntax error");
        self.maxStation = stacount;
        loop {
            let line = Self::SkipCommentsReadLine(reader)
                    .expect("Read Error");
            if line == "-1" || count >= stacount {
                break;
            }
            let items: Vec<_> = line.split(",").collect();
            let sta = Station::new(items[1].to_string(),
                                   items[3].to_string(),
                                   items[2].trim().parse::<u8>()
                                            .expect("Syntax error"));
            let sta_index = items[0].trim().parse::<u8>()
                        .expect("Syntax error");
            self.stations.insert(sta_index,sta);
            count = count + 1;
        }
        Ok(count)
    }
    /// Read in the industries file.
    ///
    /// Allocate memory for industries, and read in definitions
    ///
    /// -  IndsType        type of location
    ///
    ///     -                     "Y"   Yard
    ///     -                     "S"   Stage
    ///     -                     "I"   Industry Online
    ///     -                     "O"   Industry Offline
    ///
    /// -  IndsStation     station location of this yard or industry
    /// -  IndsName        symbolic name (may be duplicated)
    /// -  IndsTrackLen    physical track space available
    ///
    /// -  IndsAssignLen   assignable length -- the combined length of all the cars
    ///                     destinated for an industry at one time - often larger
    ///                     than TrackLen
    ///
    /// -  IndsPriority    priority of car assignment to this industry -- 1 is the
    ///                     highest priority, while MaxPriority is the lowest --
    ///                     this assures car supply to more important customers
    ///
    /// -  IndsReload      "Y" means cars delivered as loads, may leave as loads --
    ///                     provided the industry accepts the car type as empty
    ///
    /// -  IndsMirror      the identity of the industry that "mirrors" this one --
    ///                     a car delivered to this industry will be "relocated"
    ///                     immediately to the "mirror" location
    ///
    ///                     Typical mirrors: power plant --> coal mine (loads)
    ///                                      coal mine --> power plant (empties)
    ///
    /// -  IndsPlate       maximum clearance plate of cars for this industry
    /// -  IndsClass       maximum weight class of cars for this industry
    /// -  IndsDivList     where this industry will ship its loads
    /// -  IndsCarLen      maximum car length of cars for this industry
    /// -  IndsLoadTypes   what CarTypes are accepted as loads
    /// -  IndsEmptyTypes  what CarTypes are accepted as empties
    /// ## Parameters:
    /// - filename the file to read the industries from.
    ///
    /// __Returns__ the number of industries read or an Err
    fn ReadIndustries(&mut self,filename: &PathBuf) ->
            std::io::Result<usize> {
        let f = File::open(filename.to_str().unwrap())
                .expect("Cannot open industries file");
        let mut reader = BufReader::new(f);
        let mut count: usize = 0;
        let buffer = Self::SkipCommentsReadLine(&mut reader)
                    .expect("Read Error");
        let temp  = buffer.split_once("=").unwrap();
        if temp.0.trim() != "Industries" {
            return Err(Error::new(ErrorKind::Other,"Missing Industries = "));
        }
        let indcount: usize = temp.1.trim()
                .parse::<usize>()
                .expect("Syntax error");
        self.maxIndustry = indcount;
        let industries = Arc::make_mut(&mut self.industries);
        loop {
            let mut line = Self::SkipCommentsReadLine(&mut reader)
                    .expect("Read Error");
            if line == "-1" || count >= indcount {
                break;
            }
            let mut commacount = line.matches(",").count();
            while commacount < 15 {
                line.push(',');
                line.push_str(Self::SkipCommentsReadLine(&mut reader)
                                .expect("Read Error").as_str());
                commacount = line.matches(",").count();
            }
            let items: Vec<_> = line.split(",").collect();
            //println!("In ReadIndustries(): items is {:?}",items);
            let Ix = items[0].trim().parse::<usize>().expect("Syntax Error");
            if industries.contains_key(&Ix) {
                return Err(Error::new(ErrorKind::Other,"Duplicate industry index"));
            }
            let mut tword  = String::from(items[1].trim());
            tword.make_ascii_uppercase();
            let IndsType = tword.chars().next().unwrap();
            let IndsStation = items[2].trim().parse::<u8>().expect("Syntax Error");
            if IndsStation == 0 {
                continue
            } else if IndsStation > 1 {
                if !self.stations.contains_key(&IndsStation) {
                    return Err(Error::new(ErrorKind::Other,"Undefined station index"));
                }
            }
            let IndsName = String::from(items[3].trim());
            let IndsTrackLen = items[4].trim().parse::<u32>().expect("Syntax Error");
            let IndsAssignLen = items[5].trim().parse::<u32>().expect("Syntax Error");
            let IndsPriority = items[6].trim().parse::<u8>().expect("Syntax Error");
            let yesno = items[7].trim().chars().next().unwrap();
            let IndsReload = if yesno == 'Y' || yesno == 'y' {
                                true
                             } else if yesno == 'N' || yesno == 'n' {
                                false
                             } else {
                                return Err(Error::new(ErrorKind::Other,"Undefined boolean"));
                             };
            let hword = items[8].trim();
            let IndsHazard = if hword.len() > 0 {hword.chars().next().unwrap()} else {' '};
            let IndsMirror = items[9].trim().parse::<usize>().expect("Syntax Error");
            let IndsPlate = items[10].trim().parse::<u8>().expect("Syntax Error");
            let IndsClass = items[11].trim().parse::<u8>().expect("Syntax Error");
            let IndsDivList = String::from(items[12].trim());
            let IndsCarLen = items[13].trim().parse::<u32>().expect("Syntax Error");
            let IndsLoadTypes = String::from(items[14].trim());
            let IndsEmptyTypes = String::from(items[15].trim());
            industries.insert(Ix, IndustryFile::new(IndsStation, IndsMirror, 
                                         IndsName,
                                         IndsLoadTypes, IndsEmptyTypes, 
                                         IndsDivList, IndsTrackLen, 
                                         IndsAssignLen, IndsPriority, 
                                         IndsPlate, IndsClass, IndsCarLen,
                                         IndsReload, IndsType, IndsHazard));

            count = count + 1;
        }
        Ok(count)
    }
    /// Strip quotation marks from a string.
    ///
    /// ## Parameters:
    /// - s the string to strip quotes from.
    ///
    /// __Returns__ the string without the quotes.
    fn StripQuotes(s: &str) -> String {
        if s.chars().next() == Some('"') {
            let l = s.len() - 1;
            let s1 = &s[1..l];
            String::from(s1)
        } else {
            String::from(s)
        }
    }
    /// Read in the trains file.
    ///
    /// Allocate memory for trains, and read in definitions
    ///
    /// -   TrnType        "M"anifest "W"ayfreight "P"assenger "B"oxmove
    /// -   TrnShift       shift number 1 or 2 or 3
    /// -   TrnDone        "N" means cars "Car Done" is not set by move in train
    /// -   TrnName        symbolic name of the train
    /// -   TrnMxCars      maximum number of cars in the train at once
    /// -   TrnDivList     "forwarding list" of divisions (MANIFESTS)
    /// -   TrnStops       stops (industries, or stations)
    /// -   TrnOnDuty      scheduled time of start
    /// -   TrnPrint()     "P" means print the train order, else not
    /// -   TrnMxClear     maximum clearance plate of cars in this train
    /// -   TrnMxWeigh     maximum weight class of cars in this train
    /// -   TrnCarTypes    which car types are allowed in the train
    /// -   TrnMxLen       maximum length of the train in feet
    /// -   TrnDesc        one line text description for train orders printout
    /// ## Parameters:
    /// - filename the file to read the trains from.
    ///
    /// __Returns__ the number of trains read or an Err
    fn ReadTrains(&mut self,filename: &PathBuf) -> std::io::Result<usize> {
        let f = File::open(filename.to_str().unwrap())
                .expect("Cannot open trains file");
        let mut reader = BufReader::new(f);
        let mut count: usize = 0;
        let buffer = Self::SkipCommentsReadLine(&mut reader)
                    .expect("Read Error");
        let temp  = buffer.split_once("=").unwrap();
        if temp.0.trim() != "Trains" {
            return Err(Error::new(ErrorKind::Other,"Missing Trains = "));
        }
        let traincount: usize = temp.1.trim()
                 .parse::<usize>()
                 .expect("Syntax error");
        self.maxTrain = traincount;
        let trains = Arc::make_mut(&mut self.trains);
        loop {
            let mut line = Self::SkipCommentsReadLine(&mut reader)
                    .expect("Read Error");
            if line == "-1" || count >= traincount {
                break;
            }
            let mut commacount = line.matches(",").count();
            while commacount < 15 {
                line.push(',');
                line.push_str(Self::SkipCommentsReadLine(&mut reader)
                                .expect("Read Error").as_str());
                commacount = line.matches(",").count();
            }
            let items: Vec<_> = line.split(",").collect();
            //println!("In ReadTrains(): items is {:?}",items); 
            let Tx = items[0].trim().parse::<usize>().expect("Syntax Error");
            if trains.contains_key(&Tx) {
                return Err(Error::new(ErrorKind::Other,"Duplicate train index"))
            }
            let mut tword  = String::from(items[1].trim());
            tword.make_ascii_uppercase();
            let TrnType = TrainType::new(tword.chars().next().unwrap());
            let TrnShift = items[2].trim().parse::<u8>().expect("Syntax error");
            //println!("In ReadTrains(): train {} is a {}",Tx, TrnType);
            let yesno = items[3].trim().chars().next().unwrap();
            //println!("In ReadTrains(): yesno (TrnDone) is {}",yesno);
            let  TrnDone = if yesno == 'Y' || yesno == 'y' {
                              true
                           } else if yesno == 'N' || yesno == 'n' {
                              false
                           } else {
                              return Err(Error::new(ErrorKind::Other,"Undefined boolean"));
                           };
            let TrnName = String::from(items[4].trim()); 
            let TrnMxCars = items[5].trim().parse::<u32>().expect("Syntax error"); 
            let TrnDivList = String::from(items[6].trim());
            let onDuty = String::from(items[9].trim());
            let TrnOnDutyH = onDuty[0..2].parse::<u32>().expect("Syntax error");
            let TrnOnDutyM = onDuty[2..4].parse::<u32>().expect("Syntax error");
            let TrnOnDuty = 60 * TrnOnDutyH + TrnOnDutyM;
            let yesno = items[10].trim().chars().next().unwrap();
            //println!("In ReadTrains(): yesno (TrnPrint) is {}",yesno);
            let TrnPrint = if yesno == 'P' || yesno == 'p' {
                              true
                           } else if yesno == 'N' || yesno == 'n' {
                              false
                           } else {
                              return Err(Error::new(ErrorKind::Other,"Undefined boolean"));
                           };
            let TrnMxClear: u8 = items[11].trim().parse::<u8>().expect("Parse Error");
            let TrnMxWeigh: u8 = items[12].trim().parse::<u8>().expect("Parse Error");
            let TrnCarTypes = String::from(items[13].trim());
            let TrnMxLen = items[14].trim().parse::<u32>().expect("Parse Error");
            let TrnDescr = Self::StripQuotes(items[15].trim());
            //println!("In ReadTrains(): TrnDescr is '{}'", TrnDescr);
            let mut train = Train::new(TrnName.clone(), TrnDivList, TrnCarTypes,
                                         TrnDescr, TrnShift, TrnMxCars, 
                                         TrnMxClear, TrnMxWeigh, TrnMxLen,
                                         TrnOnDuty, TrnPrint, TrnDone, 
                                         TrnType);
            for stop in items[7].trim().split_ascii_whitespace() {
                //println!("In ReadTrains(): stop is {}",stop);
                let stopnumber = stop.parse::<usize>().expect("Syntax error");
                if stopnumber == 0 {
                    break;
                }
                if train.Type() == TrainType::Manifest {
                    train.AddStop(&Stop::newIndustryStop(stopnumber));
                } else {
                    train.AddStop(&Stop::newStationStop(stopnumber as u8));
                }
            }
            trains.insert(Tx, train);
            self.trainIndex.insert(TrnName.clone(), Tx);
            count = count + 1;
        }
        Ok(count)
    }
    /// Read in the train orders file.
    ///
    /// ## Parameters:
    /// - filename the file to read the train orders from.
    ///
    /// __Returns__ the number of train orders read or an Err
    fn ReadTrainOrders(&mut self,filename: &PathBuf) -> 
            std::io::Result<usize> {
        let f = File::open(filename.to_str().unwrap())
                .expect("Cannot open orders file");
        let mut reader = BufReader::new(f);
        let mut count: usize = 0;
        let trains = Arc::make_mut(&mut self.trains);
        loop {
            let result = Self::SkipCommentsReadLine(&mut reader);
            //println!("In ReadTrainOrders(): result is {:?}", result);
            if result.is_err() {
                break;
            }
            let buffer = result.unwrap();
            
            let items: Vec<_> = buffer.split(",").collect();
            if items.len() < 2 {
                return Err(Error::new(ErrorKind::Other,"Syntax error"));
            }
            let trainname = String::from(items[0].trim());
            let trainorder = Self::StripQuotes(items[1].trim());
            //println!("In ReadTrainOrders(): trainname is {}, trainorder is {}",
            //            trainname,trainorder);
            let zero: &usize = &0;
            let tx: &usize = self.trainIndex.get(&trainname).unwrap_or(&0);
            if tx != zero {
                trains.get_mut(&tx)
                    .expect("Unknown train")
                    .AddOrder(trainorder.clone());
            }
            count = count + 1;
        }
        Ok(count)
    }
    /// Read in the car types file.
    ///
    /// ## Parameters:
    /// - filename the file to read the car types from.
    ///
    /// __Returns__ () or an Err
    fn ReadCarTypes(&mut self,filename: &PathBuf) ->  std::io::Result<()> {
        let f = File::open(filename.to_str().unwrap())
                .expect("Cannot open cartypes file");
        let mut reader = BufReader::new(f);
        for car_type_count in 0..NUMBER_OF_CARTYPES {
            let line = Self::SkipCommentsReadLine(&mut reader)
                    .expect("Read Error");
            let items: Vec<_> = line.split(",").collect();
            if items.len() < 5 {
                return Err(Error::new(ErrorKind::Other,"Syntax error"));
            }
            //println!("In ReadCarTypes(): items is {:?}", items);
            let symbol: char = items[0].trim().chars().next().unwrap_or(' ');
            let group: char = items[1].trim().chars().next().unwrap_or(' ');
            let type_name: String = String::from(items[2].trim());
            //let pad = items[3];
            let comment: String = String::from(items[4].trim());
            self.carTypesOrder.push(symbol);
            self.carTypes.insert(symbol, CarType::new(comment,type_name,group));
        }
        for car_type_count in 0..MAX_CAR_GROUP {
            let result = Self::SkipCommentsReadLine(&mut reader);
            if result.is_err() {
                break;
            }
            let line = result.unwrap();
            let items: Vec<_> = line.split(",").collect();
            if items.len() < 2 {
                return Err(Error::new(ErrorKind::Other,"Syntax error"));
            }
            let symbol: char = items[0].trim().chars().next().unwrap();
            let comment: String = String::from(items[1].trim());
            //let pad = items[2];
            self.carGroups.push(CarGroup::new(symbol,comment));
        }
        Ok(())
    }
    /// Read in the owners file.
    ///
    /// ## Parameters:
    /// - filename the file to read the owners from.
    ///
    /// __Returns__ the number of owners read or an Err
    fn ReadOwners(&mut self,filename: &PathBuf) ->  
        std::io::Result<usize> {
        let mut count = 0;
        let f = File::open(filename.to_str().unwrap())
                .expect("Cannot open owners file");
        let mut reader = BufReader::new(f);
        let line = Self::SkipCommentsReadLine(&mut reader)
                    .expect("Read Error");
        let TotalOwners = line.trim().parse::<usize>().expect("Syntax error");
        for ox in 0..TotalOwners {
            let result = Self::SkipCommentsReadLine(&mut reader);
            if result.is_err() {
                break;
            }
            let line = result.unwrap();
            let items: Vec<_> = line.split(",").collect();
            if items.len() < 3 {
                return Err(Error::new(ErrorKind::Other,"Syntax error"));
            }
            let initials = items[0].trim();
            let name = items[1].trim();
            let comment = items[2].trim();
            self.owners.insert(String::from(initials), 
                    Owner::new(String::from(initials),
                               String::from(name),
                               String::from(comment)));
            count = count + 1;
        }
        Ok(count)
    }
    /// Delete all existing cars.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ nothing.
    fn DeleteAllExistingCars(&mut self) {
        self.cars.clear();
    }
    /// Load car file
    ///
    /// Allocate memory for cars, and read in definitions
    ///
    /// -  CrsType        car type from TypesFile
    /// -  CrsRR          railroad reporting mark symbols or lessor/lessee string
    /// -  CrsNum         car number or car number/units -- a string not a number
    /// -  CrsDivList     division assignment list for empty -- or no restriction
    /// -  CrsLen         extreme car (or multi-car) length over couplers
    /// -  CrsPlate       clearance plate -- see PLATE.TXT file
    /// -  CrsClass       car weight class -- see WEIGHT.TXT file
    /// -  CrsLtWt        car light weight in tons
    /// -  CrsLdLmt       car load limit in tons
    /// -  CrsStatus      loaded or empty status is "L" or "E"
    /// -  CrsOkToMirror  Y means car may be mirrored
    /// -  CrsFixedRoute  Y means car can only be routed to home divisions
    /// -  CrsOwner       car owner's initials -- see OWNERS.TXT
    /// -  CrsDone        car is done moving -- receives TrnDone value
    /// -  CrsTrain       last train to move this car
    /// -  CrsMoves       number of times car was moved this session
    /// -  CrsLoc         car's current location
    /// -  CrsDest        car's destination
    /// -  CrsTrips       number of moves for this car
    /// -  CrsAssigns     number of assignments for this car
    ///
    /// -  CrsPeek        temporary look-ahead array for car handling
    /// -  CrsTmpStatus   status during assignment
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the number of cars read or Err.
    fn LoadCarFile(&mut self) -> std::io::Result<usize> {
        let mut count = 0;
        let f = File::open(&self.carsFile)
                .expect("Cannot open cars file");
        let mut reader = BufReader::new(f);
        let line = Self::SkipCommentsReadLine(&mut reader)
                        .expect("Read Error");
        self.sessionNumber = line.trim().parse::<u32>().expect("Syntax error");
        let line = Self::SkipCommentsReadLine(&mut reader)
                        .expect("Read Error");
        self.shiftNumber = line.trim().parse::<u8>().expect("Syntax error");
        let line = Self::SkipCommentsReadLine(&mut reader)
                        .expect("Read Error");
        let totalCars: usize = line.trim().parse::<usize>().expect("Syntax error");
        self.totalShifts = self.sessionNumber * 3;
        self.NextShift();
        self.sessionNumber = self.sessionNumber + (self.shiftNumber as u32);
        self.DeleteAllExistingCars();
        let mut Cx = 0;
        loop {
            let result = Self::SkipCommentsReadLine(&mut reader);
            if result.is_err() {
                break;
            }
            Cx += 1;
            let line = result.unwrap();
            let items: Vec<_> = line.split(',').collect();
            if items.len() < 20 {
                return Err(Error::new(ErrorKind::Other,"Syntax error"));
            }
            let CrsType: char = items[0].trim().chars().next().unwrap();
            let CrsRR = items[1].trim();
            let CrsNum = items[2].trim();
            let CrsDivList = items[3].trim(); 
            let CrsLen: u32 = items[4].trim().parse::<u32>().expect("Syntax error");
            let CrsPlate: u8 = items[5].trim().parse::<u8>().expect("Syntax error");
            let CrsClass: u8 = items[6].trim().parse::<u8>().expect("Syntax error");
            let CrsLtWt: u32 = items[7].trim().parse::<u32>().expect("Syntax error");
            let CrsLdLmt: u32 = items[8].trim().parse::<u32>().expect("Syntax error");
            let yesno: char = items[9].trim().chars().next().unwrap();
            let CrsStatus: bool = if yesno == 'L' || yesno == 'l' {
                                    true
                                  } else if yesno == 'E' || yesno == 'e' {
                                    false
                                  } else {
                                    return Err(Error::new(ErrorKind::Other,"Undefined load status"));
                                  };
            let yesno: char = items[10].trim().chars().next().unwrap();
            let CrsOkToMirror: bool = if yesno == 'Y' || yesno == 'y' {
                                        true
                                      } else if yesno == 'N' || yesno == 'n' {
                                        false
                                      } else {
                                    return Err(Error::new(ErrorKind::Other,"Undefined boolean"));
                                  };
            let yesno: char = items[11].trim().chars().next().unwrap();
            let CrsFixedRoute: bool = if yesno == 'Y' || yesno == 'y' {
                                        true
                                      } else if yesno == 'N' || yesno == 'n' {
                                        false
                                      } else {
                                    return Err(Error::new(ErrorKind::Other,"Undefined boolean"));
                                  };
            let CrsOwner = items[12].trim();
            let ownerCheck = self.owners.get(&String::from(CrsOwner));
            if ownerCheck.is_none() {
                self.owners.insert(String::from(CrsOwner),
                                   Owner::new(String::from(CrsOwner),
                                              String::from(CrsOwner),
                                              String::from("")));
            }
            let yesno: char = items[13].trim().chars().next().unwrap();
            let CrsDone: bool = if yesno == 'Y' || yesno == 'y' {
                                  true
                                } else if yesno == 'N' || yesno == 'n' {
                                   false
                                } else {
                                   return Err(Error::new(ErrorKind::Other,"Undefined boolean"));
                            };
            let CrsTrain: usize = items[14].trim().parse::<usize>().expect("Syntax error");
            let CrsMoves: u32 = items[15].trim().parse::<u32>().expect("Syntax error");
            let CrsLoc: usize = items[16].trim().parse::<usize>().expect("Syntax error");
            let CrsDest: usize = items[17].trim().parse::<usize>().expect("Syntax error");
            let CrsTrips: u32 = items[18].trim().parse::<u32>().expect("Syntax error");
            let CrsAssigns: u32 = items[19].trim().parse::<u32>().expect("Syntax error");
            self.cars.push(Car::new(CrsType,String::from(CrsRR),
                                    String::from(CrsNum),
                                    String::from(CrsDivList),CrsLen,CrsPlate,
                                    CrsClass,CrsLtWt,CrsLdLmt,CrsStatus,
                                    CrsOkToMirror,CrsFixedRoute,
                                    String::from(CrsOwner),CrsDone,CrsTrain,
                                    CrsMoves,CrsLoc,CrsDest,CrsTrips,
                                    CrsAssigns));
            count += 1;
        }
        Ok(count)
    }
    /// Load statistics file
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the number of statistics read or Err.
    fn LoadStatsFile(&mut self) -> std::io::Result<(usize,HashMap<usize, IndustryWorking>)> {
        let f = File::open(&self.statsFile)
                .expect("Cannot open stats file");
        let mut reader = BufReader::new(f);
        let mut line = String::new();
        let mut newformat: bool = false;
        let result = reader.read_line(&mut line);
        if result.is_err() {
            let e = result.err().unwrap();
            return Err(e);
        }
        let temp = line.find(',');
        if temp.is_some() {
            let pos = temp.unwrap();
            let temp = line.as_str();
            let word = &temp[0..pos];
            line = String::from(word);
            newformat = true;
        }
        self.statsPeriod = line.trim().parse::<u32>().expect("Syntax error");
        let mut Gx: usize = 0;
        let mut industries: HashMap<usize, IndustryWorking> = HashMap::new();
        loop {
            line.clear();
            Gx += 1;
            let result = reader.read_line(&mut line);
            if result.is_err() {break;}
            if result.unwrap() == 0 {break;} 
            let Ix: usize;
            let cn: u32;
            let cl: u32;
            let sl: u32;
            if newformat {
                let vlist: Vec<_> = line.split(',').collect();
                //println!("in LoadStats (newformat): vlist is {:?}", vlist);
                Ix = vlist[0].trim().parse::<usize>().expect("Syntax error");
                cn = vlist[1].trim().parse::<u32>().expect("Syntax error");
                cl = vlist[2].trim().parse::<u32>().expect("Syntax error");
                sl = vlist[3].trim().parse::<u32>().expect("Syntax error");
            } else {
                let line = line.as_str();
                let Ixword = &line[0..4].trim();
                Ix = Ixword.parse::<usize>().expect("Syntax error");
                let cnword = &line[4..7].trim();
                cn = cnword.parse::<u32>().expect("Syntax error");
                let clword = &line[7..10].trim();
                cl = clword.parse::<u32>().expect("Syntax error");
                let slword = &line[10..15].trim();
                sl = slword.parse::<u32>().expect("Syntax error");
            }
            industries.insert(Ix, IndustryWorking::new(self.industries[&Ix].Name()));
            let industry = industries.get_mut(&Ix).unwrap();
            industry.SetCarsNum(cn);
            industry.SetCarsLen(cl);
            industry.SetStatsLen(sl);
        }
        for (Ix, industry)  in industries.iter_mut() {
            if self.statsPeriod == 1 {
                industry.SetCarsNum(0);
                industry.SetCarsLen(0);
                industry.SetStatsLen(0);
            }
            industry.IncrementStatsLen(self.industries[Ix].TrackLen());
        }
        Ok((Gx,industries))
    }
    /// Restart loop numbers.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ nothing.
    fn RestartLoop(&mut self) {
        self.carsMoved = 0;
        self.carsAtDest = 0;
        self.carsNotMoved = 0;
        self.carsMovedOnce = 0;
        self.carsMovedTwice = 0;
        self.carsMovedThree = 0;
        self.carsMovedMore = 0;
        self.carMovements = 0;
        self.carsInTransit = 0;
        self.carsAtWorkBench = 0;
        for car in &self.cars {
            if car.Location() == 0 {self.carsAtWorkBench += 1;}
            else {
                if car.Location() == car.Destination() {
                    self.carsAtDest += 1;
                } else {
                    self.carsInTransit += 1;
                }
                self.carMovements += car.MovementsThisSession();
                if car.MovementsThisSession() == 0 {self.carsNotMoved += 1;}
                if car.MovementsThisSession() >  0 {self.carsMoved += 1;}
                if car.MovementsThisSession() == 1 {self.carsMovedOnce += 1;}
                if car.MovementsThisSession() == 2 {self.carsMovedTwice += 1;}
                if car.MovementsThisSession() == 3 {self.carsMovedThree += 1;}
                if car.MovementsThisSession() >  3 {self.carsMovedMore += 1;}
            }
        }
        self.carsAtDest_carsInTransit = self.carsAtDest + self.carsInTransit;
    }
    /// The constructor for the system.  
    ///
    /// Takes the path to a system file (typically _system.dat_) and loads the 
    /// complete system. The system file contains the names of the additional 
    /// files, containing the remaining system data.  All of the files are 
    /// presumbed to exist in the same directory as the system file.  All of 
    /// the files are loaded and a sanity check is made to insure that the 
    /// data is sane.
    /// ## Parameters:
    /// - systemfile Pathname to the system file.
    ///
    /// __Returns__ a freshly initialized System struct.
    pub fn new(systemfile: String) -> (Self, HashMap<usize, IndustryWorking>) {
        let systemfilePath: PathBuf = fs::canonicalize(systemfile)
                .expect("Path not found");
        //let systemDirectory = systemfilePath.with_file_name("");
        let f = File::open(systemfilePath.to_str().unwrap())
                .expect("Cannot open system file");
        let mut reader = BufReader::new(f);
        //============================================================================
        //
        // Read System and File names
        //
        //============================================================================
	// Get system name
        let systemname = Self::SkipCommentsReadLine(&mut reader).expect("Read error");
	// Get name of industries file.
        let industriesfile: PathBuf = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
	// Get name of trains file
        let trainsfile: PathBuf = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
	// Get name of Train Orders file.
        let ordersfile: PathBuf = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
	// Get name of owners file
        let ownersfile: PathBuf = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
	// Get name of car types file
        let cartypesfile: PathBuf = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
	// Get name of cars file
        let carsfile: PathBuf = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
	// Get name of stats file
        let statsfile: PathBuf = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
        let mut this = Self {systemFile: systemfilePath.to_str().unwrap().to_string(), 
              systemName: systemname.clone(), 
              industriesFile: industriesfile.to_str().unwrap().to_string(), 
              trainsFile: trainsfile.to_str().unwrap().to_string(), 
              ordersFile: ordersfile.to_str().unwrap().to_string(), 
              ownersFile: ownersfile.to_str().unwrap().to_string(), 
              carTypesFile: cartypesfile.to_str().unwrap().to_string(), 
              carsFile: carsfile.to_str().unwrap().to_string(), 
              statsFile: statsfile.to_str().unwrap().to_string(), 
              divisions: HashMap::new(), maxDivision: 0,
              stations: HashMap::new(), maxStation: 0,
              trains: Arc::new(HashMap::new()), maxTrain: 0, trainIndex: HashMap::new(), 
              industries: Arc::new(HashMap::new()), maxIndustry: 0,
              carTypesOrder: Vec::new(), carTypes: HashMap::new(), 
              carGroups: Vec::new(), owners: HashMap::new(),
              cars: Vec::new(), switchList: SwitchList::new(), 
              sessionNumber: 0, 
              shiftNumber: 1, totalShifts: 0, 
              ranAllTrains: 0, totalPickups: 0, 
              totalLoads: 0, totalTons: 0, 
              totalRevenueTons: 0, trainPrintOK: false, 
              wayFreight: false, deliver: false, 
              trainLength: 0, numberCars: 0, 
              trainTons: 0, trainLoads: 0, 
              trainEmpties: 0, trainLongest: 0, 
              statsPeriod: 0, carsMoved: 0, 
              carsAtDest: 0, carsNotMoved: 0, 
              carsMovedOnce: 0, carsMovedTwice: 0, 
              carsMovedThree: 0, carsMovedMore: 0, 
              carMovements: 0, carsInTransit: 0, 
              carsAtWorkBench: 0, 
              carsAtDest_carsInTransit: 0, 
              printYards: false, printAlpha: false, 
              printAtwice: false, printList: false, 
              printLtwice: false, printDispatch: false, 
              printem: false, 
              carDestIndex: 0, curDivIndex: 0, originYardIndex: 0,
              trainLastLocationIndex: 0 };

	// Read in divisions
        this.ReadDivisions(&mut reader).expect("Read error");
        //println!("Read divisions");
	// Read in stations.
        this.ReadStations(&mut reader).expect("Read error");
        //println!("Read Stations");
	// Read industries file
        this.ReadIndustries(&industriesfile).expect("Read error");
        //println!("Read Industries");
	// Read in trains file
        this.ReadTrains(&trainsfile).expect("Read error");
        //println!("Read Trains");
	// Read in Train Orders file
        this.ReadTrainOrders(&ordersfile).expect("Read error");
        //println!("Read TrainOrders");
	// Read in car types file
        this.ReadCarTypes(&cartypesfile).expect("Read error");
        //println!("Read CarTypes");
	// Read in owners file
        this.ReadOwners(&ownersfile).expect("Read error");
        //println!("Read Owners");
	// Load in Cars file)
        this.LoadCarFile().expect("Read error");
        //println!("Loaded Cars");
	// Load in stats file.
        let (count, working_industries) = this.LoadStatsFile().expect("Read error");
        //println!("Loaded Stats");
	// Initialize assignment loop variables.
        this.RestartLoop();
        //println!("Restarted Loop");
        (this, working_industries)
    }
    ///   Function to write one car to disk.
    ///
    /// ## Parameters:
    /// - car The car to write.
    /// - w The buffer to write the car to.
    ///
    /// __Returns__ () or an Err.
    fn WriteOneCarToDisk(&self,car: &Car,w: &mut BufWriter<File>) -> 
            std::io::Result<()> {
        writeln!(w,
                "{:1},{:>9},{:<8},{:<18},{:5},{:1},{:1},{:4},{:5},{:1},{:1},{:1},{:3>},{:1},{:3},{:3},{:3},{:3},{:4},{:4}",
                car.Type(),car.Marks(),
                car.Number(),car.Divisions(),car.Length(),car.Plate(),
                car.WeightClass(),car.LtWt(),car.LdLmt(),
                if car.LoadedP() {'L'} else {'E'},
                if car.OkToMirrorP() {'Y'} else {'N'},
                if car.FixedRouteP() {'Y'} else {'N'},
                car.Owner(),
                if car.IsDoneP() {'Y'} else {'N'},
                car.LastTrain(),car.MovementsThisSession(),
                car.Location(),car.Destination(),
                car.Trips(),car.Assignments())
    }
    /// Save cars (and statistics).
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ true if the save was successful, false otherwise.
    pub fn SaveCars(&mut self,industries: &HashMap<usize, IndustryWorking>) -> bool {
        let mut backupfile = PathBuf::from(self.carsFile.clone());
        backupfile.set_extension("bak");
        let mut tempfile = PathBuf::from(self.carsFile.clone());
        tempfile.set_file_name(format!("CARS{:06}",rand::rng().random_range(0..1000000)));
        {
            let junkf = File::create(tempfile.to_str().unwrap())
                           .expect("Could not open junk file stream!");
            let mut junkfilestream = BufWriter::new(junkf);
            let oldf = File::open(self.carsFile.to_string())
                            .expect("Could not open cars file (reading)");
            let mut oldcarsstream = BufReader::new(oldf);
            let backupf = File::create(backupfile.to_str().unwrap())
                            .expect("Could not open backup file (write)");
            let mut backupcarsstream = BufWriter::new(backupf);
            let mut line = String::new();
            oldcarsstream.read_line(&mut line)
                        .expect("Could not read cars file (Session Number)");
            let oldSessionNumber = line.trim().parse::<u32>()
                        .expect("Syntax error");
            line.clear();
            oldcarsstream.read_line(&mut line)
                        .expect("Could not read cars file (Shift Number)");
            let oldShiftNumber = line.trim().parse::<u8>()
                        .expect("Syntax error"); 
            line.clear();
            oldcarsstream.read_line(&mut line)
                        .expect("Could not read cars file (Total Cars)");
            let oldTotalCars = line.trim().parse::<u32>()
                        .expect("Syntax error"); 
            if self.ranAllTrains == 0 {
                self.sessionNumber = oldSessionNumber;
                self.shiftNumber = oldShiftNumber;
            }
            let totalCars = self.cars.len() + 10;
            writeln!(junkfilestream," {}",self.sessionNumber)
                    .expect("Error writing junk file");
            writeln!(junkfilestream," {}",self.shiftNumber)
                    .expect("Error writing junk file");
            writeln!(junkfilestream," {}",totalCars)
                    .expect("Error writing junk file");
        
            writeln!(backupcarsstream," {}",oldSessionNumber)
                    .expect("Error writing backup file");
            writeln!(backupcarsstream," {}",oldShiftNumber)
                    .expect("Error writing backup file");
            writeln!(backupcarsstream," {}",oldTotalCars)
                    .expect("Error writing backup file");
        
            self.totalShifts += 1;
            self.NextShift();
        
            let mut Cx = 0;
            loop {
                line.clear();
                let result = oldcarsstream.read_line(&mut line);
                if result.is_err() {
                    return false;
                }
                if line.len() == 0 && result.unwrap() == 0 {
                    break;
                }
                write!(backupcarsstream,"{}",line)
                    .expect("Error writing backup file");
                let trimline = line.trim();
                if trimline.len() == 0 || trimline.starts_with("'") {
                    write!(junkfilestream,"{}",line)
                    .expect("Error writing junk file");
                } else {
                    let car = &self.cars[Cx]; Cx += 1;
                    if car.Length() > 0 {
                        if car.Destination() != IND_SCRAP_YARD {
                            self.WriteOneCarToDisk(&car,&mut junkfilestream)
                                .expect("Error writing car");
                        }
                    }
                }
            }
            while Cx < self.cars.len() {
                let car = &self.cars[Cx]; Cx += 1;
                if car.Length() > 0 {
                    if car.Destination() != IND_SCRAP_YARD {
                        self.WriteOneCarToDisk(&car,&mut junkfilestream)
                            .expect("Error writing car");
                     }
                }
            }
        }
        {
            let junkf = File::open(tempfile.to_str().unwrap())
                                .expect("Could not open junk file stream!");
            let mut junkfilestream = BufReader::new(junkf);
            let newf = File::create(self.carsFile.to_string())
                                .expect("Could not open cars file (writing)");
            let mut newcarsstream = BufWriter::new(newf);
            loop {
                let mut line = String::new();
                let result = junkfilestream.read_line(&mut line);
                if result.is_err() {
                    return false;
                }
                if line.len() == 0 && result.unwrap() == 0 {
                    break;
                }
                write!(newcarsstream,"{}",line)
                        .expect("Error writing cars file");
            }
        }
        std::fs::remove_file(tempfile.to_str().unwrap())
            .expect("Could not remove junk file");
        {
            let statf = File::create(self.statsFile.to_string())
                                .expect("Could not open stats file (writing)");
            let mut statsstream = BufWriter::new(statf);
            self.statsPeriod += self.ranAllTrains;
            writeln!(statsstream,"{},",self.statsPeriod)
                .expect("Could not write stats file");
            for (Ix, ind) in industries.iter() {
                writeln!(statsstream,"{},{},{},{}",
                        Ix,ind.CarsNum(),ind.CarsLen(),ind.StatsLen())
                        .expect("Could not write stats file");
                 
            }
        }
        self.ranAllTrains = 0;
        true
    }
    /// Check if an industry takes a certain car.
    ///
    /// ## Parameters:
    /// - Ix The industry to check.
    /// - Cx The car to check.
    ///
    /// __Returns__ true if this industry can take the specified car.
    fn IndustryTakesCar(&self,industry: &IndustryFile,car: usize) -> bool {
        if self.cars[car].TmpStatus() {
            industry.LoadsAccepted().contains(self.cars[car].Type())
        } else {
            industry.EmptiesAccepted().contains(self.cars[car].Type())
        }
    }
    /// Check to see if a certain car can be mirrored on a fixed route at a 
    /// certain industry. 
    ///
    /// ## Parameters:
    /// - Ix The industry to check.
    /// - Cx The car to check.
    ///
    /// __Returns__ true if this car can be mirrored on a fixed route at this
    /// industry.
    fn FixedRouteMirrorCheck(&self,Cx: usize,industry: &IndustryFile) -> bool {
        if !self.cars[Cx].FixedRouteP() {return true;}
        let mystation = industry.MyStationIndex();
        let station = self.StationByIndex(mystation);
        if station.is_none() {return false;}
        let station = station.unwrap();
        let division = self.DivisionByIndex(station.DivisionIndex());
        if division.is_none() {return false;}
        let MirrorDivI = division.unwrap();
        let MirrorDivS = MirrorDivI.Symbol();
        // if  the car is loaded --
        //
        //  Make sure the industry's division is included in this car's home list.
        if self.cars[Cx].TmpStatus() {
            if !self.cars[Cx].Divisions().contains(MirrorDivS) {
                return false;
            }
        } else {
            // If the car is empty --
            //
            //  The industry's division list (normally only applicable to 
            //  loaded cars) must have a division in common with the car's 
            //  home division list. When an assignment is made (later), this 
            //  empty fixed route  car is directed by the industry's division 
            //  list and it's own home list.
            for pxdiv in industry.DivisionControlList().chars() {
                if self.cars[Cx].Divisions().contains(pxdiv) {return true;}
            }
            return false;
        }
        true
    }
    /// Get a car type class instance pointer given a car type.
    ///
    /// ## Parameters:
    /// - c The car type character.
    ///
    /// __Returns__ a reference to a CarType or None
    fn TheCarType(&self, c: char) -> Option<&CarType> {
        self.carTypes.get(&c)
    }
    ///  Return car status information. 
    ///
    /// ## Parameters:
    /// - Cx the car index
    ///
    /// __Returns__ a tuple containing two Strings, the car's status (loaded or 
    /// empty) and its car type description.
    fn GetCarStatus(&self, Cx: usize) -> (String, String) {
        let car: &Car = &self.cars[Cx];
        let Ct = self.carTypes.get(&car.Type());
        (if car.LoadedP() {String::from("LOADED")} else {String::from("EMPTY")},
         match Ct {
            Some(val) => val.Type(),
            None => String::from("Unknown")
         })
        
    }
    /// Car assignment procedure.  
    ///
    /// The is one of the main workhorse procedures.  It goes through all of 
    /// the cars, finding ones that are ready to be moved and determines where 
    /// they could be moved to, based on a number of critiera, such as whether 
    /// they are loaded or empty,
    /// 
    /// Main car assignment function.  Loops through all cars looking for cars*    /// that are unassigned and trys to find assignments for those cars.
    /// Assignments are based on things like car type and whether it is loaded
    /// or empty.  Loaded cars are forwarded to industries that consume the
    /// type of load and empty cars are forwarded either to their home yards
    /// or to industries that produce loads for that sort of car.
    ///
    /// Checks are made to be sure that an industry does not get more cars
    /// than it can handle and so on.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ nothing.
    pub fn CarAssignment(&mut self,
                         working_industries: &mut HashMap<usize, IndustryWorking>) {
        let mut RouteCars: i32 = 0;
        let industries = Arc::clone(&self.industries);
        let industries_keys: Vec<&usize> = industries.keys().collect();
        let mut LastIx = 0;
        for AssignLoop in 1..3 { // 1
            println!("{} ({})",self.SystemName(),AssignLoop);
            // ----------- Outer Loop Initialization --------------
            for Ix1 in industries.keys() { // 2
                match working_industries.get_mut(Ix1) {
                    Some(ind) => {ind.SetUsedLen(0);},
                    None => {
                        working_industries.insert(*Ix1,
                                    IndustryWorking::new(industries[Ix1].Name()));
                    },
                };
            } // 2
            for Cx in 0..self.cars.len() { // 2
                //let car: &mut Car = self.cars.get_mut(Cx).unwrap();
                //eprintln!("*** Outer Loop: car {} has Destination {}",self.cars[Cx],self.cars[Cx].Destination());
                if self.cars[Cx].Destination() == IND_SCRAP_YARD {continue;}
                //eprintln!("*** Outer Loop: car {} has Location {}",self.cars[Cx],self.cars[Cx].Location());
                if self.cars[Cx].Location() == IND_RIP_TRACK {continue;}
                if self.cars[Cx].Destination() == IND_RIP_TRACK { // 3
                    let newlocation = self.cars[Cx].Location();
                    self.cars[Cx].SetDestination(newlocation);
                } // 3
                //eprintln!("*** Outer Loop(2): car {} has Destination {}",self.cars[Cx],self.cars[Cx].Destination());
                { // 3
                    let temp = self.cars[Cx].LoadedP();
                    self.cars[Cx].SetTmpStatus(temp);
                } // 3
                if self.cars[Cx].Destination() == self.cars[Cx].Location() { // 3
                    // This marks the car for assignment
                    self.cars[Cx].SetDestination(IND_RIP_TRACK);
                    // --------------------------------------------------------------
                    // If this is a MIRROR industry, the car moves to a new location,
                    // but it does not change its status - if it was loaded then the
                    // mirror target must load such cars, and so on.
                    // --------------------------------------------------------------
                    let mut CarWasMirrored: bool = false;
                    let mut LocIndIx = self.cars[Cx].Location();
                    if LocIndIx != IND_RIP_TRACK && industries[&LocIndIx]
                                                        .MyMirrorIndex() != 0 { // 4
                        if self.cars[Cx].OkToMirrorP() { // 5
                            let MirrorInd = industries[&LocIndIx]
                                                .MyMirrorIndex();
                            // -----------------------------------------------------------
                            // First check to see that the industry would receive this car
                            // in its mirrored loaded or empty state ...  
                            // -----------------------------------------------------------
                            { // 6
                                let temp = !self.cars[Cx].LoadedP();
                                self.cars[Cx].SetTmpStatus(temp);
                            } // 6
                            if self.IndustryTakesCar(&industries[&MirrorInd],Cx) { // 6
                                // ------------------------------------------------------
                                // Fixed route check then uses the car state that will be
                                // used for making an assignment from the mirrored 
                                // industry ...
                                // ------------------------------------------------------
                                { // 7
                                    let temp = self.cars[Cx].LoadedP();
                                    self.cars[Cx].SetTmpStatus(temp);
                                } // 7
                                if self.FixedRouteMirrorCheck(Cx,&industries[&MirrorInd]) { // 7
                                    // Success! This car can in fact be mirrored! It will soon
                                    // be assigned from this new location.
                                    working_industries
                                        .get_mut(&LocIndIx)
                                        .unwrap()
                                        .RemoveCar(Cx);
                                    self.cars[Cx].SetLocation(MirrorInd);
                                    LocIndIx = self.cars[Cx].Location();
                                    working_industries
                                        .get_mut(&MirrorInd)
                                        .unwrap()
                                        .AddCar(Cx);
                                    CarWasMirrored = true;
                                } // 7 
                            } // 6
                        } // 5
                    } // 4
                    if !CarWasMirrored { // 4
                        if self.cars[Cx].EmptyP() { // 5
                            // ---------------------------------------------------------
                            // An empty car in a yard, will remain empty for purpose of
                            // finding an assignment. Otherwise this car becomes a load.
                            // ---------------------------------------------------------
                            if industries[&LocIndIx].Type() != 'Y' { // 6
                                self.cars[Cx].SetTmpStatus(true);
                            } else { // 6
                                self.cars[Cx].SetTmpStatus(false);
                            } // 6
                        } else { // 5
                            // ---------------------------------------------------------
                            // If this is a RELOAD industry, the car is loaded again,
                            // but only if the industry ships out this type of car.
                            //  ---------------------------------------------------------
                            self.cars[Cx].SetTmpStatus(false);
                            if industries[&LocIndIx]
                                .Reload() { // 6
                                if industries[&LocIndIx]
                                    .EmptiesAccepted()
                                    .contains(self.cars[Cx].Type()) { // 7
                                    self.cars[Cx].SetTmpStatus(true);
                                } // 7
                            } // 6
                        } // 5
                    } // 4
                } // 3
                // Car has no assignment
	        // ========================================================================
	        // If the car has a destination then add this car's
                // length to the destination's assigned track space
                if self.cars[Cx].Destination() != IND_RIP_TRACK { // 3
                    let dest = self.cars[Cx].Destination();
                    let carlen = self.cars[Cx].Length();
                    working_industries
                        .get_mut(&dest)
                        .unwrap()
                        .AddToUsedLen(carlen as u32);
                 } // 3
            } // 2
            let mut reverse: bool = false;
            if rand::random_bool(0.5) { // 3
                reverse = true;
                println!("Checking cars from {} to {}",self.cars.len()-1,0);
            } else { // 3
                println!("Checking cars from {} to {}",0,self.cars.len()-1);
            } // 3
            let mut CountCars = 0;
            for CxI in 0..self.cars.len() {
                let mut Cx = CxI;
                if reverse {
                    Cx = (self.cars.len()-1) - CxI;
                }
                let mut HaveDest = false;
                self.cars[Cx].SetNotDone();
                self.cars[Cx].ClearMovementsThisSession();
                self.cars[Cx].SetLastTrain(0);
                //eprintln!("*** in assignment loop: car {} has destination {}",self.cars[Cx],self.cars[Cx].Destination());
                //eprintln!("*** in assignment loop: car {} has location {}",self.cars[Cx],self.cars[Cx].Location());
                if self.cars[Cx].Destination() != IND_RIP_TRACK {continue;}
                if self.cars[Cx].Location() == IND_RIP_TRACK {continue;}
                CountCars += 1;
                println!("Processing car {}",self.cars[Cx]);
                println!("Cars inspected {}",CountCars);
	        println!("Cars Assigned  {}",RouteCars);
                println!("Last Industry  {} ({})",industries_keys[LastIx],
                                   industries[industries_keys[LastIx]].Name());
                println!("");
                println!("");
                println!("{} {} at {}",
                    if self.cars[Cx].TmpStatus() {"Loaded"} else {"Empty"},
                    self.cars[Cx],
                    industries[&self.cars[Cx].Location()]
                            .Name());
                let mut Ix = LastIx;
                let mut IIx = self.cars[Cx].Location();
                for IndPriorityLoop in 1..5 {
                    // ----------- Inner Loop --------------
                    // The purpose of the PassLoop is to try to reload cars in the
                    // same division where they are, whether they are "offline" or
                    // are "online"
                    for PassLoop in 1..3 {
                        for IndLoop in 0..industries.len() {
                            Ix += 1;
                            if Ix >= industries_keys.len() {
                                Ix = 0;
                                Ix += 1;
                            }                                
                            IIx = *industries_keys[Ix];
                            if !industries.contains_key(&IIx) {continue;}
                            if industries[&IIx].Priority() != IndPriorityLoop {continue;}
                            if industries[&IIx].AssignLen() == 0 {continue;}
                            // Cars are never assigned to yards
                            //  --------------------------------
                            if industries[&IIx].Type() == 'Y' {continue;}
                            // If the car is at an industry that mirrors, never route
                            // the car to the mirror itself. This does not apply when
                            // the car is not allowed to mirror.
                            // ------------------------------------------------------
                            if industries[&self.cars[Cx].Location()].MyMirrorIndex() != 0 {
                                if industries[&self.cars[Cx].Location()].MyMirrorIndex() == IIx {
                                    if self.cars[Cx].OkToMirrorP() {continue;} 
                                }
                            }
                            // Does industry accept this car ?
                            // -------------------------------
                            if !self.IndustryTakesCar(&industries[&IIx],Cx) {continue;}
                            // Eliminate incompatible industries for this car
                            // ----------------------------------------------
                            if self.cars[Cx].Plate() > industries[&IIx].MaxPlate() {continue;}
                            if self.cars[Cx].WeightClass() > industries[&IIx].MaxWeightClass() {continue;}
                            if self.cars[Cx].Length() > industries[&IIx].MaxCarLen() {continue;}
                            // Is there space available for this car ?
                            // -------------------------------------
                            if working_industries[&IIx].UsedLen() + 
                                self.cars[Cx].Length() > 
                                    industries[&IIx].AssignLen() {continue;}
                            let CarDivI = &self.divisions[
                                    &self.stations[
                                        &industries[
                                            &self.cars[Cx].Location()]
                                                .MyStationIndex()].DivisionIndex()];
                            let CarDivS = CarDivI.Symbol();
                            let IndDivI = &self.divisions[
                                            &self.stations[
                                                &industries[&IIx]
                                                    .MyStationIndex()]
                                                        .DivisionIndex()];
                            let IndDivS = IndDivI.Symbol();
                            // -------------------------------------------------
                            // If the car has a fixed route then the destination
                            // must be in the car's home list.
                            // -------------------------------------------------
                            if self.cars[Cx].FixedRouteP() {
                                // AND the destination ALSO must be in the current car
                                // location's destination list - regardless of whether
                                // the car is loaded/empty -- unless the list is empty.
                                // ---------------------------------------------------
                                let DCL_temp = industries[
                                    &self.cars[Cx]
                                        .Location()]
                                            .DivisionControlList();
                                if DCL_temp.len() > 0 {
                                    if !DCL_temp.contains(IndDivS) {continue;}
                                }
                            }
                            // Car has a FIXED route
                            // ===========================================================
                            // EMPTY CARS
                            // ===========================================================
                            if !self.cars[Cx].TmpStatus() {
                                if industries[&IIx].Type() == 'O' &&
                                   industries[&self.cars[Cx].Location()].Type() != 'I' {
                                   LastIx = Ix;
                                   working_industries
                                        .get_mut(&self.cars[Cx].Location())
                                        .unwrap()
                                        .RemoveCar(Cx);
                                    self.cars[Cx].SetLocation(IIx);
                                    if let Some(val) = working_industries.get_mut(&IIx) { val.AddCar(Cx); };
                                    HaveDest = true;
                                    break;
                                }
                                // ----------------------------------------------------
                                //
                                // Ok! The Car and Industry -ARE- in the same area.
                                // The empty car will travel a shorter distance to
                                // be reloaded.
                                //
                                // NOTE a key assumption is that from this area, it is
                                // possible to route the car back to its HOME division
                                // when the industry is not in a home div.
                                //
                                // ----------------------------------------------------
                                if self.cars[Cx].Divisions().len() > 0 &&
                                   industries[&IIx].DivisionControlList().len() > 0 {
                                    // If the car is in a home division, we're ok
                                    let mut YesNo: bool;
                                    if !self.cars[Cx].Divisions().contains(CarDivS) {
                                        YesNo = false;
                                        for PxDiv in industries[&IIx].DivisionControlList().chars() {
                                            if self.cars[Cx].Divisions().contains(PxDiv) {
                                                YesNo = true;
                                                break;
                                            }
                                        }
                                        if YesNo {continue;}
                                    }
                                    LastIx = Ix;
                                    RouteCars += 1;
                                    HaveDest = true;
                                    break;
                                }
                                // Car and Industry are in SAME AREA
                                // -------------------------------------------------
                                // On the first pass for empty cars, skip industries
                                // that are outside the car's present AREA.
                                // -------------------------------------------------
                                if PassLoop == 1 && self.cars[Cx].FixedRouteP() {continue;}    
                                // ------------------------------------------------------
                                //
                                // The EMPTY and an Industry are not in the same area, so
                                // check the Car's Division List to see whether it can be
                                // routed to the Industry for loading.
                                //
                                // ------------------------------------------------------
                                if self.cars[Cx].Divisions().len() == 0 ||
                                   self.cars[Cx].Divisions().contains(IndDivS) {
                                    LastIx = Ix;
                                    RouteCars += 1;
                                    HaveDest = true;
                                    break;
                                }
                                if self.cars[Cx].FixedRouteP() {continue;}
                                // ------------------------------------------------------
                                //
                                // Last chance for an empty -- if the car is offline then
                                // we let it go to any destination where it can be loaded.
                                // 
                                // ------------------------------------------------------
                                if AssignLoop == 2 && PassLoop == 2 {
                                    if industries[&self.cars[Cx].Location()].Type() == 'O' {
                                        LastIx = Ix;
                                        RouteCars += 1;
                                        HaveDest = true;
                                        break;
                                    }
                                }
                                // END of Empty Car case
                                // ===========================================================
                                // LOADED CARS
                                // ===========================================================
                            } else {
                                // self.cars[Cx].tmpStatus == true (loaded)
                                // If the Car and the Industry are in the same area AND
                                // the Industry is Offline and the Car is Offline, then
                                // do not assign the Car to the Industry.
                                // --------------------------------------------------------
                                if CarDivI.Area() == IndDivI.Area() {
                                    if industries[&IIx].Type() == 'O' &&
                                       industries[
                                            &self.cars[Cx].Location()].Type() == 'I' {continue;}
                                }
                                // When the Car is loaded where it can go is under control
                                // of the Industry's Division List
                                // -------------------------------------------------------
                                let mut DestList = industries[&self.cars[Cx].Location()].DivisionControlList();
                                // 
                                // CHANGE 6/24/96 -- As a last resort, use the car's list
                                // of home divisions as possible destinations. Usually we
                                // got this far because the car is at an industry outside
                                // of its home divisions, that does NOT ship to the car's
                                // home divisions.
                                // ------------------------------------------------------
                                if AssignLoop == 2 && PassLoop == 2 {
                                    // Oops! Since I allow an offline car to be routed to
                                    // any destination of the shipper, I do not use a car
                                    // home division list in that case.
                                    // --------------------------------------------------
                                    // if (car->Location()->Type() == 'I') {
                                        DestList = self.cars[Cx].Divisions();
                                    // }
                                }
                                // END CHANGE 6/24/96
                                // ------------------
                                if DestList.len() == 0 ||
                                   DestList.contains(IndDivS) {
                                    // ----------------------------------------------------
                                    //
                                    // The car's current industry can ship to this industry
                                    //
                                    // Normally if the car itself is NOT in a home division
                                    // then it must be routed BACK to a home division
                                    //
                                    // Now I make an exception -- if the car is offline, it
                                    // may be routed to any valid destination division from
                                    // the current industry.
                                    //
                                    // The reason for this is that cars at offline industry
                                    // may be "relocated" somewhere in the same area, and I
                                    // don't check home divisions when I do it (see above).
                                    //
                                    // ----------------------------------------------------
                                    if AssignLoop == 2 && PassLoop == 2 {
                                        if industries[&self.cars[Cx].Location()].Type() == 'O' {
                                            // GOTO IndustryIsOk
                                            LastIx = Ix;
                                            RouteCars += 1;
                                            HaveDest = true;
                                            break;
                                        }
                                    }
                                    if self.cars[Cx].Divisions().len() > 0 {
                                        // If the car is not now in a home division ..
                                        // -------------------------------------------
                                        if !self.cars[Cx].Divisions().contains(CarDivS) {
                                            // ANd the industry is not in a home division ..
                                            // ---------------------------------------------
                                            if !self.cars[Cx].Divisions().contains(IndDivS) {
                                                // This industry cannot receive this car
                                                continue;
                                            }
                                        }
                                    }
                                    LastIx = Ix;
                                    RouteCars += 1;
                                    HaveDest = true;
                                    break;
                                } // If you get here you have failed
                            } // Loaded Car case
                        } // IndLoop
                        if HaveDest {break;}
                    } // PassLoop    
                    if HaveDest {break;}
                } // IndPriorityLoo
                if !HaveDest {
                    // We failed to find a destination. If the car is EMPTY and if the
                    // car is sitting at an ONLINE industry, then assign this car just
                    // to move to the industry's home yard.
                    // IF AssignLoop% = 2 THEN
                    //   IF CrsTmpStatus(Cx%) = "E" AND IndsType(CrsLoc%(Cx%)) = "I" THEN
                    //     Ix% = DivsHome%(StnsDiv%(IndsStation%(CrsLoc%(Cx%))))
                    //     GOTO HaveDest
                    //   END IF
                    // END IF ' AssignLoop% = 2 i.e. last chance
                    //
                    // If we fall into this code, then we have failed to find any
                    // destination for this car -- so just leave it alone for now.
                    IIx = self.cars[Cx].Location();
                    {
                        let temp = self.cars[Cx].LoadedP();
                        self.cars[Cx].SetTmpStatus(temp);
                    }
                }
                // HaveDest:
                self.cars[Cx].SetDestination(IIx);
                if self.cars[Cx].TmpStatus() {
                    self.cars[Cx].Load();
                } else {
                    self.cars[Cx].UnLoad();
                }
                // Adjust the used assignment space for this industry -
                // Should I do this only if the car is not at its dest?
                // ----------------------------------------------------
                working_industries
                    .get_mut(&IIx).unwrap()
                    .AddToUsedLen(self.cars[Cx].Length());
                if IIx != self.cars[Cx].Location() {
                    // Whenever a car receives an assignment to move somewhere else
                    // we count this as 1 assignment for our statistics.
                    self.cars[Cx].IncrementAssignments();
                    let (Status, CarTypeDesc) = self.GetCarStatus(Cx);
                    println!("{} {} is {}",self.cars[Cx],CarTypeDesc,Status);
                    println!(" Now at {}",
                        industries[&self.cars[Cx].Location()].Name());
                    println!(" Send to {}",
                        industries[&self.cars[Cx].Destination()].Name());
                    //println!(" IndsAssignLen = {} IndsUsedLen = {}",
                    //    industries[&self.cars[Cx].Destination()].AssignLen(),
                    //    industries[&self.cars[Cx].Destination()].UsedLen());
                }
            } // 3 For Cx loop
        } // 2 AssignLoop
        let mut hflag = true;
        for Cx in 0..self.cars.len() {
            let car: &Car = &self.cars[Cx];
            if car.Location() == car.Destination() {
                if hflag {
                    println!("\n\nCars without assignments");
                    hflag = false;
                }
                let typeDescr = match self.TheCarType(car.Type()) {
                    Some(val) => val.Type(),
                    None => String::from("Unknown"),
                };
                println!("{} {} @ {}",car,typeDescr,
                       industries[&car.Location()].Name());
            }
        }
    } // 1
    /// Update industry car counts.
    ///
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ nothing.
    fn GetIndustryCarCounts(&mut self,working_industries: &mut HashMap<usize, IndustryWorking>) {
        for Ix in working_industries.values_mut() {
            Ix.SetUsedLen(0);
        }
        for Cx in 0..self.cars.len() {
            let car = &self.cars[Cx];
            match working_industries.get_mut(&car.Location()) {
                Some(location) => {location.AddToUsedLen(car.Length());},
                None           => (),
            }
        }
    }
    ///  Print a train's current location.
    /// Print/display our current location and status
    /// ## Parameters:
    /// - train The train to print.
    /// - Px The stop number that train is at. 
    ///
    /// __Returns__ nothing
    fn PrintTrainLoc(&mut self,train: &Train,Px: usize) {
        println!("{} is now at station {}",
            train.Name(), match train.Stop(Px) {
                None => String::from(""),
                Some(theStop) => match theStop {
                    Stop::StationStop(station) => 
                        self.stations[&station].Name(),
                        Stop::IndustryStop(Ix) => {
                            let station = self.industries[&Ix].MyStationIndex();
                            self.stations[&station].Name()
                        },
                },
            });
    }
    const CARHOLE: usize = 999999;
    /// Pick up one car.
    /// General helper to pickup a car.
    /// ## Parameters:
    /// - Cx The car index to possibly pick up.
    /// - train The train to pick up the car for.
    /// - boxMove Is this a box move?
    /// - consist The train's consist.
    /// - Px The stop number that train is at. 
    /// - Lx Place in the train to put the car if it is picked up.
    /// - printer Printer device.
    /// - carDest_W The destination work structure
    fn TrainPickupOneCar(&mut self,Cx: usize,train: &Train,boxMove: bool,
		consist: &mut Vec<usize>,Px: usize, Lx: Option<usize>,
		printer: &mut Printer,carDest_W: &mut IndustryWorking) {
    }
    /// Check to see if this other car can be picked up.
    /// ## Parameters:
    /// - car The car to check.
    /// - train The train to check.
    fn OtherCarOkForTrain(car: &Car, train: &Train) -> bool {
        false
    }    
    ///  Check to see if we can really pick up this car.
    /// ## Parameters:
    /// - Cx The car to check.
    /// - train The train to pick up the car for.
    /// - boxMove Is this a box move?
    /// - consist The train's consist.
    /// - didAction Flag to set (update) if something was done.
    /// - Px The stop number that train is at. 
    /// - printer Printer device.
    fn TrainCarPickupCheck(&mut self,Cx: usize,train: &Train,boxMove: bool,
		consist: &mut Vec<usize>,didAction: bool,Px: usize,
		printer: &mut Printer,
                carDest_S: &IndustryFile,
                carDest_W: &mut IndustryWorking) -> bool {
        //  Check for obvious things that prevent the car from being picked up!
        //      Has the car already been picked up?
        if consist.contains(&Cx) {return didAction;}
        //      Has the car already finished moving ?
        let car: &Car = &self.cars[Cx];
        if !boxMove {
            if car.IsDoneP() {return didAction;}
        }
        //      Is car already at its destination ? 
        if car.Location() == car.Destination() {return didAction;}
        //      Is the car too long for this train ?
        if (self.trainLength + car.Length()) > train.MaxLength() {return didAction;}
        //      Is the car too large, or too heavy for the train ?
        if car.Plate() > train.MaxClear() {return didAction;}
        if car.WeightClass() > train.MaxWeight() {return didAction;}
        //      Is the car too large, or too heavy for the destination ?
        if car.Length() > carDest_S.MaxCarLen() {return didAction;}
        if car.Plate()  > carDest_S.MaxPlate() {return didAction;}
        if car.WeightClass() > carDest_S.MaxWeightClass() {return didAction;}
        //      Can the train move this type of car ?
        let trainCarTypes = train.CarTypes();
        if trainCarTypes.len() > 0 {
            if trainCarTypes.chars().nth(0) == Some('-') {
                let trainCarTypes = trainCarTypes.clone();
                if trainCarTypes.contains(car.Type())  {return didAction;}
            } else {
                if !trainCarTypes.contains(car.Type())  {return didAction;} 
            }
        }
        //	That's it for MANIFEST trains -- this car is Ok!
        //	-----------------------------------------------
        if !self.wayFreight {
            carDest_W.SubRemLen(car.Length());
            let Lx = consist.into_iter().position(|cx| *cx == Self::CARHOLE);
            self.TrainPickupOneCar(Cx,train,boxMove,consist,Px,Lx,printer,carDest_W);
            return true;
        }
        // A WAYFREIGHT needs to have some space available - unless it's a yard
        // -----------------------------------------------
        let exp1 = (carDest_W.UsedLen() + car.Length()) <= carDest_S.TrackLen();
        if exp1 || carDest_S.Type() == 'Y' {
            carDest_W.SubRemLen(car.Length());
            let Lx = consist.into_iter().position(|cx| *cx == Self::CARHOLE);
            self.TrainPickupOneCar(Cx,train,boxMove,consist,Px,Lx,printer,carDest_W);
            return true;
        }
        //============================================================================
        // Oops! Now for some fancy footwork -- we look ahead to see whether
        // this train will REMOVE another car from the destination, to create
        // an opening for this car.
        //============================================================================
        for OtherCx in 0..self.cars.len() {
            let otherCar: &Car = &self.cars[OtherCx];
            if !otherCar.Peek() && otherCar.Location() == car.Destination() {
                // Exp1 means the other car has a new destination, and is able to move
                let exp1a = otherCar.Destination() != car.Destination();
                let exp1b = !otherCar.IsDoneP();
                let exp1  = exp1a && exp1b;
                // Exp2 means the removal of the other car will make room for this one	    
                let exp2 = (otherCar.Length() + carDest_W.RemLen()) >= car.Length();
                // Exp3 was used to test to see if removal of this car from its YARD
                // would make room for the other car to replace it -- but this makes
                // no sense in some cases so I deleted this test.
                // bool exp3 = car->Location()->TrackLength() >= ( car->Location()->usedLen - car->Length() + otherCar->Length() );
                if exp1 && exp2 {
                    if Self::OtherCarOkForTrain(otherCar,train) {
                        carDest_W.SubRemLen(car.Length());
                        self.cars[OtherCx].SetPeek(true);
                        let Lx = consist.into_iter().position(|cx| *cx == Self::CARHOLE);
                        self.TrainPickupOneCar(Cx,train,boxMove,consist,Px,Lx,printer,carDest_W);
                        return true;
                    }
                }
            }
        }
        didAction
    }
    ///  Make up a local train.
    /// Basically, starting from the origin, to the next to last stop, pick
    /// up every car in the origin yard that is destined for an industry at
    /// that particular stop -- IF possible.
    ///
    /// A car may not necessarily be picked up - if the destination already
    /// has too many cars, or the train cannot handle this type of car, etc.
    /// ## Parameters:
    /// - train The train to make up.
    /// - boxMove Is this a box move?
    /// - Px The stop number that train is at. 
    /// - consist The train's consist.
    /// - printer Printer device.
    ///
    /// __Returns__ true if something was done, false if not.
    fn TrainLocalOriginate(&mut self,train: &Train,boxMove: bool,
                           Px: usize, consist: &mut Vec<usize>,
                           printer: &mut Printer,
                           working_industries: &mut HashMap<usize, IndustryWorking> ) -> bool {
        let mut didAction = false;
        let industries = Arc::clone(&self.industries);
        for FuturePx in Px+1..train.NumberOfStops()-1 {
            if (self.numberCars+1) > train.MaxCars() {return didAction;}
            for (Ix, ind) in industries.iter() {
                match train.Stop(FuturePx).unwrap() {
                    Stop::StationStop(station) => {
                        if *station != ind.MyStationIndex() {continue;}
                    },
                    Stop::IndustryStop(industry) => {
                        let station = industries[&industry].MyStationIndex();
                        if station != ind.MyStationIndex() {continue;}
                    },
                };
                for Cx in 0..self.cars.len() {
                    if (self.numberCars+1) > train.MaxCars() {return didAction;}
                    let car = &self.cars[Cx];
                    if car.Destination() == *Ix &&
                       car.Location() == self.originYardIndex {
                        self.carDestIndex = *Ix;
                        didAction = self.TrainCarPickupCheck(Cx,train,boxMove,consist,didAction,Px,printer,
                                                &industries[&self.carDestIndex],
                                        working_industries.get_mut(&self.carDestIndex).unwrap());
                    }
                }
            }
        }
        // KLUDGE CITY --
        //
        //  Allow local trains to forward cars under the control of a forwarding
        //  division list.
        if train.DivisionList().len() > 0 {
            for Cx in 0..self.cars.len() {
                //          If this car is at the train's origin yard
                //          -----------------------------------------
                if (self.numberCars+1) > train.MaxCars() {return didAction;}
                let car = &self.cars[Cx];
                let carDestDiv = self.stations[
                                    &industries[
                                        &car.Destination()]
                                    .MyStationIndex()].DivisionIndex();
                let carLocDiv = self.stations[
                                    &industries[
                                        &car.Location()]
                                    .MyStationIndex()].DivisionIndex();
                if self.divisions[&carDestDiv].Home() ==
                     self.divisions[&carLocDiv].Home() {continue;}
                //  The train division list can be exclusive
                //  ----------------------------------------
            let train_divlist = train.DivisionList();
                if train_divlist.chars().nth(0) == Some('-') {
                    if !train_divlist.contains(self.divisions[&carDestDiv].Symbol()) {
                        self.carDestIndex = self.trainLastLocationIndex;
                        didAction = self.TrainCarPickupCheck(Cx,train,boxMove,
                                            consist,didAction,Px,printer,
                                                &industries[&self.carDestIndex],
                                        &mut working_industries.get_mut(&self.carDestIndex).unwrap());
                    }
                } else {
                    // The train division list can include everything - *
                    //
                    // otherwise it specifies which divisions for forwarding 
                    // -------------------------------------------------------
                    if train_divlist == String::from("*") ||
                        train_divlist.contains(self.divisions[&carDestDiv].Symbol()) {
                        self.carDestIndex = self.trainLastLocationIndex;
                        didAction = self.TrainCarPickupCheck(Cx,train,boxMove,
                                                consist,didAction,Px,printer,
                                                &industries[&self.carDestIndex],
                                        &mut working_industries.get_mut(&self.carDestIndex).unwrap());
                    }                  
                }
            }
        }
        didAction
    }
    /// Print a train's consist summary.
    /// ## Parameters:
    /// - train The train to print a summary for.
    /// - consist The train's consist.
    /// -printer Printer device.
    ///
    /// __Returns__ nothing.
    fn TrainPrintConsistSummary(&mut self,train: &Train,
                                consist: &mut Vec<usize>,
                                printer: &mut Printer) {
    }
    ///  Drop cars from a local (box move or way freight).
    /// Drop cars destined for the current (local) industry.
    /// ## Parameters:
    /// - train The train to drop cars from.
    /// - Px The stop number that train is at. 
    /// - consist The train's consist.
    /// - printer Printer device.
    ///
    /// __Returns__ true if cars were dropped otherwise false.
    fn TrainLocalDrops(&mut self,train: &Train,Px: usize,
                        consist: &mut Vec<usize>,
                        printer: &mut Printer) -> bool {
        false
    }
    /// Pick up cars for a local train (box move or way freight).
    /// Basically, look at each industry at the current station. For each
    /// car at the industry, see if there is a logical place to take that
    /// car -- i.e. a stop where we can drop the car.
    /// ## Parameters:
    /// - train The train to pick up cars for.
    /// - boxMove Is this a box move?
    /// - Px The stop number that train is at. 
    /// - consist The train's consist.
    /// - printer Printer device.
    ///
    /// __Returns__ true if cars were picked up, false otherwise.
    fn TrainLocalPickups(&mut self,train: &Train,boxMove: bool,
                        Px: usize, consist: &mut Vec<usize>,
                        printer: &mut Printer) -> bool {
        false
    }
    /// Drop all cars from a train at the current stop (usually
    /// the last stop). 
    /// Drop all cars from a train.  This happens when we get to our final
    /// location.
    /// ## Parameters:
    /// - train The train to drop cars from.
    /// - Px The stop number that train is at. 
    /// - consist The train's consist.
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    fn TrainDropAllCars(&mut self,train: &Train,Px: usize,
                        consist: &mut Vec<usize>,
                        printer: &mut Printer) {
    }
    /// Print a train's final summary.
    /// ## Parameters:
    /// - train The train to print the final summary for.
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    fn TrainPrintFinalSummary(&mut self,train: &Train,printer: &mut Printer) {
    }
    ///  One one local train.
    /// A local train runs YARD to STATION(S) to YARD
    ///
    /// ## Parameters:
    /// - train The train to run.
    /// - boxMove Is this a box move?
    /// - consist The train's consist.
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    fn RunOneLocal(&mut self,train: &Train,boxMove: bool,
                    consist: &mut Vec<usize>,printer: &mut Printer,
                    working_industries: &mut HashMap<usize, IndustryWorking>) {
        let mut didAction; // = false;

        self.wayFreight = true;
        self.deliver = true;
        self.curDivIndex = match train.Stop(0) {
                        None => 0,
                        Some(theStop) => match theStop {
                            Stop::StationStop(station) => self.stations[&station].DivisionIndex(),
                            Stop::IndustryStop(Ix) => {
                                let station = self.industries[&Ix].MyStationIndex();
                                self.stations[&station].DivisionIndex()
                            },
                        },
        };
        self.originYardIndex = self.divisions[&self.curDivIndex].Home();
        let lastLocDivI = 
            match train.Stop(train.NumberOfStops()-1) {
                None => 0,
                Some(theStop) => match theStop {
                    Stop::StationStop(station) => self.stations[&station].DivisionIndex(),
                    Stop::IndustryStop(Ix) => {
                        let station = self.industries[&Ix].MyStationIndex();
                        self.stations[&station].DivisionIndex()
                    },
                },
        };
        self.trainLastLocationIndex = self.divisions[&lastLocDivI].Home();
        // Print and display our starting location.
        self.PrintTrainLoc(train,0);
       	// Originate the train, picking up all of the cars at the originating
        // yard.
        didAction = self.TrainLocalOriginate(train,boxMove,0,consist,printer,
                                working_industries);
        // Display our summary if anything happened
        if didAction {
            self.TrainPrintConsistSummary(train,consist,printer);
        }
	// For each stop...
        for Px in 1..train.NumberOfStops()-1 {
            //didAction = false;
            // Print and display our new location
            self.PrintTrainLoc(train,Px);
            // Do our local drops
            didAction = self.TrainLocalDrops(train,Px,consist,printer);
            // Do our local pickups
            didAction = 
                self.TrainLocalPickups(train,boxMove,Px,consist,printer) || 
                didAction;
            // If anything happened, print/display our summary.
            if didAction {
                self.TrainPrintConsistSummary(train,consist,printer);
            }
        }
        self.PrintTrainLoc(train,train.NumberOfStops()-1);
        // Drop all of the remaining cars.
        self.TrainDropAllCars(train,train.NumberOfStops()-1,consist,printer); 
        // Print our final summary.
        if self.totalPickups > 0 {
            self.TrainPrintFinalSummary(train,printer);
        }
    }
    /// Run one manifest freight train.
    /// A manifest runs from INDUSTRY/YARD to INDUSTRY/YARD
    ///
    /// ## Parameters:
    /// - train The train to run.
    /// - boxMove Is this a box move?
    /// - consist The train's consist.
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    fn RunOneManifest(&mut self,train: &Train, boxMove: bool, 
                        consist: &mut Vec<usize>,printer: &mut Printer,
                        working_industries: &mut HashMap<usize, IndustryWorking>) {
    }
    /// One one passenger train.
    /// Run a passenger train.  Not much happens -- passenger trains are not
    /// involved in freight forwarding...
    /// 
    /// ## Parameters:
    /// - train The train to run.
    /// - boxMove Is this a box move?
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    fn RunOnePassenger(&mut self,train: &Train,boxMove: bool,
                        printer: &mut Printer) {
    }
    ///  Internal function to run a single train.
    /// 
    /// ## Parameters:
    /// - Tx The index of the train to run.
    /// - boxMove Is this a box move?
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    fn InternalRunOneTrain(&mut self,train: &Train,boxMove: bool, 
                           working_industries: &mut HashMap<usize, IndustryWorking>,
                           printer: &mut Printer) {
        let mut consist: Vec<usize> = Vec::new();

	// Initialize counters
        self.totalPickups = 0;
        self.totalLoads = 0;
        self.totalTons = 0;
        self.totalRevenueTons = 0;
        self.trainLength = 0;
        self.numberCars = 0;
        self.trainTons = 0;
        self.trainLoads = 0;
        self.trainEmpties = 0;
        self.trainLongest = 0;

        self.trainPrintOK = false;

        if self.printem && train.Print() {self.trainPrintOK = true;}
        if boxMove {self.trainPrintOK = false;}

        // Make sure the industry remaining lengths are up to date.
        for (Ix, ind) in working_industries.iter_mut() {
            let remlen = self.industries[Ix].TrackLen() - ind.UsedLen();
            ind.SetRemLen(remlen);
        }
        // Fan out based on train type.
        match train.Type() {
            // Way freights are a flavor of local 
            TrainType::Wayfreight => 
                    {self.RunOneLocal(train,boxMove,&mut consist,printer,
                                      working_industries);},
            // As are box moves 
            TrainType::BoxMove =>
                    {self.RunOneLocal(train,boxMove,&mut consist,printer,
                                        working_industries);},
            // Manifest freights. 
            TrainType::Manifest =>
                    {self.RunOneManifest(train,boxMove,&mut consist,printer,
                                        working_industries);},
            // Passenger trains.                    
            TrainType::Passenger =>
                    {self.RunOnePassenger(train,boxMove,printer);},
            _ => (),
        }
    }
    
    /// Run all trains procedure.  
    ///
    /// The is another workhorse procedure.  This procedure runs the initial 
    /// box moves, then the way freights and manifest trains.  It is necessary 
    /// to run the box moves again after running this procedure, unless 
    /// additional sections of the way freights or manifest trains need to be 
    /// run first. 
    ///
    /// Run all of the trains in an operating session.  This simulates all   
    /// movements.  Assuming the session went smoothly, the results of this  
    /// will mirror what actually happens on the layout and all cars that were
    /// moved this session will be at whereever they would be after the      
    /// session.								
    ///
    /// We can then save the car state and/or run the car assignment for the 
    /// next session.  In practice, what will happen is after running this   
    /// code (and manually run the box moves afterward), some of the cars will
    /// be edited to reflect mistakes and problems encounted durring the     
    /// the operating system.  That is, the car data will be fixed to reflect
    /// what really happened.  Presumably a large part of what happened was  
    ///
    /// was supposed to happen.						
    /// This code also creates information about things like switch lists    
    /// relating to switching that the train crew(s) will perform durring the
    /// operating session.							
    ///
    /// ## Parameters:
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    pub fn RunAllTrains(&mut self, 
                        working_industries: &mut HashMap<usize, IndustryWorking>,
                        printer: &mut Printer) {
        // Get the initial car counts.
        self.GetIndustryCarCounts(working_industries);
        // Reset the switch lists.
        self.switchList.ResetSwitchList();
        // Flag that we were called.
        self.ranAllTrains += 1;
        // Display our banner.
        println!("{}",self.SystemName());
        // First runn all of the box moves (yard locals).
        self.RunBoxMoves(printer);
        let boxMove = false;
        // For every train...
        for (Tx, train) in Arc::clone(&self.trains).iter() {
            if train.Type() == TrainType::Manifest ||
               train.Type() == TrainType::Wayfreight {
                if train.Shift() == self.shiftNumber {
                    self.InternalRunOneTrain(train,boxMove,
                                    working_industries,printer);
                }
            }
        }
    }
    /// Run all boxmove trains.  
    /// The is another workhorse procedure.  This procedure runs all of the 
    /// box moves.
    ///
    /// ## Parameters:
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    pub fn RunBoxMoves(&mut self, printer: &mut Printer) {
    }
    ///  Print all of the various yard and switch lists.
    ///
    /// ## Parameters:
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    pub fn PrintAllLists(&self, printer: &mut Printer) {
    }
    /// Run one single train.
    ///
    /// ## Parameters:
    /// - train The train to run.
    /// - boxMove Is this a box move?
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    pub fn RunOneTrain(&mut self, train: usize, boxMove: bool, 
                        printer: &mut Printer) {
    }
    /// Display cars not moved. 
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ nothing.
    pub fn ShowCarsNotMoved(&self) {
        let mut Total = 0;
        let mut CarCount = 0;
        for Cx in 0..self.cars.len() {
            let car = &self.cars[Cx];
            if car.MovementsThisSession() == 0 && !car.IsDoneP() &&
            car.Location() != IND_SCRAP_YARD && 
            car.Location() != IND_RIP_TRACK {
                if Total == 0 {
                    println!("{}",self.SystemName());
                    println!("{:<20}{:<18}{:<29} {}","Cars Not Moved","Car type",
                             "Status  Location","Destination");
                    println!("{:-<79}","");
                }
                let (status, carTypeDescr) = self.GetCarStatus(Cx);
                let Loc = car.Location();
                let LocName = match self.industries.get(&Loc) {
                    Some(val) => val.Name(),
                    None      => String::from("-"),
                };
                let Dest = car.Destination();
                let DestName = if Loc == Dest {String::from("-")} else {
                    match self.industries.get(&Dest) {
                        Some(val) => val.Name(),
                        None      => String::from("-"),
                    }
                };
                println!("{:<11}{:<9}{:<18}{:<8}{:<21} {}", car.Marks(),
                         car.Number(), carTypeDescr, status, LocName, 
                         DestName);
                Total += 1;
                CarCount += 1;
                if Total == 18 {Total = 0;}
            }
        }
        if CarCount == 0 {return;}
        println!("\n                                                    Cars subtotal: {}",CarCount);
    }
    /// Get the train index, given a Train reference.
    ///
    /// ## Parameters:
    /// - train a reference to a Train.
    ///
    /// __Returns__ a train index.
    fn TrainIndex(&self, train: &Train) -> usize {
        for (Tx, atrain) in self.trains.iter() {
            if atrain == train {return *Tx;}
        }
        0
    }
    /// Get the industry index, given an Industry reference.
    ///
    /// ## Parameters:
    /// - industry a reference to a Industry.
    ///
    /// __Returns__ an industry index.
    fn IndustryIndex(&self, industry: &IndustryFile) -> usize {
        for (Ix, anindustry) in self.industries.iter() {
            if anindustry == industry {return *Ix;}
        }
        0
    }
    /// Show all car movements.
    ///
    /// ## Parameters:
    /// - showAll Show all movements?
    /// - Ix Show movements by industry.
    /// - Tx Show movements by train.
    ///
    /// __Returns__ nothing.
    pub fn ShowCarMovements(&self, showAll: bool, TOption: Option<&Train>, 
                            IOption: Option<&IndustryFile>) {
        let mut Total = 0;
        let mut CarCount = 0;
        let mut trains = [String::from(""), String::from(""), 
                          String::from("")];
        for Cx in 0..self.cars.len() {
            let car: &Car = &self.cars[Cx];
            if car.Location() == IND_SCRAP_YARD ||
               car.Location() == IND_RIP_TRACK {continue;}
            if !showAll {
                if car.MovementsThisSession() == 0 {continue;}
            }
            let mut banner1 = String::from("");
            if TOption.is_some() {
                let train: &Train = TOption.unwrap();
                let Tx = self.TrainIndex(train);
                let Gx = self.switchList.LimitCars();
                // Only show moves if the car travelled in this train!
                for Gx in 0..self.switchList.LimitCars() {
                    if self.switchList.PickCarEq(Gx as isize,Cx) &&
                       self.switchList.PickTrainEq(Gx as isize,Tx) {break;}
                }
                if Gx >= self.switchList.LimitCars() {continue;}
            } else if IOption.is_some() {
                //eprintln!("*** in ShowCarMovements(): IOption is {:?}",IOption);
                let industry: &IndustryFile = IOption.unwrap();
                if banner1.len() == 0 {
                    banner1 = format!("Cars at {}",industry.Name());
                }
                let Ix = self.IndustryIndex(industry);
                if Ix != car.Location() {continue;}
            }
            if Total == 0 {
                println!("{}",self.SystemName());
                if banner1.len() > 0 {println!("{}",banner1);}
                println!("{:<21}{:<7}{:<8}{:<8}{:<8}{:<8}{}\n{:-<79}",
                        "Cars Moved","Type","Prv","1st","2nd","3rd",
                        "Destination","");
            }
            let (Status, CarTypeDesc) = self.GetCarStatus(Cx);
            let DestName = match self.industries.get(&car.Destination()) {
                Some(ind) => ind.Name(),
                Nome      => String::from("-"),
            };
            let mut carid = car.Marks();
            while carid.len() < 11 {
                carid.push(' ');
            }
            carid += &car.Number();
            while carid.len() < 21 {
                carid.push(' ');
            }
            let mut typeName = CarTypeDesc;
            let tail = typeName.split_off(5);
            while typeName.len() < 7 {
                typeName.push(' ');
            }
            let mut prev = String::from("-");
            if car.PrevTrain() > 0 {
                prev = self.trains[&car.PrevTrain()].Name();
                let tail = prev.split_off(7);
            }
            while prev.len() < 8 {
                prev.push(' ');
            } 
            let mut Count = 0;
            for Gx in 0..self.switchList.LimitCars() {
                if self.switchList.PickCarEq(Gx as isize,Cx) {
                    let Tx = self.switchList[Gx].PickTrain();
                    trains[Count] = self.trains[&Tx].Name();
                    let tail = trains[Count].split_off(7);
                    while trains[Count].len() < 8 {
                        trains[Count].push(' ');
                    }
                    Count += 1;
                    if Count == 3 {break;}
                }
            }
            while Count < 3 {
                trains[Count] = String::from("-");
                while trains[Count].len() < 8 {
                    trains[Count].push(' ');
                }
                Count += 1;
            }
            println!("{}{}{}{}{}{}{}",carid,typeName,prev,trains[0],trains[1],
                    trains[2],DestName);
            Total += 1;
            CarCount += 1;
            if Total == 18 {Total = 0;}
        } // for Cx in ...
        if CarCount > 0 {
            println!("\n                                                    Cars subtotal: {}",CarCount);
        }
    }
    /// Show cars moved by a specific train.
    ///
    /// ## Parameters:
    /// - train The specific train.
    ///
    /// __Returns__ nothing.
    pub fn ShowTrainCars(&self,train: &Train) {
        let mut Total = 0;
        let mut CarCount = 0;
        let Tx = self.TrainIndex(train);
        for Gx in 0..self.switchList.PickIndex() {
            if self.switchList.PickTrainEq(Gx as isize,Tx) {
                let Cx = self.switchList[Gx].PickCar();
                let car = &self.cars[Cx];
                if Total == 0 {
                    println!("{:<10}{:<10}{:<18}{:<29} {}",
                             train.Name(),"pickups","Car type",
                             "Status  Location","Destination");
                }
                let (status, carTypeDescr) = self.GetCarStatus(Cx);
                let LocName = match self.industries.get(&car.Location()) {
                    Some(ind) => ind.Name(),
                    None      => String::from("-"),
                };
                let DestName = if car.Location() == car.Destination() {
                                    String::from("-")
                                } else {
                                    match self.industries.get(&car.Destination()) {
                                        Some(ind) => ind.Name(),
                                        None      => String::from("-"),
                                    }
                                 };
                println!("{:<11}{:<9}{:<18}{:<8}{:<21} {}",car.Marks(),
                         car.Number(),carTypeDescr,status,LocName,DestName);
                Total += 1;
                CarCount += 1;
                if Total == 18 {Total = 0;}
            }
        }
        if CarCount == 0 {return;}
        println!("\n                                                    Cars subtotal: {}",CarCount);
    }
    /// Show cars in a specificed division.
    ///
    /// ## Parameters: 
    /// - division The specific division.
    ///
    /// __Returns__ nothing.
    pub fn ShowCarsInDivision(&self, division: u8) {
        let mut Total = 0;
        let mut CarCount = 0;
        for Cx in 0..self.cars.len() {
            let car = &self.cars[Cx];
            let iLoc = self.industries.get(&car.Location());
            if iLoc.is_none() {continue;}
            let Loc = iLoc.unwrap();
            let LocName = Loc.Name();
            let istation = &self.stations[&Loc.MyStationIndex()];
            if istation.DivisionIndex() == division {
                if Total == 0 {
                    println!("{}",self.SystemName());
                    println!("{:<18}{} {:<18}{:<29} {}",
                             "Cars In Div",self.divisions[&division].Symbol(),
                             "Car type","Status  Location","Destination");
                }
                let (status,carTypeDescr) = self.GetCarStatus(Cx);
                let DestName = if car.Location() == car.Destination() {
                        String::from("-")
                    } else {
                        match self.industries.get(&car.Destination()) {
                            Some(ind) => ind.Name(),
                            None      => String::from("-"),
                        }
                    };
                println!("{:<11}{:<9}{:<18}{:<8}{:<21} {}n", car.Marks(), 
                         car.Number(), carTypeDescr, status, LocName, 
                         DestName);
                Total += 1;
                CarCount += 1;
                if Total == 18 {Total = 0;}
            }
        }
    }
    /// Show train totals.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ nothing.
    pub fn ShowTrainTotals(&self) {
        println!("{}",self.SystemName());
        let mut line = String::from("Train");
        while line.len() < 11 {line.push(' ');}
        line += &String::from("Cars");
        while line.len() < 31 {line.push(' ');}
        line += &String::from("Train");
        while line.len() < 41 {line.push(' ');}
        line += &String::from("Cars\n");
        let mut dashes = String::new();
        while dashes.len() < 78 {dashes.push('-');}
        line += &dashes;
        println!("{}",line);

        let mut TrainCount = 0;
        for (Tx, train) in self.trains.iter() {
            if train.Shift() == self.shiftNumber {
                let mut Count = 0;
                TrainCount += 1;
                for Gx in 0..self.switchList.LimitCars() {
                    if self.switchList.PickTrainEq(Gx as isize,*Tx) {Count += 1;}
                }
                let z = TrainCount & 1;
                let mut halfLine = train.Name();
                while halfLine.len() < 11 {halfLine.push(' ');}
                halfLine += &format!("{}",Count);
                while halfLine.len() < 31 {halfLine.push(' ');}
                if z == 1 {
                    line = halfLine;
                } else {
                    line += &halfLine;
                    println!("{}",line);
                }
            }
        }
    }
    /// Show unassigned cars.
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ nothing.
    pub fn ShowUnassignedCars(&self) {
        let mut Total = 0;
        for Cx in 0..self.cars.len() {
            let car = &self.cars[Cx];
            if car.Location() == car.Destination() {
                if Total == 0 {
                    println!("{}",self.SystemName());
                    println!("{:<50}{}\n","Cars Without Assignments",
                             "Location");
                }
                let (status, carTypeDescr) = self.GetCarStatus(Cx);
                let LocName = match self.industries.get(&car.Location()) {
                    Some(val) => val.Name(),
                    None      => String::from("-"),
                };
                println!("{:<10}{:<9}{:<31}{}",car.Marks(),car.Number(),
                         carTypeDescr,LocName);
                Total += 1;
                if Total == 18 {Total = 0;}
            }
        }
    }
    /// Reload car file. 
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ nothing.
    pub fn ReLoadCarFile(&mut self) {
        self.LoadCarFile().expect("Read error");
        self.LoadStatsFile().expect("Read error");
        self.RestartLoop();
    }
    /// Reset industry statistics. 
    ///
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ nothing.
    pub fn ResetIndustryStats(&mut self, working_industries: &mut HashMap<usize, IndustryWorking>) {
        self.statsPeriod = 1;
        for Ix in working_industries.values_mut() {
            Ix.SetCarsNum(0);
            Ix.SetCarsLen(0);
            Ix.SetStatsLen(0);
        }
    }
    /// Report on all industries. 
    ///
    /// ## Parameters: 
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    pub fn ReportIndustries(&self, printer: &mut Printer) {
    }
    /// Report on all trains.
    ///
    /// ## Parameters: 
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    pub fn ReportTrains(&self, printer: &mut Printer) {
    }
    /// Report on all cars.
    ///
    /// ## Parameters: 
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    pub fn ReportCars(&self, printer: &mut Printer) {
    }
    /// Report on cars not moved.
    ///
    /// ## Parameters: 
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    pub fn ReportCarsNotMoved(&self, printer: &mut Printer) {
    }
    /// Report on car types.
    ///
    /// ## Parameters: 
    /// - rtype Type of report to produce. 
    /// - carType Car type to report on (only used when the report type is for 
    ///             a single type).
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    pub fn ReportCarTypes(&self, rtype: CarTypeReport, carType: char, 
                            printer: &mut Printer) {
    }
    /// Car location report.
    ///
    /// ## Parameters: 
    /// - cltype Type of report. 
    /// - index Index of thing to report by (industry, station, or division).
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    pub fn ReportCarLocations(&self, cltype: CarLocationType, index: usize, 
                                printer: &mut Printer) {
    }
    /// Industry analysis report. 
    ///
    /// ## Parameters: 
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    pub fn ReportAnalysis(&self, printer: &mut Printer) {
    }
    /// Report on a specified car owner. 
    ///
    /// ## Parameters: 
    /// - ownerInitials Car owner's initials to report on.
    /// - printer Printer device.
    ///
    /// __Returns__ nothing.
    pub fn ReportCarOwners(&self, ownerInitials: String, printer: &mut Printer) {
    }
}
