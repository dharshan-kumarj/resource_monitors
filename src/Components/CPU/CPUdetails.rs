use sysinfo::{System,SystemExt,CpuExt};


pub fn display(sys: &mut System) {
    sys.refresh_all();
    
    let cpus = sys.cpus();
    let total_core = cpus.len();
    let frequency = cpus.first().map(|cpu| cpu.frequency()).unwrap_or(0);
    let usage: Vec<f32> = cpus.iter().map(|cpu| cpu.cpu_usage()).collect();
    
    println!("Total cores: {}", total_core);
    println!("Frequency: {}", frequency);
    
    for (i, core_usage) in usage.iter().enumerate() {
        println!("Core {}: {:.2}%", i, core_usage);
    }
}