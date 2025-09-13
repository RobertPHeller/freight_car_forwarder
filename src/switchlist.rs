// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-09-02 15:13:27
//  Last Modified : <250913.1135>
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
use std::fmt;
use std::ops::{Index, IndexMut};
use std::sync::Arc;
use std::collections::HashMap;
use crate::train::*;

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

trait __NewSorI<T> {
    fn new(index: T) -> Self;
}

impl __NewSorI<u8> for StationOrIndustry {
    fn new(index: u8) -> Self {
        StationOrIndustry::StationStop(index)
    }
}

impl __NewSorI<usize> for StationOrIndustry {
    fn new(index: usize) -> Self {
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
trait __NewSWElement<T> {
    fn new(pickLoc: usize, pickCar: usize, pickTrain: usize,         
                          lastTrain: usize, eletype: T) -> SwitchListElement;
}
impl __NewSWElement<u8> for SwitchListElement {
    fn new(pickLoc: usize, pickCar: usize, pickTrain: usize, 
                      lastTrain: usize, station: u8) -> Self {
        Self {pickLoc: pickLoc, pickCar: pickCar, pickTrain: pickTrain,
              lastTrain: lastTrain, 
              dropStop: StationOrIndustry::StationStop(station) }
    }
}

impl __NewSWElement<usize> for SwitchListElement {
    fn new(pickLoc: usize, pickCar: usize, pickTrain: usize, 
                      lastTrain: usize, industry: usize) -> Self {
        Self {pickLoc: pickLoc, pickCar: pickCar, pickTrain: pickTrain,
              lastTrain: lastTrain, 
              dropStop: StationOrIndustry::IndustryStop(industry) }
    }
}

impl SwitchListElement {
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
    pub fn DropStopEQ(&self, px:usize,trains: &Arc<HashMap<usize, Train>>)
             -> bool {
        if !trains.contains_key(&self.pickTrain) {return false;}
        let train = &trains[&self.pickTrain];
        let theStopOpt = train.Stop(px);
        if theStopOpt.is_none() {return false;}
        let theStop = theStopOpt.unwrap();
        match self.dropStop {
            StationOrIndustry::None => {return false;}
            StationOrIndustry::StationStop(dropStation) =>
                {return *theStop == Stop::StationStop(dropStation);},
            StationOrIndustry::IndustryStop(dropIndustry) =>
                {return *theStop == Stop::IndustryStop(dropIndustry);},
        };
        //false
    }
}

#[derive(Debug, Clone)]
pub struct SwitchList {
    theList: Vec<SwitchListElement>,
    pickIndex: usize,
    limitCars: usize,
    lastIndex: isize,
}

impl fmt::Display for SwitchList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<#SwitchList {} elements>", self.theList.len())
    }
}

impl Default for SwitchList {
    fn default() -> Self {
        Self {theList: Vec::new(), pickIndex: 0, lastIndex: -1, limitCars: 0}
    }
}

pub trait __StopType<T> {
    fn AddSwitchListElement(&mut self,pickloc: usize, pickcar: usize, 
                            picktrain: usize, lasttrain: usize, stop: T);
}

impl __StopType<u8> for SwitchList {
    fn AddSwitchListElement(&mut self,pickloc: usize, pickcar: usize, 
                                picktrain: usize, lasttrain: usize, stop: u8) {
        let newele: SwitchListElement = 
            SwitchListElement::new(pickloc, pickcar, picktrain, lasttrain,
                                   stop);
        if self.pickIndex >= self.theList.len() {
            self.theList.push(newele);
            self.pickIndex = self.theList.len();
        } else {
            self.theList[self.pickIndex] = newele;
            self.pickIndex += 1;
        }
        self.limitCars = self.pickIndex;
    }
}

impl __StopType<usize> for SwitchList {
    fn AddSwitchListElement(&mut self,pickloc: usize, pickcar: usize,
                                picktrain: usize, lasttrain: usize, 
                                stop: usize) {
        let newele: SwitchListElement = 
            SwitchListElement::new(pickloc, pickcar, picktrain, lasttrain,
                                   stop);
        if self.pickIndex >= self.theList.len() {
            self.theList.push(newele);
            self.pickIndex = self.theList.len();
        } else {
            self.theList[self.pickIndex] = newele;
            self.pickIndex += 1;
        }
        self.limitCars = self.pickIndex;
    }
}

impl SwitchList {
    pub fn new() -> Self {
        Self {theList: Vec::new(), pickIndex: 0, lastIndex: -1, limitCars: 0}
    }
    pub fn ResetSwitchList(&mut self) {
        self.pickIndex = 0;
        self.lastIndex = -1;
    }
    pub fn DiscardSwitchList(&mut self) {
        self.ResetSwitchList();
        self.limitCars = 0;
    }
    
    pub fn NextSwitchListForCarAndIndustry(&mut self, car: usize, 
                                            industry: usize) -> isize {
        let start: usize = self.lastIndex as usize + 1;
        let end: usize = self.pickIndex;
        for Gx in start..end {
            let igx: usize = Gx;
            if self.theList[igx].PickCar() == car &&
               self.theList[igx].PickLocation() == industry {
                self.lastIndex = Gx as isize;
                return self.lastIndex;
            }
        }
        self.lastIndex = -1;
        self.lastIndex
    }
    pub fn PickIndex(&self) -> usize {self.pickIndex}
    pub fn LimitCars(&self) -> usize {self.limitCars}
    pub fn ResetLastIndex(&mut self) {self.lastIndex = -1;}
    pub fn PickLocationEq(&self, Gx: isize, Ix: usize) -> bool {
        if Gx < 0 || Gx as usize >= self.pickIndex {return false;}
        else {return self.theList[Gx as usize].PickLocation() == Ix;}
    }
    pub fn PickCarEq(&self, Gx: isize, Cx: usize) -> bool {
        if Gx < 0 || Gx as usize >= self.pickIndex {return false;}
        else {return self.theList[Gx as usize].PickCar() == Cx;}
    }
    pub fn PickTrainEq(&self, Gx: isize, Tx: usize) -> bool {
        if Gx < 0 || Gx as usize >= self.pickIndex {return false;}
        else {return self.theList[Gx as usize].PickTrain() == Tx;}
    }
    
}    

impl Index<usize> for SwitchList {
    type Output = SwitchListElement;
    fn index(&self, i: usize) -> &Self::Output {
        &self.theList[i]
    }
}

impl IndexMut<usize> for SwitchList {
    //type Output = SwitchListElement;
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.theList[i]
    }
}


