// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-09-02 15:08:54
//  Last Modified : <250902.2103>
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

#[derive(Default, Debug, Clone)]
pub struct Division {
    name: String,
    //stations: Vec<&'system Station>,
    //home: &'system Industry,
    home_index: usize,
    symbol: char,
    area: char,
}

impl Division {
    pub fn new(name: String, home_index: usize, symbol: char, area: char) -> Self {
        Self {name: name,
              home_index: home_index,
              symbol: symbol,
              area: area,
         }
    }
    pub fn Name(&self) -> String {self.name.clone()}
    pub fn Home(&self) -> usize {self.home_index}
    pub fn Symbol(&self) -> char {self.symbol}
    pub fn Area(&self) -> char {self.area}
}

use std::fmt;
impl fmt::Display for Division {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<#Division {} (Symbol: {})>", self.name, self.symbol)
    }
}

