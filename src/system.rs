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
    divisions: HashMap<u8, Division>,
    stations: HashMap<u8, Station>,
    trains: HashMap<usize, Train>,
    trainIndex: HashMap<String, usize>,
    industries: HashMap<usize, Industry<'system>>,
    carTypesOrder: Vec<char>,
    carTypes: HashMap<char, CarType>,
    carGroups: Vec<CarGroup>,
    owners: HashMap<String, Owner>,
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

use std::fmt;
impl fmt::Display for System<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "<#System {}>", self.systemName)
  }
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
            buffer.clear();
            reader.read_line(&mut buffer)?;
            buffer = buffer.trim().to_string();
            //println!("In SkipCommentsReadLine: buffer is {}",buffer);
            if buffer.len() > 0 && !buffer.starts_with("'") {
                break;
            }
        }
        //println!("Returning from SkipCommentsReadLine");
        Ok(buffer)
    }
    
    fn ReadDivisions(reader: &mut BufReader<File>,
                     divisions: &mut  HashMap<u8, Division>) ->
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
             divisions.insert(div_index,div);
             count = count + 1;
        }
        Ok(count)
    }

    fn ReadStations(reader: &mut BufReader<File>,
                    stations: &mut  HashMap<u8, Station>) ->
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
            stations.insert(sta_index,sta);
            count = count + 1;
        }
        Ok(count)
    }
    fn ReadIndustries(filename: &PathBuf,
                      industries: &mut  HashMap<usize, Industry>,
                      stations: &HashMap<u8, Station>) ->
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
                if !stations.contains_key(&IndsStation) {
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
            industries.insert(Ix, Industry::new(IndsStation, IndsMirror, 
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
    fn ReadTrains(filename: &PathBuf, trains: &mut HashMap<usize, Train>,
                  trainindex: &mut HashMap<String, usize>, 
                  industries: &HashMap<usize, Industry>,
                  stations: &HashMap<u8, Station>) -> std::io::Result<usize> {
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
            if trains.contains_key(&Tx) {
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
            trains.insert(Tx, train);
            trainindex.insert(TrnName.clone(), Tx);
            count = count + 1;
        }
        Ok(count)
    }
    fn ReadTrainOrders(filename: &PathBuf,
                       trains: &mut HashMap<usize, Train>,
                       trainindex: &HashMap<String, usize>) -> 
            std::io::Result<usize> {
        let f = File::open(filename.to_str().unwrap())
                .expect("Cannot open trains file");
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
            let tx = trainindex.get(&trainname).expect("Unknown train");
            trains.get_mut(&tx)
                .expect("Unknown train")
                .AddOrder(trainorder.clone());
            count = count + 1;
        }
        Ok(count)
    }
    fn ReadCarTypes(filename: &PathBuf,
                    cartypesorder: &mut Vec<char>,
                    cartypes: &mut HashMap<char, CarType>,
                    cargroups: &mut Vec<CarGroup>) ->  std::io::Result<()> {
        let f = File::open(filename.to_str().unwrap())
                .expect("Cannot open trains file");
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
            cartypesorder.push(symbol);
            cartypes.insert(symbol, CarType::new(comment,type_name,group));
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
            cargroups.push(CarGroup::new(symbol,comment));
        }
        Ok(())
    }
    fn ReadOwners(filename: &PathBuf, owners: &mut HashMap<String, Owner>) ->  
        std::io::Result<usize> {
        let mut count = 0;
        Ok(count)
    }
    fn LoadCarFile(filename: &PathBuf, cars: &mut Vec<Car>) ->
        std::io::Result<usize> {
        let mut count = 0;
        Ok(count)
    }
    //fn LoadStatsFile(filename: &PathBuf,
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
        let mut divisions: HashMap<u8, Division> = HashMap::new();
        Self::ReadDivisions(&mut reader,&mut divisions)
                                        .expect("Read error");
        let mut stations: HashMap<u8, Station> = HashMap::new();
        Self::ReadStations(&mut reader,&mut stations).expect("Read error");
        let mut industries: HashMap<usize, Industry> = HashMap::new();
        Self::ReadIndustries(&industriesfile,&mut industries,&stations)
                .expect("Read error");
        let mut trains: HashMap<usize, Train> = HashMap::new();
        let mut trainindex: HashMap<String, usize> = HashMap::new();
        Self::ReadTrains(&trainsfile, &mut trains, &mut trainindex,
                         &industries, &stations).expect("Read error");
        Self::ReadTrainOrders(&ordersfile, &mut trains, &trainindex)
                .expect("Read error");
        let mut cartypesorder: Vec<char> = Vec::new();
        let mut cartypes: HashMap<char, CarType> = HashMap::new();
        let mut cargroups: Vec<CarGroup> = Vec::new();
        Self::ReadCarTypes(&cartypesfile, &mut cartypesorder, &mut cartypes,
                            &mut cargroups)
                .expect("Read error");
        let mut owners: HashMap<String, Owner> = HashMap::new();
        Self::ReadOwners(&ownersfile, &mut owners)
                .expect("Read error");
        let mut cars: Vec<Car> = Vec::new();
        Self::LoadCarFile(&carsfile, &mut cars).expect("Read error");
        //Self::LoadStatsFile(&statsfile, &mut 
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
              carTypesOrder: cartypesorder, carTypes: cartypes, 
              carGroups: cargroups, owners: owners,
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
