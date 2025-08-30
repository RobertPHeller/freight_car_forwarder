extern crate getopts;
use getopts::Options;
use std::env;
//use std::io;
use std::io::{self, Write};


pub use freight_car_forwarder::system::System;
//use freight_car_forwarder::switchlist::*;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn manage_trains_and_printing(system: &mut System) {
}

fn run_one_train(system: &mut System) {
}

fn reports_menu(system: &mut System) {
}

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
    //let output = matches.opt_str("o");
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
        println!("Load cars file {}",system.CarsFile());
        println!("Save cars file {}",system.CarsFile());
        //println!("Groups = {}", SelectGroups$);
        println!("Manage trains/printing");
        //println!("View car information");
        //println!("Edit car information");
        //println!("Add a New car");
        //println!("Delete an existing car");
        println!("Show Unassigned cars");
        println!("Run the car Assignment procedure");
        println!("Run all Trains in Operating session");
        println!("Run the Boxmove trains");
        println!("Run Trains one at a time");
        println!("Print yard lists, etc");
        println!("Show Cars on screen");
        println!("Go to Reports Menu");
        println!("Reset Industry statistics");
        println!("Quit -- exit NOW");
        let mut command = String::new();
        println!("");
        print!("Enter [LSMUAOBTPCRIQ]: ");
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
             'M' | 'm' => manage_trains_and_printing(&mut system),
             'U' | 'u' => system.ShowUnassignedCars(),
             'A' | 'a' => system.CarAssignment(),
             'O' | 'o' => system.RunAllTrains(),
             'B' | 'b' => system.RunBoxMoves(),
             'T' | 't' => run_one_train(&mut system),
             'P' | 'p' => system.PrintAllLists(),
             'C' | 'c' => system.ShowCarMovements(true),
             'R' | 'r' => reports_menu(&mut system),
             'I' | 'i' => system.ResetIndustryStats(),
             'Q' | 'q' => break,
             _ => println!("Unreconized command character: {}",cmd),
        }
    }
}
