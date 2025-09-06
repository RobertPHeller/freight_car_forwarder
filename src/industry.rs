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
//  Last Modified : <250906.1110>
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

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Industry {
    cars: Vec<usize>,
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
    carsNum: u32,
    carsLen: u32,
    statsLen: u32,
    usedLen: u32,
    remLen: u32,
    reload: bool,
    indtype: char,
    hazard: char,
}

impl Industry {
    pub fn new(station_index: u8, mirror: usize, name: String,
               loadTypes: String, emptyTypes: String, 
               divisionControlList: String, trackLen: u32, assignLen: u32,
               priority: u8, plate: u8, weightclass: u8, maxCarLen: u32,
               reload: bool, indtype: char, hazard: char) -> Self {
        Self {cars: Vec::new(), station_index: station_index, mirror: mirror,
              name: name, loadTypes: loadTypes, emptyTypes: emptyTypes,
              divisionControlList: divisionControlList, trackLen: trackLen,
              assignLen: assignLen, priority: priority, plate: plate,
              weightclass: weightclass, maxCarLen: maxCarLen, carsNum: 0,
              carsLen: 0, statsLen: 0, usedLen: 0, remLen: 0, reload: reload,
              indtype: indtype, hazard: hazard}
    }
    pub fn Type(&self) -> char {
        self.indtype
    }
    pub fn MyStationIndex(&self) -> u8 {
        self.station_index
    }
    pub fn Name(&self) -> String {
        self.name.clone()
    }
    pub fn TrackLen(&self) -> u32 {
        self.trackLen
    }
    pub fn AssignLen(&self) -> u32 {
        self.assignLen
    }
    pub fn Priority(&self) -> u8 {
        self.priority
    }
    pub fn Reload(&self) -> bool {
        self.reload
    }
    pub fn Hazard(&self) -> char {
        self.hazard
    }
    pub fn MyMirrorIndex(&self) -> usize {
        self.mirror
    }
    pub fn MaxPlate(&self) -> u8 {
        self.plate
    }
    pub fn MaxWeightClass(&self) -> u8 {
        self.weightclass
    }
    pub fn DivisionControlList(&self) -> String {
        self.divisionControlList.clone()
    }
    pub fn MaxCarLen(&self) -> u32 {
        self.maxCarLen
    }
    pub fn LoadsAccepted(&self) -> String {
        self.loadTypes.clone()
    }
    pub fn EmptiesAccepted(&self) -> String {
        self.emptyTypes.clone()
    }
    pub fn TheCar(&self, i: usize) -> Option<usize> {
        if i < self.cars.len() {
            Some(self.cars[i])
        } else {
            None
        }
    }
    pub fn NumberOfCars(&self) -> usize {
        self.cars.len()
    }
    pub fn AddCar(&mut self, carindex: usize) {
        self.cars.push(carindex);
    }
    pub fn IncrementStatsLen(&mut self, i: u32) {
        self.statsLen = self.statsLen + i;
    }
    pub fn IncrementStatsLen1(&mut self) {
        self.statsLen = self.statsLen + 1;
    }
    pub fn CarsNum(&self) -> u32 {self.carsNum}
    pub fn SetCarsNum(&mut self, cn: u32) {self.carsNum = cn;}
    pub fn CarsLen(&self) -> u32 {self.carsLen}
    pub fn SetCarsLen(&mut self, cl: u32) {self.carsLen = cl;}    
    pub fn StatsLen(&self) -> u32 {self.statsLen}
    pub fn SetStatsLen(&mut self, sl: u32) {self.statsLen = sl;}
    pub fn UsedLen(&self) -> u32 {self.usedLen}
    pub fn SetUsedLen(&mut self, ul: u32) {self.usedLen = ul;}
    pub fn AddToUsedLen(&mut self, cl: u32) {self.usedLen += cl;}
    pub fn RemoveCar(&mut self,Cx: usize) {
        for c in 0..self.cars.len() {
            if self.cars[c] == Cx {
                self.cars.remove(c);
                return;
            }
        }
    }
}

use std::fmt;
impl fmt::Display for Industry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<#Industry {}>", self.name)
    }
}
