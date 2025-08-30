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
        

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();    
    opts.optopt("o", "", "set output file name", "NAME");
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
    
    let system = System::new(systemfile);
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
        if cmd == 'Q' || cmd == 'q' {break;}
    }
}
