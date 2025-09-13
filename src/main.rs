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
//  Last Modified : <250912.2021>
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
use std::path::PathBuf;
use std::ffi::OsStr;
use std::fs;
use std::collections::HashMap;
pub use freight_car_forwarder::system::System;
pub use freight_car_forwarder::industry::IndustryWorking;
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


fn run_one_train(system: &mut System, printer: &Printer) {
}

fn ask_for_filename(prompt: &str, extension: &str) -> String {
    let mut result: String = String::new();
    let os_extension = OsStr::new(extension);
    loop {
        let mut answer = String::new();
        print!("{} name (*.{})? ",prompt,extension); 
        io::stdout().flush().unwrap();
        let status = match io::stdin().read_line(&mut answer) {
            Ok(m) => { m },
            Err(f) => { eprintln!("{}", f.to_string()); 0 },
        };
        if status == 0 {break;}
        let origpath = PathBuf::from(answer.trim());
        //eprintln!("*** ask_for_filename(), origpath is {:?}", origpath);
        //eprintln!("*** ask_for_filename(), origpath.parent() is {:?}", origpath.parent());
        let parent = match origpath.parent() {
            Some(parent) => if parent.to_str().unwrap_or("") == "" {
                                PathBuf::from(".")
                            } else {
                                PathBuf::from(parent)
                            },
            None         => PathBuf::from("."),
        };
        //eprintln!("*** ask_for_filename(), parent is {:?}",parent);
        let mut path = match fs::canonicalize(parent) {
                                Ok(parentpath) => parentpath,
                                Err(f)      => {eprintln!("{}", f.to_string()); continue;},
                              };
        //eprintln!("*** ask_for_filename(), (before set_file_name) path is {:?}", path);
        match origpath.file_name() {
            Some(filename) => path.set_file_name(filename),
            None           => (),
        };
        //eprintln!("*** ask_for_filename(), (after set_file_name) path is {:?}", path);
        if path.extension().is_none() {
            path.set_extension(extension);
            match path.to_str() {
                Some(pathname) => {result = String::from(pathname);break;},
                None => (),
            };
        } else if path.extension() != Some(os_extension) {
            print!("File has wrong extension, use anyway (yN)? ");
            io::stdout().flush().unwrap();
            let status = match io::stdin().read_line(&mut answer) {
                Ok(m) => { m },
                Err(f) => { eprintln!("{}", f.to_string()); 0 },
            };
            if status == 0 {break;}
            match answer.chars().next().unwrap_or('N') {
                'Y' | 'y' => match path.to_str() {
                                Some(pathname) => 
                                        {result = String::from(pathname);break;},
                                None => (),
                              },
                 _ => (),
            };
        } else {
            match path.to_str() {
                Some(pathname) => {result = String::from(pathname);break;},
                None => (),
            };
        }
    }
    result
}

fn manage_trains_and_printing(system: &mut System, working_industries: &mut HashMap<usize, IndustryWorking>) {
    let mut printfile = String::from("");
    loop {
        println!("{}\n",system.SystemName()); 
        if printfile.len() == 0 {
            println!("Print file name is unset\n");
            println!("Enter <N> To set print filename");
        } else {
            println!("Print file name is {}\n",printfile);
        }
        println!("Enter <O> Run all Trains in Operating session");
        println!("Enter <B> Run the Boxmove trains");
        println!("Enter <T> Run Trains one at a time");
        println!("Enter <P> Print yard lists, etc");
        println!("Enter <other> To return to the main menu");
        let mut command = String::new();
        print!("Your command: "); io::stdout().flush().unwrap();
        let status = match io::stdin().read_line(&mut command) {
            Ok(m) => { m },
            Err(f) => { eprintln!("{}", f.to_string()); 0 },
        };
        if status == 0 {break;}
        let key = command.chars().next().unwrap_or(' ');
        match key {
            'N' | 'n' => { 
                        if printfile.len() != 0 {
                            loop {
                                println!("Print file name is {}\n",printfile);
                                print!("Do you really want to change it (y|N)?");
                                io::stdout().flush().unwrap();
                                let status = match io::stdin().read_line(&mut command) {
                                     Ok(m) => { m },
                                    Err(f) => { eprintln!("{}", f.to_string()); 0 }, 
                                };
                                if status == 0 {break;}
                                let key = command.chars().next().unwrap_or(' ');
                                match key {
                                    'Y' | 'y' => {
                                        printfile = ask_for_filename("Print file","pdf");
                                        break;
                                        },
                                    'N' | 'n' | '\n' => {break;}
                                    _ => {println!("Please anser Y or N");},
                                };
                            }
                        } else {
                            printfile = ask_for_filename("Print file","pdf");
                        } },
            'O' | 'o' => {
                        if printfile.len() == 0 {
                            println!("Select a PDF file to print to!");
                        } else {
                            let mut printer: Printer = Printer::new(&printfile,
                                                                "All train in operating session",
                                                                 PageSize::Letter);
                            system.RunAllTrains(working_industries,&mut printer);
                            printfile = String::new();
                        }
                      },
            'B' | 'b' => {
                        if printfile.len() == 0 {
                            println!("Select a PDF file to print to!");
                        } else {
                            let mut printer: Printer = Printer::new(&printfile,
                                                                "All boxmoves",
                                                                PageSize::Letter);
                            system.RunBoxMoves(working_industries,&mut printer);
                            printfile = String::new();
                        }
                      },
            'T' | 't' => {
                        if printfile.len() == 0 {
                            println!("Select a PDF file to print to!");
                        } else {
                            let mut printer: Printer = Printer::new(&printfile,
                                                                "One train",
                                                                PageSize::Letter);
                            run_one_train(system,&mut printer);
                            printfile = String::new();
                        }
                      },
            'P' | 'p' => {
                        if printfile.len() == 0 {
                            println!("Select a PDF file to print to!");
                        } else {
                            let mut printer: Printer = Printer::new(&printfile,
                                                                "Yard lists",
                                                                PageSize::Letter);
                            system.PrintAllLists(&mut printer);
                            printfile = String::new();
                        }
                      },
            _ => {break;},
        }
    }
}

