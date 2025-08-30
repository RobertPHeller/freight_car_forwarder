pub use crate::division::Division;
pub use crate::station::Station;
pub use crate::train::*;
pub use crate::industry::Industry;
pub use crate::owner::Owner;
pub use crate::car::Car;
pub use crate::cartype::*;
use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::io::*;
//use std::io::prelude::*;
//use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;
use std::fs;


#[derive(Debug)]
pub struct System {
    systemFile: String,
    systemName: String,
    industriesFile: String,
    trainsFile: String,
    ordersFile: String,
    ownersFile: String,
    carTypesFile: String,
    carsFile: String,
    statsFile: String,
    divisions: HashMap<u8, Division>,
    stations: HashMap<u8, Station>,
    trains: HashMap<usize, Train>,
    trainIndex: HashMap<String, usize>,
    industries: HashMap<usize, Industry>,
    carTypesOrder: Vec<char>,
    carTypes: HashMap<char, CarType>,
    carGroups: Vec<CarGroup>,
    owners: HashMap<String, Owner>,
    cars: Vec<Car>,
    //switchList: SwitchList,
    sessionNumber: u32,
    shiftNumber: u8,
    totalShifts: u32,
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
    curDivIndex: u8,
    originYardIndex: usize,
    trainLastLocationIndex: usize,
    carDestIndex: usize,
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

use std::fmt;
impl fmt::Display for System {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "<#System {}>", self.systemName)
  }
}

impl System {
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
    pub fn DivisionByIndex(&self, i: u8) -> Option<&Division> {
        self.divisions.get(&i)
    }
    pub fn FindDivisionBySymbol(&self, symbol: char) -> Option<&Division> {
        for (id, div) in self.divisions.iter() {
            if div.Symbol() == symbol {
                return Some(div);
            }
        }
        None
    }

