// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-09-02 15:10:29
//  Last Modified : <250918.1547>
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

/// The station structure conains the name, a comment and a division index.
#[derive(Default, Debug, Clone)]
pub struct Station {
    name: String,
    comment: String,
    division_index: u8,
}

impl Station {
    /// Initialize a new station.
    /// ## Parameters:
    /// - name The name of the station.
    /// - comment A comment about the station.
    /// - division_index the division index.
    ///
    /// __Returns__ a fresh station sructure.
    pub fn new(name: String, comment: String, division_index: u8) -> Self {
        Self {name: name, comment: comment, division_index: division_index}
    }
    /// The name of the station.
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the name of the station.
    pub fn Name(&self) -> String {self.name.clone()}
    /// The comment for the station
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the comment for the station
    pub fn Comment(&self) -> String {self.comment.clone()}
    /// The division index for the station
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the station's division index.
    pub fn DivisionIndex(&self) -> u8 {self.division_index}
}

use std::fmt;
impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<#Station {}>", self.name)
    }
}
