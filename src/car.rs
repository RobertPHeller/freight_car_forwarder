// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-09-02 15:14:54
//  Last Modified : <250919.1426>
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


/// This Struct holds all of the information for a single car.
///
/// Including its reporting marks, car number, type, division list, owner, 
/// length, weight, and so on.
///
///	
///
#[derive(Debug, Default, Clone)]
pub struct Car {
    owner: String,
    lasttrain: usize,
    prevtrain: usize,
    location: usize,
    destination: usize,
    marks: String,
    number: String,
    divisions: String,
    length: u32,
    plate: u8,
    weightclass: u8,
    ltwt: u32,
    ldlmt: u32,
    trips: u32,
    moves: u32,
    assignments: u32,
    loadedP: bool,
    mirrorP: bool,
    fixedP: bool,
    doneP: bool,
    peek: bool,
    tmpStatus: bool,
    cartype: char,
}

use std::fmt;

impl fmt::Display for Car {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.marks, self.number)
    }
}

impl Car {
    /// Initializer for a Car struct.
    /// ## Parameters:
    /// - t the type character
    /// - m railroad reporting marks or lessor/lessee
    /// - n car number or car number/number of units
    /// - d car home divisions
    /// - l extreme car length
    /// - p  clearance plate
    /// - wc car weight class
    /// - lw light weight in tons
    /// - ldw load limit in tons
    /// - lp loaded/empty status
    /// - mp ok to mirror
    /// - fp fixed route
    /// - own car owner's 3 character initals
    /// - dp car is done moving for this session
    /// - t last train to handle the car
    /// - vs actual movements this session
    /// - loc car's present location
    /// - dest car's destination
    /// - trps number of car trips
    /// - asgns number of car assignments
    ///
    /// __Returns__ a freshly initialized Car struct.
    pub fn new(t: char, m: String, n: String, d: String, l: u32, p: u8, 
               wc: u8, lw: u32, ldw: u32, lp: bool, mp: bool, fp: bool,
               own: String, dp: bool,lt: usize,mvs: u32, loc: usize,
               dest: usize, trps: u32, asgns: u32) -> Self {
        Self {owner: own, lasttrain: lt, prevtrain: lt, location: loc,
              destination: dest, marks: m, number: n, divisions: d,
              length: l, plate: p, weightclass: wc, ltwt: lw,
              ldlmt: ldw, trips: trps, moves: mvs, assignments: asgns,
              loadedP: lp, mirrorP: mp, fixedP: fp, doneP: dp, peek: false,
              tmpStatus: false, cartype: t}
    }
    /// Car type
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the type
    pub fn Type(&self) -> char {self.cartype}
    /// Set the car's type
    /// ## Parameters
    /// - t the new value for the car's type
    ///
    /// __Returns__ nothing
    pub fn SetType(&mut self,t: char) {self.cartype = t;}
    /// Car marks
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the car's marks
    pub fn Marks(&self) -> String {self.marks.clone()}
    /// Set the car's marks
    /// ## Parameters
    /// - m the new value for the car's marks
    ///
    /// __Returns__ nothing
    pub fn SetMarks(&mut self,m: String) {self.marks = m;}
    /// Car number
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the number
    pub fn Number(&self) -> String {self.number.clone()}
    /// Set the car's number
    /// ## Parameters
    /// - n the new value for the car's number
    ///
    /// __Returns__ nothing
    pub fn SetNumber(&mut self,n: String) {self.number = n;}
    /// Car divisions
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the divisions
    pub fn Divisions(&self) -> String {self.divisions.clone()}
    /// Set the car's divisions
    /// ## Parameters
    /// - d the new value for the car's division list
    ///
    /// __Returns__ nothing
    pub fn SetDivisions(&mut self,d: String) {self.divisions = d;}
    /// Car type
    /// ## Parameters:
    /// length
    ///
    /// __Returns__ the length
    pub fn Length(&self) -> u32 {self.length}
    /// Set the car's length
    /// ## Parameters
    /// - l the new value for the car's 
    ///
    /// __Returns__ nothing
    pub fn SetLength(&mut self,l: u32) {self.length = l;}
    /// Car plate
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the plate
    pub fn Plate(&self) -> u8 {self.plate}
    /// Set the car's plate
    /// ## Parameters
    /// - p the new value for the car's plate
    ///
    /// __Returns__ nothing
    pub fn SetPlate(&mut self, p: u8) {self.plate = p;}
    /// Car weight class
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the weight class
    pub fn WeightClass(&self) -> u8 {self.weightclass}
    /// Set the car's weight class
    /// ## Parameters
    /// - wc the new value for the car's weight class
    ///
    /// __Returns__ nothing
    pub fn SetWeightClass(&mut self, wc: u8) {self.weightclass = wc;}
    /// Car light weight in tons
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the light weight in tons
    pub fn LtWt(&self) -> u32 {self.ltwt}
    /// Set the car's light weight in tons
    /// ## Parameters
    /// - lw the new value for the car's light weight in tons
    ///
    /// __Returns__ nothing
    pub fn SetLtWt(&mut self, lw: u32) {self.ltwt = lw;}
    /// Car load limit in tons
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the load limit in tons
    pub fn LdLmt(&self) -> u32 {self.ldlmt}
    /// Set the car's load limit in tons
    /// ## Parameters
    /// - ldw the new value for the car's load limit in tons
    ///
    /// __Returns__ nothing
    pub fn SetLdLmt(&mut self, ldw: u32) {self.ldlmt = ldw;}
    /// Car is loaded?
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ true if car is loaded
    pub fn LoadedP(&self) -> bool {self.loadedP}
    /// Car is empty?
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ true if car is empty
    pub fn EmptyP(&self) -> bool {!self.loadedP}
    /// Load Car
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ nothing
    pub fn Load(&mut self) {self.loadedP = true;}
    /// Unload Car
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ nothing
    pub fn UnLoad(&mut self) {self.loadedP = false;}
    /// Car ok to mirror?
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ true if it is ok to mirror this car
    pub fn OkToMirrorP(&self) -> bool {self.mirrorP}
    /// Set the car's mirror status
    /// ## Parameters
    /// - m the new value for the car's mirror status
    ///
    /// __Returns__ nothing
    pub fn SetOkToMirrorP(&mut self, m: bool) {self.mirrorP = m;}
    /// Car fixed route?
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ true if this car has a fixed route
    pub fn FixedRouteP(&self) -> bool {self.fixedP}
    /// Set the car's fixed route status
    /// ## Parameters
    /// - f the new value for the car's fixed route status
    ///
    /// __Returns__ nothing
    pub fn SetFixedRouteP(&mut self, f: bool) {self.fixedP = f;}
    /// Car owner
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the owner
    pub fn Owner(&self) -> String {self.owner.clone()}
    /// Set the car's owner
    /// ## Parameters
    /// - o the new value for the car's owner
    ///
    /// __Returns__ nothing
    pub fn SetCarOwner(&mut self, o: String) {self.owner = o;}
    /// Car is done?
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ true if the car is done moving this session
    pub fn IsDoneP(&self) -> bool {self.doneP}
    /// Mark the car as done
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ nothing
    pub fn SetDone(&mut self) {self.doneP = true;}
    /// Mark the car as not done
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ nothing
    pub fn SetNotDone(&mut self) {self.doneP = false;}
    /// The last train this car was on
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the last train this car was on
    pub fn LastTrain(&self) -> usize {self.lasttrain}
    /// Set the car's peek status
    /// ## Parameters
    /// - p the new value for the car's peek status
    ///
    /// __Returns__ nothing
    pub fn SetLastTrain(&mut self, lt: usize) {self.lasttrain = lt;}
    /// The previous train this car was on 
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the previous train this car was on
    pub fn PrevTrain(&self) -> usize {self.prevtrain}
    /// Set the car's peek status
    /// ## Parameters
    /// - p the new value for the car's peek status
    ///
    /// __Returns__ nothing
    pub fn SetPrevTrain(&mut self, lt: usize) {self.prevtrain = lt;}
    /// Car movements
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the number of movements this session
    pub fn MovementsThisSession(&self) -> u32 {self.moves}
    /// Clear car movements
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ nothing
    pub fn ClearMovementsThisSession(&mut self) {self.moves = 0;}
    /// Increment car movements
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ nothing
    pub fn IncrmentMovementsThisSession(&mut self) {self.moves = self.moves + 1;}
    /// Car location
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the car's location
    pub fn Location(&self) -> usize {self.location}
    /// Set the car's peek status
    /// ## Parameters
    /// - p the new value for the car's peek status
    ///
    /// __Returns__ nothing
    pub fn SetLocation(&mut self, newloc: usize) {self.location = newloc;}
    /// Car's destination
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the car's destination
    pub fn Destination(&self) -> usize {self.destination}
    /// Set the car's peek status
    /// ## Parameters
    /// - p the new value for the car's peek status
    ///
    /// __Returns__ nothing
    pub fn SetDestination(&mut self, newdest: usize) {self.destination = newdest;}
    /// Car trip count
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the car trip count
    pub fn Trips(&self) -> u32 {self.trips}
    /// Clear car trip count
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ nothing
    pub fn ClearTrips(&mut self) {self.trips = 0;}
    /// Increment car trip count
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ nothing
    pub fn IncrementTrips(&mut self) {self.trips = self.trips + 1;}
    /// Car assignments
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the number of car assignment this session
    pub fn Assignments(&self) -> u32 {self.assignments}
    /// Set the car's assignments
    /// ## Parameters
    /// - a the new value for the car's assignments
    ///
    /// __Returns__ nothing
    pub fn SetAssignments(&mut self, a: u32) {self.assignments = a;}
    /// Clear car assignments
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ nothing
    pub fn ClearAssignments(&mut self) {self.assignments = 0;}
    /// Increment car assignments
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ nothing
    pub fn IncrementAssignments(&mut self) {self.assignments = self.assignments + 1;}
    /// Car peek flag
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the value of the car peek flag
    pub fn Peek(&self) -> bool {self.peek}
    /// Set the car's peek status
    /// ## Parameters
    /// - p the new value for the car's peek status
    ///
    /// __Returns__ nothing
    pub fn SetPeek(&mut self, p: bool) {self.peek = p;}
    /// Car temp status
    /// ## Parameters:
    /// None
    ///
    /// __Returns__ the car temp status
    pub fn TmpStatus(&self) -> bool {self.tmpStatus}
    /// Set the car's temp status
    /// ## Parameters
    /// - p the new value for the car's temp status
    ///
    /// __Returns__ nothing
    pub fn SetTmpStatus(&mut self, p: bool) {self.tmpStatus = p;}
}               
