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
//  Last Modified : <250918.1355>
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
//!
//! ## Main program
//!
//! Included is a terminal-based main program that implements most of the 
//! basic functionallity that the original QBasic provided and much of the
//! functionallity of the Tcl/TK & C++ version.
//!
//! ### Synopsis
//! 
//! freight_car_forwarder FILE \[options\]
//!
//! ### Options
//! - -y, --yard \[true or false\] set PrintYards flag
//! - -a, --alpha \[0, 1, or 2\] set PrintAlpha and PrintAtwice flags
//! - -l, --list \[0, 1, or 2\] set PrintList and PrintLtwice flags
//! - -d, --dispatch \[true or false\] set PrintDispatch flag
//! - -f, --flag \[true or false\] set Printem (trains) flag
//! - -b, --batch BATCHFILE run commands in a batch file
//! - -h, --help          print a help menu
//! ### Parameters
//! 
//! The system.dat file to load.  This file contains the divisions, stations,
//! and the names of the additional files, containing the remainder of the
//! layout "database".  These other files must be in the same directory as
//! system file.
//!
//! ### Description
//!
//! The main program operates in two ways: interactively via terminal I/O
//! and via a batch file containing a series of commands to run. By default
//! it runs in interactive mode, using the terminal API provided by Crossterm.
//!
//! #### Interactive menus
//! ##### Main menu
//! The main menu looks something like this:
//! ```
//! Chesapeake System
//!
//! Enter <L> Load cars file /home/heller/ChesapeakeSystem/cars.dat
//! Enter <S> Save cars file /home/heller/ChesapeakeSystem/cars.dat
//! Enter <M> Manage trains/printing
//! Enter <U> Show Unassigned cars
//! Enter <A> Run the car Assignment procedure
//! Enter <C> Show Cars on screen
//! Enter <R> Go to Reports Menu
//! Enter <I> Reset Industry statistics
//! Enter <Q> Quit -- exit NOW
//! Enter [LSMUACRIQ]: 
//! ```
//! Commands are single characters (case insensitive), and either run the 
//! selected operation or display a sub-menu.
//! ###### Commands at the main menu
//! - L Reloads the cars file from disk.
//! This resets the cars data to the previously saved version.
//! - S Saves an updates cars file.
//! This saves an updated cars data.  A backup file is saved.
//! - M Goes to the train and printing sub-menu.
//! This goes to the *Manage trains/printing* menu
//! - U Shows the unassigned cars on the screen.
//! This displays the cars that have not been assigned destinations.
//! - A Runs the car assignment procedure.
//! This runs the car assignment procedure.  Available cars (those not 
//! presently in transit) are assigned destinations.
//! - C Show the cars on the screen.
//! This enters the *Car Movement* sub-menu.
//! - R Goes to the reports sub-menu.
//! This goes to the *Reports* sub-menu.
//! - I Resets the industry statistics.
//! This resets the industry statistics.
//! - Q Quits the program.
//! This exits the program.
//!
//! ##### Manage trains/printing menu.
//! The Manage trains/printing menu looks like this:
//! ```
//! Chesapeake System
//!
//! Print file name is unset
//! Enter <N> To set print filename
//! Enter <Y> to toggle PrintYards (true)
//! Enter <A> to toggle PrintAlpha (true)
//! Enter <2> to toggle PrintAtwice (false)
//! Enter <L> to toggle PrintList (false)
//! Enter <+> to toggle PrintLtwice (false)
//! Enter <D> to toggle PrintDispatch (true)
//! Enter <F> to toggle Printem (true)
//! Enter <O> Run all Trains in Operating session
//! Enter <B> Run the Boxmove trains
//! Enter <T> Run Trains one at a time
//! Enter <P> Print yard lists, etc
//! Enter <R> To return to the main menu
//! Enter [NYA2L+DFOBTPR]: 
//! ```
//! ###### Commands at this menu are:
//! - N Ask for the print filename.  When the print file name is set, this
//! command becomes unavailable until after and operation that would generate
//! print output.
//! - Y Toggles the PrintYards flag.
//! - A Toggles the PrintAlpha flag.
//! - 2 Toggles the PrintAtwice flag.
//! - L Toggles the PrintList flag.
//! - + Toggles the PrintLtwice flag.
//! - D Toggles the PrintDispatch flag.
//! - F Toggles the Printem (train printing) flag.
//! - O Runs all trains in the operating session.
//! - B Runs the Box move trains.
//! - T Runs a single train.  
//! - P Print the various lists.
//! - R Returns to the main menu.
//! 
//! ##### Car Movement menu.
//! The Car Movement menu looks like this: 
//! ```
//! Chesapeake System 
//!
//! Enter <N>     to show cars NOT moved
//! Enter <M>     to show car movements
//! Enter <T>     to show car movements by train
//! Enter <L>     to show car movements by location
//! Enter <E>     to show cars moved and NOT moved
//! Enter <D>     to show cars in division
//! Enter <A>     to show train totals
//! Enter <?>     to list train names
//! Enter <R> To return to the main menu
//! Enter [NMTLEDA?R]: 
//! ```
//! ###### Commands at this menu are:
//! - N Shows the cars not moved.
//! - M Shows car movements.
//! - T Shows movements by train.
//! - L Shows movements by location.
//! - E Shows cars moved and not moved.
//! - D Shows cars in division.
//! - A Shows train totals.
//! - ? Lists available trains.
//! - R To return to mainmenu
//!
//! ##### Reports menu.
//! The reports menu looks like this:
//! ```
//! Chesapeake System
//!
//! Print file name is unset
//! Enter <P> To set print filename
//! Enter <I> for Industries Report
//! Enter <T> for Trains Report
//! Enter <C> for Cars Report
//! Enter <N> for Cars Not Moved Report
//! Enter <L> for Car (Load) types Report
//! Enter <O> for Car lOcations Report
//! Enter <A> for Analysis Report
//! Enter <W> for Car oWners Report
//! Enter [PITCNLOAWR]: 
//! ```
//! ###### Commands at this menu are:
//! - P Ask for the print filename.  When the print file name is set, this
//! command becomes unavailable until after and operation that would generate
//! print output.
//! - I For an Industries Report
//! - T For a Trains Report 
//! - C for a Cars Report
//! - N for a Cars Not Moved Report
//! - L for a car load types Report
//! - O for a Locations Report
//! - A For an Analysis Report 
//! - W For a Car owner's report
//! - R To return to the main menu.
//! #### Batch file
//! Specifying the `-b` or `--batch` option, will cause the program to enter
//! batch mode.  The file specified by this option is read line by line. Each
//! line is one command.  Lines starting with a `#` character are consided to
//! be comments and are igonored, as are blank lines.  When the file is 
//! exhausted, the program exits.
//!
//! The commands recognized are:
//! - `reload` Reload the cars file.
//! - `save` Save the cars file.
//! - `showcarsnotmoved` Show the cars not moved.
//! - `showcarmovements <Bool> <OptionalUsize> <OptionalUsize>` Show car 
//! movements.  If the first parameter is `true` (or `yes`) all cars are shown,
//! including ones that did not move.  The second parameter could be a train
//! number, which just shows cars moved by this train, otherwise it can be `-`
//! in which cars are shown without reguard to train.  The third parameter
//! can limit the cars chown to a specific industry or all industries if it is 
//! `-`.
//! - `showtraintotals` Show totals by train.
//! - `listtrainnames <Bool> <OptionalTrainType>` The first parameter selects 
//! all trains reguarless of shift or only list trains this shift. The second
//! selects on a specific type of train or all types if it is `-`.
//! - `setprintyards <Bool>` Sets the PrintYards flag.
//! - `setprintalpha <Bool>` Sets the PrintAlpha flag.
//! - `setprintatwice <Bool>` Sets the PrintAtwice flag.
//! - `setprintlist <Bool>` Sets the PrintList flag.
//! - `setprintltwice <Bool>` Sets the PrintLtwice flag.
//! - `setprintdispatch <Bool>` Sets the PrintDispatch flag.
//! - `setprintem <Bool>` Sets the Printem (trains) flag.
//! - `runalltrains <PdfFileName>` Run all trains in operating session.
//! - `runboxmoves <PdfFileName>` Run all boxmoves.
//! - `runonetrain <TrainName> <PdfFileName>` Run the named train.
//! - `printalllists <PdfFileName>` Print all lists.
//! - `showunassigned` Show unassined cars.
//! - `carassignment` Run the car assignment procedure.
//! - `reportindustries <PdfFileName>` Generate an industry report.
//! - `reporttrains <PdfFileName>` Generate a train report.
//! - `reportcars <PdfFileName>` Generate a car movement report.
//! - `reportcarsnotmoved <PdfFileName>` Generate a cars not moved report.
//! - `reportcartypes <CarTypeReport> <TypeChar> <PdfFileName>`
//!     Generate a Car Type report.
//! - `reportcarlocations <CarLocationType> <Index> <PdfFileName>`
//!     Generate a car location report.
//! - `reportanalysis <PdfFileName>` Generate an analysis report.
//! - `reportcarowners <Owner> <PdfFileName>` Generate a car owner report.
//! - `resetindustries` Reset industry statistics.
//!
//! Where:
//! - `<Bool>` is `true`, `false`, `yes`, or `no`.
//! - `<OptionalUsize>` is a positive integer or `-`.
//! - `<OptionalTrainType>` is `-`, `unknown`, `wayfreight`, `boxmove`, 
//! `manifest`, or `passenger`.
//! - `<PdfFileName>` is the name of a PDF filename, enclosed in double quotes 
//! `"`.
//! - `<TrainName>` is the name of a train,  enclosed in double quotes `"`.
//! - `<CarTypeReport>` is one of `all`, `type`, or `summary`.
//! - `<TypeChar>` is a car type character, enclosed in single quotes `'`.
//! - `<Index>` is a positive integer (industry, station, or division index).
//! - `<CarLocationType>` is one of `industry`, `station`, `division`, or `all`.
//! - `<Owner>` is an owner's initials, enclosed in double quotes `"`.
//! ### Notes
//! I left out the ability to Add, Remove or Edit/View cars.  This could be 
//! added with a bit of effort, mostly in implementing an editor screen using 
//! crossterm's cursor management API.
//!
//! The data files this code uses and mostly CSV text files. Most of the data
//! in these files describes the model railroad layout with a view of having
//! realistic freight car movements, with the freight cars carrying legitimate
//! cargo between the modeled industries and also with offline industries
//! (industries served by railroads that the modeled railroads would 
//! interchage with).
//!

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

