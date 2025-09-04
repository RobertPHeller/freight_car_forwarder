// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-09-02 15:13:07
//  Last Modified : <250902.2107>
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

pub const NUMBER_OF_CARTYPES: u8 = 91;
pub const MAX_CAR_TYPES: u8 = 128;

#[derive(Debug, Default, Clone)]
pub struct CarType {
    comment: String,
    type_name: String,
    group: char,
}

impl CarType {
    pub fn new(comment: String, type_name: String, group: char) -> Self {
        Self {comment: comment, type_name: type_name, group: group }
    }
    pub fn Comment(&self) -> String {self.comment.clone()}
    pub fn Type(&self) -> String {self.type_name.clone()}
    pub fn Group(&self) -> char {self.group}
}

impl fmt::Display for CarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.type_name)
    }
}

pub const MAX_CAR_GROUP: u8 = 16;


#[derive(Debug, Default, Clone)]
pub struct CarGroup {
    description: String,
    group: char,
}

impl CarGroup {
    pub fn new(g: char, d: String) -> Self {
        Self {group: g, description: d}
    }
    pub fn Group(&self) -> char {self.group}
    pub fn Description(&self) -> String {self.description.clone()}
}

impl fmt::Display for CarGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<#CarGroup {} {}>", self.group, self.description)
    }
}
