use sysinfo::{System, SystemExt};

pub mod Components;

fn main() {

    println!("\nSystem Information:\n");

    let mut system = System::new_all();


    //To Display the CPU Detsils
    Components::CPU::display(&mut system);

    //To Display the Network Detsils
    Components::Network::display(&mut system);

}