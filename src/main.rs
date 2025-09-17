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
//  Last Modified : <250916.2144>
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
use std::io::{self, Write, Error, ErrorKind};
use std::path::PathBuf;
use std::ffi::OsStr;
use std::fs;
use std::collections::HashMap;
pub use freight_car_forwarder::system::*;
pub use freight_car_forwarder::industry::IndustryWorking;
//pub use freight_car_forwarder::fcfprintpdf::*;
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


fn run_one_train(system: &mut System, 
                 working_industries: &mut HashMap<usize, IndustryWorking>,
                 printer: &mut Printer)  -> io::Result<()> {
    println!("{}",system.SystemName());
    println!("\nEnter train name to run: \n");
    let mut key = String::new();
    print!("Train: "); io::stdout().flush()?;
    let status = io::stdin().read_line(&mut key)?;
    if status == 0 {return Err(Error::new(ErrorKind::UnexpectedEof,"End of file"));}
    let trainName = key.trim();
    match system.TrainByName(trainName.to_string()) {
        Some(train) => system.RunOneTrain(train.Number(),false,
                                          working_industries,printer),
        None => println!("No such train: {}",trainName),
    };
    Ok(())
}

fn ask_for_filename(prompt: &str, extension: &str) -> io::Result<String> {
    let result: String; // = String::new();
    let os_extension = OsStr::new(extension);
    loop {
        let mut answer = String::new();
        print!("{} name (*.{})? ",prompt,extension); 
        io::stdout().flush()?;
        let status = io::stdin().read_line(&mut answer)?;
        if status == 0 {return Err(Error::new(ErrorKind::UnexpectedEof,"End of file"));}
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
        let mut path = fs::canonicalize(parent)?;
        //eprintln!("*** ask_for_filename(), (before set_file_name) path is {:?}", path);
        match origpath.file_name() {
            Some(filename) => path = path.join(filename),
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
            io::stdout().flush()?;
            let status = io::stdin().read_line(&mut answer)?;
            if status == 0 {return Err(Error::new(ErrorKind::UnexpectedEof,"End of file"));}
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
    Ok(result)
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
                         printfile = ask_for_filename("Print file","pdf")?;
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
                            run_one_train(system,working_industries,&mut printer)?;
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
            'R' | 'r' => {break;},
            _ => {panic!("Should never get here");},
        }
        if needwait {wait_any_key(stdout,"Hit any key to continue")?;}
    }
    Ok(())
}

fn menu_car_type<W>(system: &System,  stdout: &mut W) 
        -> io::Result<Option<char>>
where
    W: io::Write,
{
    let mut CTMenuString: String = String::from("");
    let mut allowedV = Vec::from(system.CarTypesOrder());
    allowedV.push(' ');
    let allowed = allowedV.as_slice();
    let mut tcount = 0;
    for t in system.CarTypesOrder() {
        let ct = system.TheCarType(*t).unwrap();
        let buf = format!("Enter <{}> for {:11.11} ",*t,ct.Type());
        CTMenuString += &buf;
        tcount += 1;
        if tcount == 3 {
            CTMenuString += "\n";
            tcount = 0;
        }
    }
    CTMenuString += "Enter < > to return\n";
    let CTMenu: &str = &CTMenuString;
    let answer = menu(stdout,CTMenu,allowed)?;
    if answer != ' ' {
        Ok(Some(answer))
    } else {
        Ok(None)
    }
}

fn car_type_report<W>(system: &System,  stdout: &mut W,printer: &mut Printer) -> io::Result<()> 
where
    W: io::Write,
{
    let CTReportMenu: &str = &(system.SystemName() + "\n" + "\n" +
        "Enter <A> for all types\n" +
        "Enter <T> for a specific type\n" +
        "Enter <S> for a summery report\n" +
        "Enter <R> to return to the report menu\n" +
        "Enter [ATSR]: ");
    let reporttype = menu(stdout,CTReportMenu,&['A','a','T','t','S','s',
                                                'R','r'])?;
    match reporttype {
        'A' | 'a' => system.ReportCarTypes(CarTypeReport::All,' ',printer),
        'T' | 't' => {
                    let typechar = menu_car_type(system,stdout)?;
                    if typechar.is_some() {
                        system.ReportCarTypes(CarTypeReport::Type,
                                              typechar.unwrap(),printer);
                    };
                    },
        'S' | 's' => system.ReportCarTypes(CarTypeReport::Summary,' ',printer),
        'R' | 'r' => (),
        _ => panic!("Should never get here!"),
    };
    Ok(())
}

