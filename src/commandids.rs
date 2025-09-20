// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-09-17 15:32:09
//  Last Modified : <250919.2037>
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

use::freight_car_forwarder::system::{CarTypeReport,CarLocationType};
use::freight_car_forwarder::train::TrainType;

/// All of the understood commands as ENum variants.  The commands that
/// take parameters are included as variants with value tuples
#[derive(Debug)]
pub enum Commands {
    Reload,
    Save,
    ShowCarsNotMoved,
    ShowCarMovements(bool,Option<usize>,Option<usize>),
    ShowTrainTotals,
    ListTrainNames(bool, Option<TrainType>),
    SetPrintYards(bool),
    SetPrintAlpha(bool),
    SetPrintAtwice(bool),
    SetPrintList(bool),
    SetPrintLtwice(bool),
    SetPrintDispatch(bool),
    SetPrintem(bool),
    RunAllTrains(String),
    RunBoxMoves(String),
    RunOneTrain(String,String),
    PrintAllLists(String),
    ShowUnassigned,
    CarAssignment,
    ReportIndustries(String),
    ReportTrains(String),
    ReportCars(String),
    ReportCarsNotMoved(String),
    ReportCarTypes(CarTypeReport,char,String),
    ReportCarLocations(CarLocationType,usize,String),
    ReportAnalysis(String),
    ReportCarOwners(String,String),
    ResetIndustries,
}
