fn main() {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    println!("Total memory: {} KB", sys.total_memory());
    println!("Used memory : {} KB", sys.used_memory());
    println!("Total swap  : {} KB", sys.total_swap());
    println!("Used swap   : {} KB", sys.used_swap());
}