fn get_usize(prompt: &str) -> io::Result<usize> {
    let mut answer = String::new();
    print!("\n{}",prompt); io::stdout().flush()?;
    let status = io::stdin().read_line(&mut answer)?;
    if status == 0 {return Err(Error::new(ErrorKind::UnexpectedEof,"End of file"));}
    let result = match answer.trim().parse::<usize>() {
        Ok(m) => m,
        Err(e) => {println!("{}",e.to_string()); return Ok(0);}
    };
    Ok(result)
}

fn get_yesno<W>(stdout: &mut W,prompt: &str) -> io::Result<bool>
where
    W: io::Write,
{
    let YesNoMenu: &str = &(String::from(prompt) + "\nEnter [YN]: ");
    let answer = menu(stdout,YesNoMenu,&['Y','y','N','n'])?;
    match answer {
        'Y' | 'y' => Ok(true),
        'N' | 'n' => Ok(false),
        _ => panic!("Should never get here!"),
    }
}
fn car_locations_report<W>(system: &System, stdout: &mut W,
                        printer: &mut Printer,
                    working_industries: &mut HashMap<usize, IndustryWorking>) 
        -> io::Result<()> 
where
    W: io::Write,
{
    let LocReportMenu: &str = &(system.SystemName() + "\n" + "\n" +
        "Enter <I> for Cars at an Industry\n" +
        "Enter <S> for Cars at a Station\n" +
        "Enter <D> for Cars at a Division\n" +
        "Enter <A> for Cars at All locations\n" +
        "Enter <R> to return to the report menu\n" +
        "Enter [ISDAR]: ");
    let reporttype = menu(stdout,LocReportMenu,&['I','i','S','s','D','d',
                                                 'A','a','R','r'])?;
    match reporttype {
        'I' | 'i' => {
                    let ind = get_usize("Industry index:")?;
                    system.ReportCarLocations(CarLocationType::INDUSTRY,ind,
                            printer,working_industries);
                    },
        'S' | 's' => {
                    let ind = get_usize("Station index:")?;
                    system.ReportCarLocations(CarLocationType::STATION,ind,
                            printer,working_industries);
                    },
        'D' | 'd' => {
                    let ind = get_usize("Division index:")?;
                    system.ReportCarLocations(CarLocationType::DIVISION,ind,
                            printer,working_industries);
                    },
        'A' | 'a' =>{
                    let ind = get_yesno(stdout,"Print Workbench Cars?")?;
                    system.ReportCarLocations(CarLocationType::ALL,ind as usize,
                            printer,working_industries);
                    },
        'R' | 'r' => (),
        _ => panic!("Should not ever get here."),
    }
    Ok(())
}

fn get_owner()  -> io::Result<String>
{
    let mut answer = String::new();
    print!("\nEnter owner initials: "); io::stdout().flush()?;
    let status = io::stdin().read_line(&mut answer)?;
    if status == 0 {
        return Err(Error::new(ErrorKind::UnexpectedEof,"End of file"));
    }
    Ok(answer.trim().to_string())
}

fn owners_peport(system: &System,printer: &mut Printer) -> io::Result<()> 
{
    let owner: String = get_owner()?;
    if owner.len() > 0 {
        system.ReportCarOwners(owner,printer);
    }
    Ok(())
}

fn reports_menu<W>(system: &System,stdout: &mut W,
                    working_industries: &mut HashMap<usize, IndustryWorking>) -> io::Result<()> 
