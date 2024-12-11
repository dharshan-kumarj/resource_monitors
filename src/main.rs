use sysinfo::{System, SystemExt};

pub mod Components;

fn main() {

    println!("\nThe CPU Details:\n");
    let mut system = System::new_all();
    Components::CPU::display(&mut system);

}