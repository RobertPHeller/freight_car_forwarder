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
//  Last Modified : <250917.1232>
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

/// This struct contains information about a division
/// A division has a name, a home yard, a symbol character band an
/// area character

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
    /// Initialize a new division.
    /// ## Parameters:
    /// - name The name of the division
    /// - home_index The home yard index.
    /// - symbol The division's symbol
    /// - area The division's area
    ///
    /// __Returns__ an initialized division
    pub fn new(name: String, home_index: usize, symbol: char, area: char) -> Self {
        Self {name: name,
              home_index: home_index,
              symbol: symbol,
              area: area,
         }
    }
    /// Return the division's name.
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the name.
    pub fn Name(&self) -> String {self.name.clone()}
    /// Return the home yard index.
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the home yard index.
    pub fn Home(&self) -> usize {self.home_index}
    /// Return the symbol
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the symbol.
    pub fn Symbol(&self) -> char {self.symbol}
    /// Return the area
    /// ## Parameters:
    /// None.
    ///
    /// __Returns__ the area symbol.
    pub fn Area(&self) -> char {self.area}
}

use std::fmt;
impl fmt::Display for Division {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<#Division {} (Symbol: {})>", self.name, self.symbol)
    }
}

