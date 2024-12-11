use sysinfo::{System, SystemExt, Networks, NetworkExt};

pub fn display(sys: &mut System) {
    sys.refresh_all();
    
    let networks = sys.networks();
    
    println!("\nNetwork Details:\n");
    
    // Total number of network interfaces
    println!("Total Network Interfaces: {}", networks.len());
    
    // Iterate through each network interface
    for (interface_name, network) in networks.iter() {
        println!("\nInterface: {}", interface_name);
        
        // Network information
        println!("MAC Address: {}", network.mac_address());
        
        // Received and transmitted data
        println!("Total Received: {:.2} MB", 
            network.total_received() as f64 / 1_000_000.0);
        println!("Total Transmitted: {:.2} MB", 
            network.total_transmitted() as f64 / 1_000_000.0);
        
        // Network operational status
        println!("Is Operational: {}", network.is_operational());
    }
}