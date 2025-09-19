// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-09-02 15:14:28
//  Last Modified : <250919.1059>
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

use crate::system::System;

/// The IndustryType enum
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum IndustryType {
    /// Industry is a Yard.
    Yard,
    /// Industry is a modeled industry.
    Industry,
    /// Industry is an "offline" industry, one not actually modeled.
    Offline,
    /// Industry is an unknown type.
    Unknown,
}

impl Default for IndustryType {
    fn default() -> Self {
        IndustryType::Unknown
    }
}

impl IndustryType {
    /// Create a new IndustryType from the code letters in the file
    /// ## Parameters:
    /// - typeletter the type, one of `Y` (Yard), `I` (Industry) or `O`
    ///  (offline).
    ///
    /// __Returns__ The industry type.
    pub fn new(typeletter: char) -> Self {
        match typeletter {
            'Y' | 'y' => IndustryType::Yard,
            'I' | 'i' => IndustryType::Industry,
            'O' | 'o' => IndustryType::Offline,
            _ => IndustryType::Unknown
        }
    }
}

use std::fmt;
impl fmt::Display for IndustryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndustryType::Yard => write!(f, "<#IndustryType Yard (Y)>"),
            IndustryType::Industry => write!(f, "<#IndustryType Industry (I)>"),
            IndustryType::Offline => write!(f, "<#IndustryType Offline (O)>"),
            IndustryType::Unknown => write!(f, "<#IndustryType Unknown>"),
        }
    }
}

/// This is the "static" industry structure, loaded from the industry file.
#[derive(Debug, PartialEq, Default, Clone)]
pub struct IndustryFile {
    station_index: u8,
    mirror: usize,
    name: String,
    loadTypes: String,
    emptyTypes: String,
    divisionControlList: String,
    trackLen: u32,
    assignLen: u32,
    priority: u8,
    plate: u8,
    weightclass: u8,
    maxCarLen: u32,
    reload: bool,
    indtype: IndustryType,
    hazard: char,
}

// This is the working industry structure, used during processing.
pub struct IndustryWorking {
    name: String,
    station_index: u8,
    indtype: IndustryType,
    cars: Vec<usize>,
    carsNum: u32,
    carsLen: u32,
    statsLen: u32,
    usedLen: u32,
    remLen: u32,
}

