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
//  Last Modified : <250915.2216>
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

pub mod menu;
use crate::menu::*;

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

fn manage_trains_and_printing<W>(system: &mut System,stdout: &mut W,working_industries: &mut HashMap<usize, IndustryWorking>) -> io::Result<()>
where
     W: io::Write,
{
    let mut printfile = String::from("");
    loop {
        let a = if printfile.len() == 0 {
            String::from("Print file name is unset\nEnter <N> To set print filename\n")
        } else {
            format!("Print file name is {}\n",printfile)
        };
        let b = if printfile.len() == 0 {
            String::from("Enter [NYA2L+DFOBTPR]: ")
        } else {
            String::from("Enter [YA2L+DFOBTPR]: ")
        };
        let c = format!("Enter <Y> to toggle PrintYards ({})\n",
                        system.PrintYards());
        let d = format!("Enter <A> to toggle PrintAlpha ({})\n",
                        system.PrintAlpha());
        let e = format!("Enter <2> to toggle PrintAtwice ({})\n",
                        system.PrintAtwice());
        let f = format!("Enter <L> to toggle PrintList ({})\n",
                        system.PrintList());
        let g = format!("Enter <+> to toggle PrintLtwice ({})\n",
                        system.PrintLtwice());
        let h = format!("Enter <D> to toggle PrintDispatch ({})\n",
                        system.PrintDispatch());
        let i = format!("Enter <F> to toggle Printem ({})\n",
                        system.Printem());
        let TrainsAndPrintingMenu: &str = &(system.SystemName() + "\n" + "\n" +
            &a + &c + &d + &e + &f + &g + &h + &i +
            "Enter <O> Run all Trains in Operating session\n" +
            "Enter <B> Run the Boxmove trains\n" +
            "Enter <T> Run Trains one at a time\n" +
            "Enter <P> Print yard lists, etc\n" +
            "Enter <R> To return to the main menu\n" +
            &b);
        let allowed: &[char] = if printfile.len() == 0 {
            &['N','n','Y','y','A','a','2','L','l','+','D','d','F','f','O','o',
              'B','b','T','t','P','p','R','r']
        } else {
            &['Y','y','A','a','2','L','l','+','D','d','F','f','O','o','B','b',
              'T','t','P','p','R','r',]
        }; 
        let key = menu(stdout,TrainsAndPrintingMenu,allowed)?;
        let mut needwait = true;
        match key {
            'N' | 'n' => { 
                         printfile = ask_for_filename("Print file","pdf");
                         needwait = false;
                         },
            'Y' | 'y' => {
                        system.SetPrintYards(!system.PrintYards());
                        needwait = false;
                        }
            'A' | 'a' => {
                        system.SetPrintAlpha(!system.PrintAlpha());
                        needwait = false;
                        }
            '2'       => {
                        system.SetPrintAtwice(!system.PrintAtwice());
                        needwait = false;
                        },
            'L' | 'l' => {
                        system.SetPrintList(!system.PrintList());
                        needwait = false;
                        },
            '+'       => {
                        system.SetPrintLtwice(!system.PrintLtwice());
                        needwait = false;
                        },
            'D' | 'd' => {
                        system.SetPrintDispatch(!system.PrintDispatch());
                        needwait = false;
                        },
            'F' | 'f' => {
                        system.SetPrintem(!system.Printem());
                        needwait = false;
                        },
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
            'R' | 'r' => {break;}
            _ => {panic!("Should never get here");},
        }
        if needwait {wait_any_key(stdout,"Hit any key to continue")?;}
    }
    Ok(())
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

fn show_car_movements<W>(system: &System,stdout: &mut W) -> io::Result<()> 
where
    W: io::Write,
{
    let CarMoveMenu: &str = &(system.SystemName() + "\n" + "\n" +
        "Enter <N>     to show cars NOT moved\n" +
        "Enter <M>     to show car movements\n" +
        "Enter <T>     to show car movements by train\n" +
        "Enter <L>     to show car movements by location\n" +
        "Enter <E>     to show cars moved and NOT moved\n" +
        //"Enter <C>     to compile car movements\n" +
        "Enter <D>     to show cars in division\n" +
        "Enter <A>     to show train totals\n" +
        //"Enter <U>     to mark ALL cars in use\n" +
        "Enter <?>     to list train names\n" +
        //"Enter train name for a single train\n" +
        "Enter <R> To return to the main menu\n" +
        "Enter [NMTLEDA?R]: ");

    loop {
        let key = menu(stdout,CarMoveMenu,&['N','n','M','m','T','t',
                                            'L','l','E','e','D','d',
                                            'A','a','?','R','r'])?;
        let mut needwait = true;
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
            'R' | 'r'  => {break;},
            _ => panic!("Should never get here"),
        }
        if needwait {wait_any_key(stdout,"Hit any key to continue")?;}
    }
    Ok(())
}

/// Main program.
///
/// ## Parameters:
/// None.
///
/// __Returns__ nothing.
fn main() -> io::Result<()> {
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
        return Ok(());
    };
    //let output = matches.opt_str("o").expect("Missing option");
    let systemfile = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        panic!("Missing system file!");
    };
    
    let (mut system, mut working_industries) = System::new(systemfile);
    
    let MainMenu: &str = &(system.SystemName() + "\n" +
        "\n" +
        "Enter <L> Load cars file " + &system.CarsFile() + "\n" +
        "Enter <S> Save cars file " + &system.CarsFile() + "\n" +
        "Enter <M> Manage trains/printing\n" +
        //"Enter <V> View car information\n" +
        //"Enter <E> Edit car information\n" +
        //"Enter <N> Add a New car\n" +
        //"Enter <D> Delete an existing car\n" +
        "Enter <U> Show Unassigned cars\n" +
        "Enter <A> Run the car Assignment procedure\n" +
        "Enter <C> Show Cars on screen\n" +
        "Enter <R> Go to Reports Menu\n" +
        "Enter <I> Reset Industry statistics\n" +
        "Enter <Q> Quit -- exit NOW\n" +
        "Enter [LSMUACRIQ]: ");

    let mut stdout = io::stdout();
    loop {
        let mut needwait = true;
        let cmd = menu(&mut stdout,MainMenu,&['L','l','S','s','M','m','U','u',
                                              'A','a','C','c','R','r','I','i',
                                              'Q','q'])?;
        match cmd {
            'L' | 'l' => working_industries = system.ReLoadCarFile(),
            'S' | 's' =>
                match system.SaveCars(&working_industries) {
                    true => println!("Cars saved."),
                    false => println!("Cars not saved."),
                }, 
             'M' | 'm' => {manage_trains_and_printing(&mut system,&mut stdout,&mut working_industries)?;
                            needwait = false;},
             'U' | 'u' => system.ShowUnassignedCars(),
             'A' | 'a' => system.CarAssignment(&mut working_industries),
             'C' | 'c' => {show_car_movements(&system,&mut stdout)?;
                            needwait = false;},
             //'R' | 'r' => reports_menu(&system),
             'I' | 'i' => system.ResetIndustryStats(&mut working_industries),
             'Q' | 'q' => break,
             _ => panic!("Unreconized command character: {}",cmd),
        }
        if needwait {wait_any_key(&mut stdout,"Hit any key to continue")?;}
    }
    Ok(())
}