where
     W: io::Write,
{
    let mut printfile = String::from("");
    loop {
        let a = if printfile.len() == 0 {
            String::from("Print file name is unset\nEnter <P> To set print filename\n")
        } else {
            format!("Print file name is {}\n",printfile)
        };
        
        let z = if printfile.len() == 0 {
            String::from("Enter [PITCNLOAWR]: ")
        } else {
            String::from("Enter [ITCNLOAWR]: ")
        };
        let ReportsMenu: &str = &(system.SystemName() + "\n" + "\n" +
            &a +
            "Enter <I> for Industries Report\n" +
            "Enter <T> for Trains Report\n" +
            "Enter <C> for Cars Report\n" +
            "Enter <N> for Cars Not Moved Report\n" +
            "Enter <L> for Car (Load) types Report\n" +
            "Enter <O> for Car lOcations Report\n" +
            "Enter <A> for Analysis Report\n" +
            "Enter <W> for Car oWners Report\n" +
            &z);
        let allowed: &[char] = if printfile.len() == 0 {
            &['P','p','I','i','T','t','C','c','N','n','L','l','O','o','A','a',
              'W','w','R','r']
        } else {
            &['I','i','T','t','C','c','N','n','L','l','O','o','A','a','W','w',
              'R','r']
        };
        let key = menu(stdout,ReportsMenu,allowed)?;
        //let mut needwait = true;
        match key {
            'P' | 'p' => { 
                         printfile = ask_for_filename("Print file","pdf")?;
                         //needwait = false;
                         },
            'I' | 'i' => {
                        if printfile.len() == 0 {
                            println!("Select a PDF file to print to!");
                        } else {
                            let mut printer: Printer = Printer::new(&printfile,
                                                        "Industries Report",
                                                        PageSize::Letter);
                            system.ReportIndustries(&mut printer);
                            printfile = String::new(); 
                        }
                         },
            'T' | 't' => {
                        if printfile.len() == 0 {
                            println!("Select a PDF file to print to!");
                        } else {
                            let mut printer: Printer = Printer::new(&printfile,
                                                        "Trains Report",
                                                        PageSize::Letter);
                            system.ReportTrains(&mut printer);
                            printfile = String::new(); 
                        }
                         },
            'C' | 'c' => {
                        if printfile.len() == 0 {
                            println!("Select a PDF file to print to!");
                        } else {
                            let mut printer: Printer = Printer::new(&printfile,
                                                        "Cars Report",
                                                        PageSize::Letter);
                            system.ReportCars(&mut printer,
                                              working_industries);
                            printfile = String::new(); 
                        }
                         },
            'N' | 'n' => {
                        if printfile.len() == 0 {
                            println!("Select a PDF file to print to!");
                        } else {
                            let mut printer: Printer = Printer::new(&printfile,
                                                        "Cars Not Moved Report",
                                                        PageSize::Letter);
                            system.ReportCarsNotMoved(&mut printer,
                                              working_industries);
                            printfile = String::new(); 
                        }
                         },
            'L' | 'l' => {
                        if printfile.len() == 0 {
                            println!("Select a PDF file to print to!");
                        } else {
                            let mut printer: Printer = Printer::new(&printfile,
                                                        "Cars Types Report",
                                                        PageSize::Letter);
                            car_type_report(system,stdout,&mut printer)?;
                            printfile = String::new(); 
                        }
                         },
            'O' | 'o' => {
                        if printfile.len() == 0 {
                            println!("Select a PDF file to print to!");
                        } else {
                            let mut printer: Printer = Printer::new(&printfile,
                                                        "Cars Locations Report",
                                                        PageSize::Letter);
                            car_locations_report(system,stdout,&mut printer,
                                              working_industries)?;
                            printfile = String::new(); 
                        }
                         },
            'A' | 'a' => {
                        if printfile.len() == 0 {
                            println!("Select a PDF file to print to!");
                        } else {
                            let mut printer: Printer = Printer::new(&printfile,
                                                        "Analysis Report",
                                                        PageSize::Letter);
                            system.ReportAnalysis(&mut printer,
                                              working_industries);
                            printfile = String::new(); 
                        }
                         },
            'W' | 'w' => {
                        if printfile.len() == 0 {
                            println!("Select a PDF file to print to!");
                        } else {
                            let mut printer: Printer = Printer::new(&printfile,
                                                        "Owners Report",
                                                        PageSize::Letter);
                            owners_peport(system,&mut printer)?;
                            printfile = String::new(); 
                        }
                         },
            'R' | 'r' => {break;},
            _ => {panic!("Should never get here");}, 
        }
        //if needwait {
        //    wait_any_key(stdout,"Hit any key to continue")?;
        //} 
    }
    Ok(())
}