impl IndustryFile {
    /// Initialize a static industry struct
    /// ## Parameters:
    /// - station_index station index.
    /// - mirror mirror industry
    /// - name industry name
    /// - loadTypes loaded car types
    /// - emptyTypes empty car types
    /// - divisionControlList division control list (APD flages for yards)
    /// - trackLen track length
    /// - assignLen assignable length
    /// - priority priority
    /// - plate max (clearance) plate allowed
    /// - weightclass max weight class allowed
    /// - maxCarLen max car length
    /// - reload industry reloads
    /// - indtype industry type char (Y, I, O)
    /// - hazard hazard class
    ///
    /// __Returns__ a freshly initialized IndustryFile struct.
    pub fn new(station_index: u8, mirror: usize, name: String,
               loadTypes: String, emptyTypes: String, 
               divisionControlList: String, trackLen: u32, assignLen: u32,
               priority: u8, plate: u8, weightclass: u8, maxCarLen: u32,
               reload: bool, indtype: char, hazard: char) -> Self {
        Self {station_index: station_index, mirror: mirror,
              name: name, loadTypes: loadTypes, emptyTypes: emptyTypes,
              divisionControlList: divisionControlList, trackLen: trackLen,
              assignLen: assignLen, priority: priority, plate: plate,
              weightclass: weightclass, maxCarLen: maxCarLen, reload: reload,
              indtype: IndustryType::new(indtype), hazard: hazard}
    }
    /// Create a standard RIP (workbench) track.
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ a freshly initialized IndustryFile struct.
    pub fn RipTrack() -> Self {
        Self {station_index: System::WORKBENCH_CITY, mirror: 0,
              name: String::from("REPAIR YARD"), loadTypes: String::from(""), 
              emptyTypes: String::from(""), 
              divisionControlList: String::from(""), trackLen: 0, 
              assignLen: 0, priority: 9, plate: 0, weightclass: 0, 
              maxCarLen: 999, reload: false, indtype: IndustryType::new('I'),
              hazard: ' '}
    }
    /// Return the Industry Type
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the Industry Type
    pub fn Type(&self) -> IndustryType {
        self.indtype
    }
    /// Station index
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the station index
    pub fn MyStationIndex(&self) -> u8 {
        self.station_index
    }
    /// Name of the industry
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the industry name
    pub fn Name(&self) -> String {
        self.name.clone()
    }
    /// Track length
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the track length
    pub fn TrackLen(&self) -> u32 {
        self.trackLen
    }
    /// Assignable length
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the assignable length
    pub fn AssignLen(&self) -> u32 {
        self.assignLen
    }
    /// Priority
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the industry's priority
    pub fn Priority(&self) -> u8 {
        self.priority
    }
    /// Reloadble?
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ true if the industry reload cars
    pub fn Reload(&self) -> bool {
        self.reload
    }
    /// Hazard
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the industry's hazard class
    pub fn Hazard(&self) -> char {
        self.hazard
    }
    /// Mirror industry
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the mirror industry or 0
    pub fn MyMirrorIndex(&self) -> usize {
        self.mirror
    }
    /// Max plate (clearance)
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the max clearance place
    pub fn MaxPlate(&self) -> u8 {
        self.plate
    }
    /// Max weight
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the max weight class
    pub fn MaxWeightClass(&self) -> u8 {
        self.weightclass
    }
    /// Division control list (string of division symbol characters)
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the division control list
    pub fn DivisionControlList(&self) -> String {
        self.divisionControlList.clone()
    }
    /// Max car length
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the max car length
    pub fn MaxCarLen(&self) -> u32 {
        self.maxCarLen
    }
    /// Loaded cars accepted as a string of car type characters
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the accepted loaded car types
    pub fn LoadsAccepted(&self) -> String {
        self.loadTypes.clone()
    }
    /// Empty car types accepted as a string of car type characters
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the list of empty car types accepted.
    pub fn EmptiesAccepted(&self) -> String {
        self.emptyTypes.clone()
    }
 }

