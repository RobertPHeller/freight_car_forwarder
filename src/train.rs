// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-09-02 15:10:46
//  Last Modified : <250919.1527>
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

use std::vec::*;


/// Train type -- the type of the train
#[derive(Debug, PartialEq, Eq, Clone, Copy)] // Add traits for comparison, copy, print
pub enum TrainType {
    /// Unknown type
    Unknown,
    /// Wayfreights are scheduled trains that move cars between industries and
    /// yards
    Wayfreight,
    /// Box moves are unscheduled train that move cars between industries and
    /// yards 
    BoxMove,
    /// Manifest trains are scheduled trains that move cars between yards
    Manifest,
    /// Passenger trains don't handle freight cars.
    Passenger
}

impl Default for TrainType {
    fn default() -> Self {
        TrainType::Unknown
    }
}

impl TrainType {
    /// Create a train type Enum from a code letter
    /// ## Parameters:
    /// - code the code character in the trains,dat file
    ///
    /// __Returns__ a TrainType Enum
    pub fn new(code: char) -> Self {
        match char {
            'W' | 'w' => TrainType::Wayfreight,
            'B' | 'b' => TrainType::BoxMove,
            'M' | 'm' => TrainType::Manifest,
            'P' | 'p' => TrainType::Passenger,
            _         => TrainType::Unknown,
        }
    }
}

use std::fmt;                                                                   
impl fmt::Display for TrainType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrainType::Wayfreight => write!(f, "<#TrainType Wayfreight (W)>"),
            TrainType::BoxMove  => write!(f, "<#TrainType BoxMove (B)>"),
            TrainType::Manifest => write!(f, "<#TrainType Manifest (M)>"),
            TrainType::Passenger => write!(f, "<#TrainType Passenger (P)>"),
            TrainType::Unknown => write!(f, "<#TrainType Unknown>"),
        }
    }
}

/// Train stop enum -- contains a train's stop, at either a station or an
/// industry
#[derive(Debug, PartialEq, Eq, Clone, Copy)] // Add traits for comparison, copy, print
pub enum Stop {
    /// Stops at a station
    StationStop(u8),
    /// Stops at an industry
    IndustryStop(usize),
}

impl Stop {
    /// Create a new station stop
    /// ## Parameters:
    /// - index The station index
    ///
    /// __Returns__ a fresh StationStop
    pub fn newStationStop(index: u8) -> Self {
        Stop::StationStop(index)
    }
    /// Create a new industry stop
    /// ## Parameters:
    /// - index The industry index
    ///
    /// __Returns__ a fresh IndustryStop
    pub fn newIndustryStop(index: usize) -> Self {
        Stop::IndustryStop(index)
    }

}

impl fmt::Display for Stop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stop::StationStop(station) => 
                write!(f, "<#StationStop ({})>", station),
            Stop::IndustryStop(industry) =>
                write!(f, "<#IndustryStop ({})>", industry),
        }
    }
}

/// Train structure -- holds information about a train.
#[derive(Debug, PartialEq, Default, Clone)] 
pub struct Train {
    orders: Vec<String>,
    stops: Vec<Stop>,
    name: String,
    divList: String,
    carTypes: String,
    description: String,
    shift: u8,
    maxcars: u32,
    maxclear: u8,
    maxweight: u8,
    maxlength: u32,
    onduty: u32,
    print: bool,
    done: bool,
    traintype: TrainType,
    number: usize,
}

