use std::env;
use std::process::Command;
use sysinfo::{System, SystemExt};

#[derive(Debug)]
pub struct Sysinfo {
    pub hostname: String,
    pub platform: String,
    pub os_number: String,
    pub cpu: String,
    pub gpu: String,
    pub ram: String,
    pub mainboard: String,
}

pub fn specs() -> Sysinfo {
    let hostname_output = Command::new("cmd")
        .args(["/C", "wmic path win32_ComputerSystem get name"])
        .output()
        .expect("failed to execute HOSTNAME process");

    let hostname = String::from_utf8(hostname_output.stdout).unwrap();

    let cpu_output = Command::new("cmd")
        .args(["/C", "wmic path win32_Processor get name"])
        .output()
        .expect("failed to execute CPU process");

    let cpu = String::from_utf8(cpu_output.stdout).unwrap();

    let gpu_output = Command::new("cmd")
        .args(["/C", "wmic path win32_VideoController get name"])
        .output()
        .expect("failed to execute GPU process");

    let gpu = String::from_utf8(gpu_output.stdout).unwrap();

    let mainboard_output = Command::new("cmd")
        .args(["/C", "wmic path win32_BaseBoard get Product"])
        .output()
        .expect("failed to execute MAINBOARD process");

    let mainboard = String::from_utf8(mainboard_output.stdout).unwrap();

    let info = os_info::get();
    let mut sys = System::new_all();
    sys.refresh_all();

    let ram: u64 = sys.total_memory() / 1024 / 1024;

    let specs = Sysinfo {
        hostname: remove_characters(&hostname),
        platform: env::consts::OS.to_string(),
        os_number: info.version().to_string(),
        cpu: remove_characters(&cpu),
        gpu: remove_characters(&gpu),
        ram: format!("{} GB", ram / 1024),
        mainboard: remove_characters(&mainboard),
    };

    specs
}


fn remove_characters(data: &str) -> String {
    let parts: Vec<&str> = data.split_whitespace().collect();

    parts[1..].join(" ")
}