impl IndustryWorking {
    /// Initialize a new working industry struct
    /// ## Parameters:
    /// - station_index the station index
    /// - name the name of the industry
    /// - indtype the type of the industry
    ///
    /// __Returns__ a freshly initialized IndustryWorking struct
    pub fn new(station_index: u8, name: String, indtype: IndustryType) 
            -> Self {
        Self {name: name, station_index: station_index, indtype: indtype, 
              cars: Vec::new(), carsNum: 0, carsLen: 0, statsLen: 0, 
              usedLen: 0, remLen: 0}
    }
    /// Name of the industry
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the industry name
    pub fn Name(&self) -> String {
        self.name.clone()
    }
    /// Station index
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the station index
    pub fn MyStationIndex(&self) -> u8 {
        self.station_index
    }
    /// Return the Industry Type
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the Industry Type
    pub fn Type(&self) -> IndustryType {
        self.indtype
    }
    /// Fetch the ith car
    /// ## Parameters:
    /// - i the car index to fetch
    ///
    /// __Returns__ Some(Cx) or None
    pub fn TheCar(&self, i: usize) -> Option<usize> {
        if i < self.cars.len() {
            Some(self.cars[i])
        } else {
            None
        }
    }
    /// Fetch the car count
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the number of cars at this industry
    pub fn NumberOfCars(&self) -> usize {
        self.cars.len()
    }
    /// Add a car
    /// ## Parameters:
    /// - carindex the index of the car
    ///
    /// __Returns__ nothing
    pub fn AddCar(&mut self, carindex: usize) {
        self.cars.push(carindex);
    }
    /// Remove a car from this location.  If the car is not here, do nothing.
    /// ## Parameters:
    /// - Cx the car index
    ///
    /// __Returns__ nothing
    pub fn RemoveCar(&mut self,Cx: usize) {
        for c in 0..self.cars.len() {
            if self.cars[c] == Cx {
                self.cars.remove(c);
                return;
            }
        }
    }
    /// The number of cars at this location.
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the number of cars.
    pub fn CarsNum(&self) -> u32 {self.carsNum}
    /// Set the number of cars
    /// ## Parameters:
    /// - cn the new number of cars
    ///
    /// __Returns__ nothing
    pub fn SetCarsNum(&mut self, cn: u32) {self.carsNum = cn;}
    /// Increment the cars number
    /// ## Parameter:
    /// None
    ///
    /// __Returns__ nothing
    pub fn IncrCarsNum(&mut self) { self.carsNum += 1; }
    /// Decrement the cars number
    /// ## Parameter:
    /// None
    ///
    /// __Returns__ nothing
    pub fn DecrCarsNum(&mut self) { 
        if  self.carsNum > 1 {self.carsNum -= 1; }
    }
    /// The length of all the cars at this location.
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the length of all the cars.
    pub fn CarsLen(&self) -> u32 {self.carsLen}
    /// Set the length  of all of the cars
    /// ## Parameters:
    /// - cl the new length  of all  of the cars
    ///
    /// __Returns__ nothing
    pub fn SetCarsLen(&mut self, cl: u32) {self.carsLen = cl;}    
    /// Add to the cars length
    /// ## Parameters:
    /// - cl the ammount to add to the car length
    ///
    /// __Returns__ nothing
    pub fn AddToCarsLen(&mut self, cl: u32) {self.carsLen += cl;}
    /// Subtract from the cars length
    /// ## Parameters:
    /// - cl the ammount to subtract from the car length
    ///
    /// __Returns__ nothing
    pub fn SubFromCarsLen(&mut self, cl: u32) {
        if self.carsLen < cl {
            self.carsLen = 0;
        } else {
            self.carsLen -= cl;
        }
    }
    /// The stats length at this location.
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the length of the stats.
    pub fn StatsLen(&self) -> u32 {self.statsLen}
    /// Set the stats length
    /// ## Parameters:
    /// - sl the new stats length
    ///
    /// __Returns__ nothing
    pub fn SetStatsLen(&mut self, sl: u32) {self.statsLen = sl;}
    /// Increment stats length
    /// ## Parameters:
    /// - i the ammount to add to the stats len
    ///
    /// __Returns__ nothing
    pub fn IncrementStatsLen(&mut self, i: u32) {
        self.statsLen = self.statsLen + i;
    }
    /// Increment stats length by 1
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ nothing
    pub fn IncrementStatsLen1(&mut self) {
        self.statsLen = self.statsLen + 1;
    }
    /// The used length at this location.
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the used length.
    pub fn UsedLen(&self) -> u32 {self.usedLen}
    /// Set the used length
    /// ## Parameters:
    /// - ul the new used length
    ///
    /// __Returns__ nothing
    pub fn SetUsedLen(&mut self, ul: u32) {self.usedLen = ul;}
    /// Add to the used length
    /// ## Parameters:
    /// - cl the ammount to add to the used length
    ///
    /// __Returns__ nothing
    pub fn AddToUsedLen(&mut self, cl: u32) {self.usedLen += cl;}
    /// Subtract from the used length
    /// ## Parameters:
    /// - cl the ammount to subtract from the used length
    ///
    /// __Returns__ nothing
    pub fn SubFromUsedLen(&mut self, cl: u32) {
        if self.usedLen < cl {
            self.usedLen = 0;
        } else {
            self.usedLen -= cl;
        }
    }
    /// The remaining length at this location.
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the remaining length.
    pub fn RemLen(&self) -> u32 {self.remLen}
    /// Set the remaining length
    /// ## Parameters:
    /// - rl the new remaining length
    ///
    /// __Returns__ nothing
    pub fn SetRemLen(&mut self, rl: u32) {self.remLen = rl;}
    /// Add to the remaining length
    /// ## Parameters:
    /// - rl the ammount to add to the remaining length
    ///
    /// __Returns__ nothing
    pub fn AddRemLen(&mut self, rl: u32) {self.remLen += rl;}
    /// Subtract from the remaining length
    /// ## Parameters:
    /// - rl the ammount to subtract from the remaining length
    ///
    /// __Returns__ nothing
    pub fn SubRemLen(&mut self, rl: u32) {
        if self.remLen < rl {
            self.remLen = 0;
        } else {
            self.remLen -= rl;
        }
    }
}

impl fmt::Display for IndustryFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<#IndustryFile {}>", self.name)
    }
}
impl fmt::Display for IndustryWorking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<#IndustryWorking {}>", self.name)
    }
}
