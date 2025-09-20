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
//  Last Modified : <250919.2004>
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

/// Station or Industry Enum used as the drop stop for a Switch List Element
#[derive(Debug, PartialEq, Eq, Clone, Copy)] // Add traits for comparison, copy, print
pub enum StationOrIndustry {
    /// A station stop
    StationStop(u8),
    /// An industry stop  
    IndustryStop(usize),
    /// This should not ever be used 
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

/// trait to create a StationOrIndustry enum based on the stop type
trait __NewSorI<T> {
    fn new(index: T) -> Self;
}

impl __NewSorI<u8> for StationOrIndustry {
    /// Create a station stop
    /// ## Parameters:
    /// - index the station index
    ///
    /// __Returns__ a StationOrIndustry::StationStop
    fn new(index: u8) -> Self {
        StationOrIndustry::StationStop(index)
    }
}

impl __NewSorI<usize> for StationOrIndustry {
    /// Create an industry stop
    /// ## Parameters:
    /// - index the industry index
    ///
    /// __Returns__ a StationOrIndustry::IndustryStop
    fn new(index: usize) -> Self {
        StationOrIndustry::IndustryStop(index)
    }
}

impl Default for StationOrIndustry {
    fn default() -> Self {
        StationOrIndustry::None
    }
}

/// A switch list element contains where to pick up the car, the car to pick 
/// up, the train to do the pick up, and the last train to pick up this car
/// and where to drop it off.
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
/// A trait to create a SwitchListElement based on the type of index, u8 for
/// station drops, usize for industry drops. 
trait __NewSWElement<T> {
    fn new(pickLoc: usize, pickCar: usize, pickTrain: usize,         
                          lastTrain: usize, eletype: T) -> SwitchListElement;
}
impl __NewSWElement<u8> for SwitchListElement {
    /// Create a SwitchListElement with a station drop
    /// ## Parameters:
    /// - pickLoc the pickup location
    /// - pickCar the car to pick up
    /// - pickTrain the train to pick the car up
    /// - lastTrain the last train to pick up this car
    /// - station the station to drop the car at
    ///
    /// __Returns__ a freshly initialized SwitchListElement
    fn new(pickLoc: usize, pickCar: usize, pickTrain: usize, 
                      lastTrain: usize, station: u8) -> Self {
        Self {pickLoc: pickLoc, pickCar: pickCar, pickTrain: pickTrain,
              lastTrain: lastTrain, 
              dropStop: StationOrIndustry::StationStop(station) }
    }
}

impl __NewSWElement<usize> for SwitchListElement {
    /// Create a SwitchListElement with an industry drop
    /// ## Parameters:
    /// - pickLoc the pickup location
    /// - pickCar the car to pick up
    /// - pickTrain the train to pick the car up
    /// - lastTrain the last train to pick up this car
    /// - industry the industry to drop the car at
    ///
    /// __Returns__ a freshly initialized SwitchListElement
    fn new(pickLoc: usize, pickCar: usize, pickTrain: usize, 
                      lastTrain: usize, industry: usize) -> Self {
        Self {pickLoc: pickLoc, pickCar: pickCar, pickTrain: pickTrain,
              lastTrain: lastTrain, 
              dropStop: StationOrIndustry::IndustryStop(industry) }
    }
}

impl SwitchListElement {
    /// The pickup location
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the pickup location
    pub fn PickLocation(&self) -> usize {self.pickLoc}
    /// The pickup car
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the car to pick up.
    pub fn PickCar(&self) -> usize {self.pickCar}
    /// The pickup train
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the pickup train
    pub fn PickTrain(&self) -> usize {self.pickTrain}
    /// The last train
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the last train
    pub fn LastTrain(&self) -> usize {self.lastTrain}
    /// The industry to drop the car at, if any
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ Some(industry) or None
    pub fn DropStopIndustry(&self) -> Option<usize> {
        if self.pickTrain == 0 {return None;}
        match self.dropStop {
            StationOrIndustry::StationStop(station) => None,
            StationOrIndustry::IndustryStop(industry) => Some(industry),
            StationOrIndustry::None => None,
        }
    }
    /// The station to drop the car at
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ Some(station) or None
    pub fn DropStopStation(&self) -> Option<u8> {
        if self.pickTrain == 0 {return None;}
        match self.dropStop {
            StationOrIndustry::StationStop(station) => Some(station),
            StationOrIndustry::IndustryStop(industry) => None,
            StationOrIndustry::None => None, 
        }
    }
    /// Is this a drop stop for this train?
    /// ## Parameters:
    /// - px stop number
    /// - trains the a reference copy of the trains hashmap
    ///
    /// __Returns__ true if this is a train and a stop that matches, false 
    /// otherwise  
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

/// Switch list struct.  A specialized vector of switch list elements.
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
    /// The default initialized SwitchList
    fn default() -> Self {
        Self {theList: Vec::new(), pickIndex: 0, lastIndex: -1, limitCars: 0}
    }
}

/// A trait to add switch list elements based on the type of stop
pub trait __StopType<T> {
    fn AddSwitchListElement(&mut self,pickloc: usize, pickcar: usize, 
                            picktrain: usize, lasttrain: usize, stop: T);
}

impl __StopType<u8> for SwitchList {
    /// Add a station stop switch list element
    /// ## Parameters:
    /// - pickloc pickup location
    /// - pickcar car to pick up
    /// - picktrain the train to pick up
    /// - lasttrain the last train to pick up this car
    /// - stop the station stop
    ///
    /// __Returns__ nothing
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
    /// Add a station stop switch list element
    /// ## Parameters:
    /// - pickloc pickup location
    /// - pickcar car to pick up
    /// - picktrain the train to pick up
    /// - lasttrain the last train to pick up this car
    /// - stop the industry stop
    ///
    /// __Returns__ nothing
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
    /// Initialize a SwitchList
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ a freshly initialized SwitchList
    pub fn new() -> Self {
        Self {theList: Vec::new(), pickIndex: 0, lastIndex: -1, limitCars: 0}
    }
    /// Reset the switch list
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ nothing
    pub fn ResetSwitchList(&mut self) {
        self.pickIndex = 0;
        self.lastIndex = -1;
    }
    /// Discard the switch list
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ nothing
    pub fn DiscardSwitchList(&mut self) {
        self.ResetSwitchList();
        self.limitCars = 0;
    }
    /// Find the next switch list element for the car and industry
    /// ## Parameters:
    /// - car the car index
    /// - industry the industry index
    ///
    /// __Returns__ the the switxh list element index or -1
    pub fn NextSwitchListForCarAndIndustry(&mut self, car: usize, 
                                            industry: usize) -> isize {
        let start: usize = (self.lastIndex+1) as usize;
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
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the pick index
    pub fn PickIndex(&self) -> usize {self.pickIndex}
    /// The limit on cars
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the limit on cars
    pub fn LimitCars(&self) -> usize {self.limitCars}
    /// Reset the last index
    /// None
    ///
    /// __Returns__ nothing
    pub fn ResetLastIndex(&mut self) {self.lastIndex = -1;}
    /// Is the selected SwitchListElement a pickup for the specified location
    /// ## Parameters:
    /// - Gx the switch list element index
    /// - Ix the industry index
    ///
    /// __Returns__ true if the Gx'th switch list element index is for the
    /// specified industry 
    pub fn PickLocationEq(&self, Gx: isize, Ix: usize) -> bool {
        if Gx < 0 || Gx as usize >= self.pickIndex {return false;}
        else {return self.theList[Gx as usize].PickLocation() == Ix;}
    }
    /// Is the selected SwitchListElement a pickup for the specified car
    /// ## Parameters:
    /// - Gx the switch list element index
    /// - Cx the car index
    ///
    /// __Returns__ true if the Gx'th switch list element index is for the
    /// specified car 
    pub fn PickCarEq(&self, Gx: isize, Cx: usize) -> bool {
        if Gx < 0 || Gx as usize >= self.pickIndex {return false;}
        else {return self.theList[Gx as usize].PickCar() == Cx;}
    }
    /// Is the selected SwitchListElement a pickup for the specified train
    /// ## Parameters:
    /// - Gx the switch list element index
    /// - Tx the train index
    ///
    /// __Returns__ true if the Gx'th switch list element index is for the
    /// specified train 
    pub fn PickTrainEq(&self, Gx: isize, Tx: usize) -> bool {
        if Gx < 0 || Gx as usize >= self.pickIndex {return false;}
        else {return self.theList[Gx as usize].PickTrain() == Tx;}
    }
    
}    

/// Add indexing Trait
impl Index<usize> for SwitchList {
    type Output = SwitchListElement;
    fn index(&self, i: usize) -> &Self::Output {
        &self.theList[i]
    }
}

/// Add mutable indexing Trait
impl IndexMut<usize> for SwitchList {
    //type Output = SwitchListElement;
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.theList[i]
    }
}


