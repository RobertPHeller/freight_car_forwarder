// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-09-02 15:14:13
//  Last Modified : <250906.0856>
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
extern crate getopts;
use getopts::Options;
use std::env;
//use std::io;
use std::io::{self, Write};


pub use freight_car_forwarder::system::System;
pub use freight_car_forwarder::fcfprintpdf::*;
//use freight_car_forwarder::switchlist::*;

/// Print command line usage.
///
/// ## Parameters:
/// - program is the name of the program.
/// - opts the program options.
///
/// __Returns__ nothing.
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}


//fn run_one_train(system: &mut System) {
//}

//fn manage_trains_and_printing(system: &mut System) {
        //println!("Run all Trains in Operating session");
        //println!("Run the Boxmove trains");
        //println!("Run Trains one at a time");
        //println!("Print yard lists, etc");
//}

//fn reports_menu(system: &System) {
//}

fn movements_by_train(system: &System) { 
}

fn movements_by_location(system: &System) {
}

fn compile_car_movements(system: &System) {
}

fn show_cars_in_division(system: &System) {
}

fn list_train_names(system: &System) {
}

fn show_car_movements(system: &System) {
    loop {
        println!("{}\n",system.SystemName());
        println!("Enter <N>     to show cars NOT moved");
        println!("Enter <M>     to show car movements");
        println!("Enter <T>     to show car movements by train");
        println!("Enter <L>     to show car movements by location");
        println!("Enter <E>     to show cars moved and NOT moved");
        println!("Enter <C>     to compile car movements");
        println!("Enter <D>     to show cars in division");
        println!("Enter <A>     to show train totals");
        //println!("Enter <U>     to mark ALL cars in use");
        println!("Enter <?>     to list train names\n");
        println!("Enter train name for a single train\n");
        println!("Enter <other> to exit\n");
        let mut command = String::new();
        print!("Your command: "); io::stdout().flush().unwrap();
        let status = match io::stdin().read_line(&mut command) {
            Ok(m) => { m },
            Err(f) => { panic!("{}", f.to_string()) },
        };
        if status == 0 {break;}
        let key = command.chars().next().unwrap_or(' ');
        match key {
            'N' | 'n' => system.ShowCarsNotMoved(),
            'M' | 'm' => system.ShowCarMovements(true, None, None),
            'T' | 't' => movements_by_train(&system),
            'L' | 'l' => movements_by_location(&system),
            'E' | 'e' => {  system.ShowCarMovements(true, None, None);
                            system.ShowCarsNotMoved(); },
            'C' | 'c' => compile_car_movements(&system),
            'D' | 'd' => show_cars_in_division(&system),
            'A' | 'a' => system.ShowTrainTotals(),
            //'U' | 'u' => system.MarkAllCarsInUse(),
            '?'         => list_train_names(&system),
            _ => match system.TrainByName(command.trim().to_string()) {
                        Some(val) => system.ShowCarMovements(false,Some(val),None),
                        None => break,
                  },
        }
    }
}

/// Main program.
///
/// ## Parameters:
/// None.
///
/// __Returns__ nothing.
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();    
    //opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    };
    //let output = matches.opt_str("o").expect("Missing option");
    let systemfile = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    
    let mut system = System::new(systemfile);
    
    loop {
        println!("{}",system.SystemName());
        println!("");
        println!("Enter <L> Load cars file {}",system.CarsFile());
        println!("Enter <S> Save cars file {}",system.CarsFile());
        //println!("Enter <G> Groups = {}", SelectGroups$);
        println!("Enter <M> Manage trains/printing");
        //println!("Enter <V> View car information");
        //println!("Enter <E> Edit car information");
        //println!("Enter <N> Add a New car");
        //println!("Enter <D> Delete an existing car");
        println!("Enter <U> Show Unassigned cars");
        println!("Enter <A> Run the car Assignment procedure");
        println!("Enter <C> Show Cars on screen");
        println!("Enter <R> Go to Reports Menu");
        println!("Enter <I> Reset Industry statistics");
        println!("Enter <Q> Quit -- exit NOW");
        let mut command = String::new();
        println!("");
        print!("Enter [LSMUACRIQ]: ");
        io::stdout().flush().unwrap();
        let status = match io::stdin().read_line(&mut command) {
            Ok(m) => { m }
            Err(f) => { panic!("{}", f.to_string()) }
        };
        if status == 0 {break;}
        let cmd = command.chars().next().unwrap_or(' ');
        match cmd {
            'L' | 'l' => system.ReLoadCarFile(),
            'S' | 's' =>
                match system.SaveCars() {
                    true => println!("Cars saved."),
                    false => println!("Cars not saved."),
                }, 
             //'M' | 'm' => manage_trains_and_printing(&mut system),
             'U' | 'u' => system.ShowUnassignedCars(),
             'A' | 'a' => system.CarAssignment(),
             'C' | 'c' => show_car_movements(&system),
             //'R' | 'r' => reports_menu(&system),
             'I' | 'i' => system.ResetIndustryStats(),
             'Q' | 'q' => break,
             _ => println!("Unreconized command character: {}",cmd),
        }
    }
}
