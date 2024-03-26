use std::{path::PathBuf, process::exit};
use sysinfo::{Process, System};

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

pub fn count_processes(name: &String) -> usize{
    let sys = System::new_all();
    let processes: Vec<&Process> = sys.processes_by_exact_name(name).collect();
    return processes.len();
}

pub fn check_one_current_process(){
    let sys = System::new_all();
    let current_process =  sys.process(sysinfo::get_current_pid().unwrap()).unwrap();
    let count = count_processes(&current_process.name().to_string());
    if count > 1{
        exit(0);
    }
}