    pub fn DivisionIter(&self) ->  Iter<'_, u8, Division> {
        self.divisions.iter()
    }    
    pub fn NumberOfStations(&self) -> usize {
        self.stations.len()
    }
    pub fn StationByIndex(&self, i: u8) -> Option<&Station> {
        self.stations.get(&i)
    }
    
    pub fn FindStationByName(&self, name: String) -> Option<&Station> {
        for (id, sta) in self.stations.iter() {
            if sta.Name() == name {
                return Some(sta);
            }
        }
        None
    }
    pub fn StationIter(&self) ->  Iter<'_, u8, Station> {
        self.stations.iter()
    }
    pub fn NumberOfTrains(&self) -> usize {
        self.trains.len()
    }
    
    pub fn TrainByIndex(&self, i: usize) -> Option<&Train> {
        self.trains.get(&i)
    }
    pub fn TrainByName(&self, name: String) -> Option<&Train> {
        let result = self.trainIndex.get(&name);
        if result == None {
            return None;
        }
        self.trains.get(result.unwrap())
    }
    pub fn TrainIter(&self) ->  Iter<'_, usize, Train> {
        self.trains.iter()
    }
    pub fn NumberOfIndustries(&self) -> usize {
        self.industries.len()
    }
    pub fn IndustryByIndex(&self, i: usize) -> Option<&Industry> {
        self.industries.get(&i)
    }
    pub fn IndustryByIndexMut(&mut self, i: usize) -> Option<&mut Industry> {
        self.industries.get_mut(&i)
    }
    pub fn FindIndustryByName(&self, name: String) -> Option<&Industry> {
        for (id, ind) in self.industries.iter() {
            if ind.Name() == name {
                return Some(ind);
            }
        }
        None
    }
    pub fn IndustryIter(&self) ->  Iter<'_, usize, Industry> {
        self.industries.iter()
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
    pub fn TotalShifts(&self) -> u32 {
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
            buffer.clear();
            let result = reader.read_line(&mut buffer);
            if result.is_err() {
                return Err(result.err().unwrap());
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
    
    fn ReadDivisions(&mut self, reader: &mut BufReader<File>) ->
            std::io::Result<u8> {
        let mut count: u8 = 0;
        let buffer = Self::SkipCommentsReadLine(reader)
                    .expect("Read Error");
        let temp = buffer.split_once("=").unwrap();
        if temp.0.trim() != "Divisions" {
            return Err(Error::new(ErrorKind::Other,"Missing Divisions = "));
        }
        let divcount: u8 = temp.1.trim()
                .parse::<u8>()
                .expect("Syntax error");
        //println!("divcount is {}",divcount);
        loop {
            let line = Self::SkipCommentsReadLine(reader)
                    .expect("Read Error");
            if line == "-1" || count >= divcount {
                break;
            }
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
            if self.industries.contains_key(&Ix) {
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
            self.industries.insert(Ix, Industry::new(IndsStation, IndsMirror, 
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
    fn StripQuotes(s: &str) -> String {
        if s.chars().next() == Some('"') {
            let l = s.len() - 1;
            let s1 = &s[1..l];
            String::from(s1)
        } else {
            String::from(s)
        }
    }
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
            if self.trains.contains_key(&Tx) {
                return Err(Error::new(ErrorKind::Other,"Duplicate train index"))
            }
            let mut tword  = String::from(items[1].trim());
            tword.make_ascii_uppercase();
            let TrnType = TrainType::new(tword.chars().next().unwrap());
            let TrnShift = items[5].trim().parse::<u8>().expect("Syntax error");
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
            let TrnMxCars = items[5].trim().parse::<usize>().expect("Syntax error"); 
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
            let TrnMxLen = items[14].trim().parse::<usize>().expect("Parse Error");
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
            self.trains.insert(Tx, train);
            self.trainIndex.insert(TrnName.clone(), Tx);
            count = count + 1;
        }
        Ok(count)
    }
    fn ReadTrainOrders(&mut self,filename: &PathBuf) -> 
            std::io::Result<usize> {
        let f = File::open(filename.to_str().unwrap())
                .expect("Cannot open orders file");
        let mut reader = BufReader::new(f);
        let mut count: usize = 0;
        loop {
            let result = Self::SkipCommentsReadLine(&mut reader);
            if result.is_err() {
                break;
            }
            let buffer = result.unwrap();
            let items: Vec<_> = buffer.split(",").collect();
            if items.len() < 2 {
                return Err(Error::new(ErrorKind::Other,"Syntax error"));
            }
            let trainname = String::from(items[0].trim());
            let trainorder = String::from(items[1].trim());
            let tx = self.trainIndex.get(&trainname).expect("Unknown train");
            self.trains.get_mut(&tx)
                .expect("Unknown train")
                .AddOrder(trainorder.clone());
            count = count + 1;
        }
        Ok(count)
    }
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
            let symbol: char = items[0].trim().chars().next().unwrap();
            let group: char = items[1].trim().chars().next().unwrap();
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
    fn DeleteAllExistingCars(&mut self) {
        self.cars.clear();
    }
    fn LoadCarFile(&mut self,filename: &PathBuf) -> std::io::Result<usize> {
        let mut count = 0;
        let f = File::open(filename.to_str().unwrap())
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
            let CrsLen: u8 = items[4].trim().parse::<u8>().expect("Syntax error");
            let CrsPlate: u8 = items[5].trim().parse::<u8>().expect("Syntax error");
            let CrsClass: u8 = items[6].trim().parse::<u8>().expect("Syntax error");
            let CrsLtWt: u8 = items[7].trim().parse::<u8>().expect("Syntax error");
            let CrsLdLmt: u8 = items[8].trim().parse::<u8>().expect("Syntax error");
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
    fn LoadStatsFile(&mut self,filename: &PathBuf) -> std::io::Result<usize> {
        let f = File::open(filename.to_str().unwrap())
                .expect("Cannot open stats file");
        let mut reader = BufReader::new(f);
        let mut line = String::new();
        let mut newformat: bool = false;
        let result = reader.read_line(&mut line);
        if result.is_err() {return result;}
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
        loop {
            Gx += 1;
            let result = reader.read_line(&mut line);
            if result.is_err() {break;}
            let Ix: usize;
            let cn: u32;
            let cl: u32;
            let sl: u32;
            if newformat {
                let vlist: Vec<_> = line.split(',').collect();
                Ix = vlist[0].parse::<usize>().expect("Syntax error");
                cn = vlist[1].parse::<u32>().expect("Syntax error");
                cl = vlist[2].parse::<u32>().expect("Syntax error");
                sl = vlist[3].parse::<u32>().expect("Syntax error");
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
            let industryOpt = self.IndustryByIndexMut(Ix);
            if industryOpt.is_none() {continue;}
            let industry = industryOpt.unwrap();
            industry.SetCarsNum(cn);
            industry.SetCarsLen(cl);
            industry.SetStatsLen(sl);
        }
        for industry  in self.industries.values_mut() {
            if self.statsPeriod == 1 {
                industry.SetCarsNum(0);
                industry.SetCarsLen(0);
                industry.SetStatsLen(0);
            }
            industry.IncrementStatsLen(industry.TrackLen());
        }
            
        Ok(Gx)
    }
    fn RestartLoop(&mut self) {
    }
    pub fn new(systemfile: String) -> Self {
        let systemfilePath: PathBuf = fs::canonicalize(systemfile)
                .expect("Path not found");
        //let systemDirectory = systemfilePath.with_file_name("");
        let f = File::open(systemfilePath.to_str().unwrap())
                .expect("Cannot open system file");
        let mut reader = BufReader::new(f);
        let systemname = Self::SkipCommentsReadLine(&mut reader).expect("Read error");
        let industriesfile: PathBuf = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
        let trainsfile: PathBuf = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
        let ordersfile: PathBuf = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
        let ownersfile: PathBuf = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
        let cartypesfile: PathBuf = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
        let carsfile: PathBuf = systemfilePath
                       .with_file_name(Self::SkipCommentsReadLine(&mut reader)
                                        .expect("Read error"));
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
              divisions: HashMap::new(), stations: HashMap::new(), 
              trains: HashMap::new(), trainIndex: HashMap::new(), 
              industries: HashMap::new(), 
              carTypesOrder: Vec::new(), carTypes: HashMap::new(), 
              carGroups: Vec::new(), owners: HashMap::new(),
              cars: Vec::new(), sessionNumber: 0, 
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
              printem: false, messageBuffer: String::from(""),
              carDestIndex: 0, curDivIndex: 0, originYardIndex: 0,
              trainLastLocationIndex: 0 };

        this.ReadDivisions(&mut reader).expect("Read error");
        this.ReadStations(&mut reader).expect("Read error");
        this.ReadIndustries(&industriesfile).expect("Read error");
        this.ReadTrains(&trainsfile).expect("Read error");
        this.ReadTrainOrders(&ordersfile).expect("Read error");
        this.ReadCarTypes(&cartypesfile).expect("Read error");
        this.ReadOwners(&ownersfile).expect("Read error");
        this.LoadCarFile(&carsfile).expect("Read error");
        this.LoadStatsFile(&statsfile).expect("Read error");
        this.RestartLoop();
        this        
    }
    
}