//fn reports_menu(system: &System) {
//}

fn movements_by_train(system: &System) { 
    println!("{}",system.SystemName());
    println!("\nEnter train name to show car movements\n");
    let mut key = String::new();
    print!("Train: "); io::stdout().flush().unwrap();
    let status = match io::stdin().read_line(&mut key) {
        Ok(m) => { m },
        Err(f) => { eprintln!("{}", f.to_string()); 0 },
    };
    if status == 0 {return;}
    match system.TrainByName(key.trim().to_string()) {
        Some(val) => system.ShowCarMovements(false,Some(val),None),
        None => (),
    };
}

fn movements_by_location(system: &System) {
    println!("{}",system.SystemName());
    println!("\nEnter location code to show car movements\n");
    let mut key = String::new();
    print!("Location: "); io::stdout().flush().unwrap();
    let status = match io::stdin().read_line(&mut key) {
        Ok(m) => { m },
        Err(f) => { eprintln!("{}", f.to_string()); 0 },
    };
    if status == 0 {return;}
    match key.trim().parse::<usize>() {
        Ok(Ix) => { let IOpt = system.IndustryByIndex(Ix);
                    if IOpt.is_none() {
                        println!("No such industry: {}",Ix);
                    } else {
                        system.ShowCarMovements(true,None,IOpt);
                    };
                   },
        Err(f) => { eprintln!("{}", f.to_string());},
    };
    
}

//fn compile_car_movements(system: &System) {
//}

fn show_cars_in_division(system: &System) {
    println!("{}",system.SystemName());
    println!("\nEnter division symbol to show car movements\n");
    let mut key = String::new();
    print!("Division symbol: "); io::stdout().flush().unwrap();
    let status = match io::stdin().read_line(&mut key) {
        Ok(m) => { m },
        Err(f) => { eprintln!("{}", f.to_string()); 0 },
    };
    if status == 0 {return;}
    let divindex = system.FindDivisionIndexBySymbol(key.chars().next().unwrap_or(' '));
    match divindex {
        Some(Dx) => system.ShowCarsInDivision(Dx),
        None => (),
    };
}

fn show_car_movements(system: &System) {
    loop {
        println!("{}\n",system.SystemName());
        println!("Enter <N>     to show cars NOT moved");
        println!("Enter <M>     to show car movements");
        println!("Enter <T>     to show car movements by train");
        println!("Enter <L>     to show car movements by location");
        println!("Enter <E>     to show cars moved and NOT moved");
        //println!("Enter <C>     to compile car movements");
        println!("Enter <D>     to show cars in division");
        println!("Enter <A>     to show train totals");
        //println!("Enter <U>     to mark ALL cars in use");
        println!("Enter <?>     to list train names");
        //println!("Enter train name for a single train");
        println!("Enter <other> To return to the main menu");
        let mut command = String::new();
        print!("Your command: "); io::stdout().flush().unwrap();
        let status = match io::stdin().read_line(&mut command) {
            Ok(m) => { m },
            Err(f) => { eprintln!("{}", f.to_string()); 0 },
        };
        if status == 0 {break;}
        let key = command.chars().next().unwrap_or(' ');
        match key {
            'N' | 'n' => system.ShowCarsNotMoved(),
            'M' | 'm' => system.ShowCarMovements(false, None, None),
            'T' | 't' => movements_by_train(&system),
            'L' | 'l' => movements_by_location(&system),
            'E' | 'e' => system.ShowCarMovements(true, None, None),
            //'C' | 'c' => compile_car_movements(&system),
            'D' | 'd' => show_cars_in_division(&system),
            'A' | 'a' => system.ShowTrainTotals(),
            //'U' | 'u' => system.MarkAllCarsInUse(),
            '?'         => system.ListTrainNames(false,None),
            _ => {break;},
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
        Err(f) => { panic!("{}", f.to_string());  }
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
    
    let (mut system, mut working_industries) = System::new(systemfile);
    
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
            Err(f) => { eprintln!("{}", f.to_string()); 0 }
        };
        if status == 0 {break;}
        let cmd = command.chars().next().unwrap_or(' ');
        match cmd {
            'L' | 'l' => system.ReLoadCarFile(),
            'S' | 's' =>
                match system.SaveCars(&working_industries) {
                    true => println!("Cars saved."),
                    false => println!("Cars not saved."),
                }, 
             'M' | 'm' => manage_trains_and_printing(&mut system,&mut working_industries),
             'U' | 'u' => system.ShowUnassignedCars(),
             'A' | 'a' => system.CarAssignment(&mut working_industries),
             'C' | 'c' => show_car_movements(&system),
             //'R' | 'r' => reports_menu(&system),
             'I' | 'i' => system.ResetIndustryStats(&mut working_industries),
             'Q' | 'q' => break,
             _ => println!("Unreconized command character: {}",cmd),
        }
    }
}
