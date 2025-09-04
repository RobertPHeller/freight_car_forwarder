// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-09-02 15:07:03
//  Last Modified : <250902.2250>
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

//! This is a port of the C++ port of Timothy O'Connor's Freight Car 
//! Forwarding system.
//!
//! The C++ port is part of the Deepwoods Software Model Railroad System,
//! [https://www.deepsoft.com/home/products/modelrailroadsystem/](https://www.deepsoft.com/home/products/modelrailroadsystem/).
//!
//! Author: Robert Heller <heller@deepsoft.com>
//!
//! Github: [https://github.com/RobertPHeller/freight_car_forwarder](https://github.com/RobertPHeller/freight_car_forwarder)




/// The station module implements stations along the railroad.
///
/// Trains travel between stations, moving cars between stations.  There
/// are industries and yards associated with each station.
///
pub mod station;
/// The division module implements divisions, which are separate areas.
///
/// The railroad is divided into divisions, which represent specific 
/// geographical areas.  Each area has a set of stations and each area has
/// a home yard.
pub mod division;
/// The train module implements trains.
///
/// Trains are used to move cars around the railroad.
pub mod train;
/// The industry module implement industries around the layout.
///
/// There are several types of industries: 
/// - Online industries, which are industrial spurs or sidings on the layout.
/// - Offline industries, which are industries that are not on the layout.
/// - Yards, which are rail yards, where cars are stored and sorted.
/// - Stage, which are (hidden) staging yards.
pub mod industry;
/// The cartype module implement the car type information.
///
///  There are 91 car types, represented by single printable ASCII characters.
///
pub mod cartype;
/// The owner module implement car owners.
pub mod owner;
/// The car module implement rail cars.
///
/// Each car has a type, reporting marks & number, a home division.
/// a length. a clearance plate. a weight class, empty weight, loaded
/// weight, a location, a destination, along with other information.
pub mod car;
/// The switchlist module implement the switch list.
///
/// Switch lists are used by train crews and yardmasters to switch cars and 
/// make up trains.
pub mod switchlist;
/// The fcfprintpdf module implements the PDF output files for printed switch 
/// lists and reports.
pub mod fcfprintpdf;
/// The system module is the master module, containing the system Struct.
///
/// The System struct contains all of the information for the operating
/// session.
pub mod system;

