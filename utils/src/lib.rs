use std::{net::{IpAddr, Ipv4Addr}, path::PathBuf};

use sysinfo::{Process, System};

pub fn check_connection() -> bool{
    let result = ping::ping(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)) , None, None, None, None, None);
    return result.is_ok();
}

pub fn get_process_directory(name: &String) -> Result<PathBuf, &'static str>{
    let sys = System::new_all();
    let processes = sys.processes_by_exact_name(name).collect::<Vec<&Process>>();
    if processes.len() == 0{
        return Err("Process not found");
    }
    let process = processes[0];
    return Ok(process.exe().unwrap().parent().unwrap().to_path_buf());
}

pub fn kill_proc_by_name(name: &String){
    let sys = System::new_all();
    for proc in sys.processes_by_exact_name(name){
        proc.kill();
    }
}