impl Train {
    /// Initialize a new train struct.
    /// ## Parameters:
    /// - name Train call letters
    /// - divList Division forwarding list
    /// - carTypes Car types accepted in train
    /// - description Text description
    /// - shift Train shift number
    /// - maxcars Maximum numbers of cars at once
    /// - maxclear Maximum clearance plate
    /// - maxweight Maximum weight class 
    /// - maxlength Maximum train length
    /// - onduty Scheduled Time to Start in minutes
    /// - print Print train orders
    /// - done Done indicator
    /// - traintype Train type Manifest or Local or Boxmove
    /// - number Numeric identifier
    ///
    /// __Returns__ a freshly initialized train struct.
    pub fn new(name: String, divList: String,carTypes: String,
               description: String, shift: u8, maxcars: u32, maxclear: u8, 
               maxweight: u8, maxlength: u32, onduty: u32, print: bool, 
               done: bool, traintype: TrainType, number: usize) -> Self {
        Self {orders: Vec::new(), stops: Vec::new(), name: name, 
              divList: divList, carTypes: carTypes, description: description, 
              shift: shift, maxcars: maxcars, maxclear: maxclear, 
              maxweight: maxweight, maxlength: maxlength, onduty: onduty, 
              print: print, done: done, traintype: traintype, number: number }
    }
    /// Train name
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the train name
    pub fn Name(&self) -> String {
        self.name.clone()
    }
    /// Train division list
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the train division list 
    pub fn DivisionList(&self) -> String {
        self.divList.clone()
    }
    /// Train car types
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the train car types
    pub fn CarTypes(&self) -> String {
        self.carTypes.clone()
    }
    /// Train description
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the train description
    pub fn Description(&self) -> String {
        self.description.clone()
    }
    /// Train shift
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the train shift
    pub fn Shift(&self) -> u8 {
        self.shift
    }
    /// Set the train shift
    /// ## Parameters:
    /// - newshift the new shift
    ///
    /// __Returns__ nothing
    pub fn SetShift(&mut self, newshift: u8) {
        self.shift = newshift;
    }
    /// Train maximum number of cars
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the train maximum number of cars
    pub fn MaxCars(&self) -> u32 {
        self.maxcars
    }
    /// Train maximum clearance plate
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the train maximum clearance plate
    pub fn MaxClear(&self) -> u8 {
        self.maxclear
    }
    /// Train maximum weight
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the train maximum weight
    pub fn MaxWeight(&self) -> u8 {
        self.maxweight
    }
    /// Set the train maximum weight
    /// ## Parameters:
    /// - newmaxweight the new maximum weight
    ///
    /// __Returns__ nothing
    pub fn SetMaxWeight(&mut self, newmaxweight: u8) {
        self.maxweight = newmaxweight
    }
    /// Train maximum length
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the train maximum length 
    pub fn MaxLength(&self) -> u32{
        self.maxlength
    }
    /// Set the train maximum length
    /// ## Parameters:
    /// - newmaxlength the new maximum length
    ///
    /// __Returns__ nothing
    pub fn SetMaxLength(&mut self, newmaxlength: u32) {
        self.maxlength = newmaxlength;
    }
    /// Train on duty time in minutes
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the train on duty time in minutes 
    pub fn OnDuty(&self) -> u32 {
        self.onduty
    }
    /// Train print flag
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the train print flag 
    pub fn Print(&self) -> bool {
        self.print
    }
    /// Set the train print flag
    /// ## Parameters:
    /// - flag the new flag
    ///
    /// __Returns__ nothing
    pub fn SetPrint(&mut self, flag: bool) {
        self.print = flag;
    }
    /// Train done flag
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the done flag
    pub fn Done(&self) -> bool {
        self.done
    }
    /// Set the train done flag
    /// ## Parameters:
    /// - flag the new flag
    ///
    /// __Returns__ nothing
    pub fn SetDone(&mut self, flag: bool) {
        self.done = flag;
    }
    /// Train type
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the train type
    pub fn Type(&self) -> TrainType {
        self.traintype
    }
    /// Number of train orders
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the number of train orders
    pub fn NumberOfOrders(&self) -> usize {
        self.orders.len()
    }
    /// Get the train's ith order
    /// ## Parameters:
    /// - i the order's index
    ///
    /// __Returns__ the ith order or None
    pub fn Order(&self, i: usize) -> Option<String> {
        if i < self.orders.len() {
            Some(self.orders[i].clone())
        } else {
            None
        }
    }
    /// For iterating over the train's orders
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ an iterator into the orders vector.
    pub fn OrdersIter(&self) -> impl Iterator<Item = &String> {
        self.orders.iter()
    }
    /// Adds an order to the trains list of orders
    /// ## Parameters:
    /// order -- the additional order
    ///
    /// __Returns__ nothing
    pub fn AddOrder(&mut self, order: String) {
        self.orders.push(order);
    }
    /// Number of stops this train makes
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the number of stops this train makes
    pub fn NumberOfStops(&self) -> usize {
        self.stops.len()
    }
    /// The train's ith stop
    /// ## Parameters:
    /// - i the stop index
    ///
    /// __Returns__ the ith stop or none
    pub fn Stop(&self, i: usize) -> Option<&Stop> {
        if i < self.stops.len() {
            Some(&self.stops[i])
        } else {
            None
        }
    }
    /// Add a stop
    /// ## Parameters:
    /// - stop the stop to add
    ///
    /// __Returns__ nothing
    pub fn AddStop(&mut self, stop: &Stop) {
        self.stops.push(stop.clone());
    }
    /// Train number
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the train number
    pub fn Number(&self) -> usize {self.number}
}

impl fmt::Display for Train {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<#Train {} (Type: {}, {} stops)>", self.name, self.Type(), 
                self.NumberOfStops())
    }
}