fn movements_by_train(system: &System) -> io::Result<()> { 
    println!("{}",system.SystemName());
    println!("\nEnter train name to show car movements\n");
    let mut key = String::new();
    print!("Train: "); io::stdout().flush()?;
    let status = io::stdin().read_line(&mut key)?;
    if status == 0 {return Err(Error::new(ErrorKind::UnexpectedEof,"End of file"));}
    let trainName = key.trim();
    match system.TrainByName(trainName.to_string()) {
        Some(val) => system.ShowCarMovements(false,Some(val),None),
        None => println!("No such train: {}",trainName),
    };
    Ok(())
}

fn movements_by_location(system: &System) -> io::Result<()> {
    println!("{}",system.SystemName());
    println!("\nEnter location code to show car movements\n");
    let mut key = String::new();
    print!("Location: "); io::stdout().flush()?;
    let status = io::stdin().read_line(&mut key)?;
    if status == 0 {return Err(Error::new(ErrorKind::UnexpectedEof,"End of file"));}
    let Ix = match key.trim().parse::<usize>() {
        Ok(m) => m,
        Err(e) => {println!("{}",e.to_string()); return Ok(());}
    };
    let IOpt = system.IndustryByIndex(Ix);
    if IOpt.is_none() {
        println!("No such industry: {}",Ix);
    } else {
        system.ShowCarMovements(true,None,IOpt);
    };
    Ok(())    
}

//fn compile_car_movements(system: &System) {
//}

fn show_cars_in_division(system: &System)  -> io::Result<()> {
    println!("{}",system.SystemName());
    println!("\nEnter division symbol to show car movements\n");
    let mut key = String::new();
    print!("Division symbol: "); io::stdout().flush()?;
    let status = io::stdin().read_line(&mut key)?;
    if status == 0 {return Err(Error::new(ErrorKind::UnexpectedEof,"End of file"));}
    let divsymb = key.chars().next().unwrap_or(' ');
    let divindex = system.FindDivisionIndexBySymbol(divsymb);
    match divindex {
        Some(Dx) => system.ShowCarsInDivision(Dx),
        None => println!("No such division: {}",divsymb),
    };
    Ok(())
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
        //let mut needwait = true;
        match key {
            'N' | 'n' => system.ShowCarsNotMoved(),
            'M' | 'm' => system.ShowCarMovements(false, None, None),
            'T' | 't' => movements_by_train(&system)?,
            'L' | 'l' => movements_by_location(&system)?,
            'E' | 'e' => system.ShowCarMovements(true, None, None),
            //'C' | 'c' => compile_car_movements(&system),
            'D' | 'd' => show_cars_in_division(&system)?,
            'A' | 'a' => system.ShowTrainTotals(),
            //'U' | 'u' => system.MarkAllCarsInUse(),
            '?'         => system.ListTrainNames(false,None),
            'R' | 'r'  => {break;},
            _ => panic!("Should never get here"),
        }
        //if needwait {
            wait_any_key(stdout,"Hit any key to continue")?;
        //}
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
             'R' | 'r' => {reports_menu(&system,&mut stdout,&mut working_industries)?;needwait = false;}
             'I' | 'i' => system.ResetIndustryStats(&mut working_industries),
             'Q' | 'q' => break,
             _ => panic!("Unreconized command character: {}",cmd),
        }
        if needwait {wait_any_key(&mut stdout,"Hit any key to continue")?;}
    }
    Ok(())
}
