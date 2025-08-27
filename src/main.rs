pub use freight_car_forwarder::system::System;

fn main() {
    let system = System::new(String::from("../../Deepwoods/ModelRRSystem/ChesapeakeSystem/system.dat"));
    println!("system is {:#?}",system);
}
