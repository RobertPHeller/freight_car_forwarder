pub use freight_car_forwarder::system::System;

fn main() {
    let system = System::new(String::from("../../Deepwoods/ModelRRSystem/ChesapeakeSystem/system.dat"));
    println!("system is {}",system);
    //println!("Divisions are:");
    //for (idiv, div) in system.DivisionIter() {
    //    println!("    {}: {}",idiv,div);
    //}
    //println!("Stations are:");
    //for (ista, sta) in system.StationIter() {
    //    println!("    {}: {}",ista,sta);
    //}
    //println!("Indiustries are:");
    //for (iind, ind) in system.IndustryIter() {
    //    println!("    {}: {}",iind,ind);
    //}
    println!("Trains:");
    for (itrn, trn) in system.TrainIter() {
        println!("    {}: {}",itrn,trn);
    }
}
