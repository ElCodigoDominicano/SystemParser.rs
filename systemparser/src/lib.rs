/*
 * Rust-Based System Information CLI tool
 *
 * Used  to obtain various information with one cli tool

 * Information such as:
 *  - Hardware Specification
 *  - OS Information
 *  - Network Information
 *  - I/O Information (port information)
 *  - Includes PortScanning Tool.
 *
 *  Author: ElCodigoDominicano
 *  Date: 1/31/2024
 *
 * */

mod port_scanner;

use std::{env, io};
use std::fmt::Debug;
use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum System {
    // Os
    Cpu,
    // CpuArch,
    Ram,
    Uptime,
    LoadAverage,
    Drivers,
    VmStats,
    Power,
    Bios,
    VulnerabilityCheck,
    Ipv4,
    Ipv6
}

#[derive(Debug)]
pub struct SystemInformation {
    system: System,
    value: HashMap<String, String>,
}

impl SystemInformation {
    pub fn new(system: System) -> SystemInformation {
        SystemInformation {
            system,
            value: HashMap::new(),
        }
    }

    pub fn display(&mut self)  {
        match self.system {
            System::Cpu => {
                SystemInformation::parse_file(self, String::from("/proc/cpuinfo"));
                println!("{:?}", self);
            },
            System::Ram => {
                SystemInformation::parse_file(self, String::from("/proc/meminfo"));
                println!("{:?}", self);
            },
            System::Drivers => {
                SystemInformation::parse_file(self, String::from("/proc/modules"));
                println!("{:?}", self);
            },
            System::LoadAverage => {
                SystemInformation::parse_file(self, String::from("/proc/loadavg"));
                println!("{:?}", self);
            },
            System::Uptime => {
                SystemInformation::parse_file(self, String::from("/proc/uptime"));
                println!("{:?}", self);
            },
            System::VmStats => {
                SystemInformation::parse_file(self, String::from("/proc/vmstat"));
                println!("{:?}", self);
            },
            System::Power => { 
                SystemInformation::scan_dir(self, String::from("/sys/devices/virtual/dmi/id/power")).unwrap();
                println!("{:?}", self);
            },
            System::Bios => {
                SystemInformation::scan_dir(self, String::from("/sys/devices/virtual/dmi/id")).unwrap();
                println!("{:?}", self);
            },
            System::VulnerabilityCheck => {
                SystemInformation::scan_dir(self, String::from("/sys/devices/system/cpu/vulnerabilities")).unwrap();
                println!("{:?}", self);
            },
            System::Ipv4 => {
                SystemInformation::scan_dir(self, String::from("/proc/sys/net/ipv4")).unwrap();
                println!("{:?}", self);
            },
            System::Ipv6 => {
                SystemInformation::scan_dir(self, String::from("/proc/sys/net/ipv6")).unwrap();
                print!("{:?}", self);
            },
            // System::CpuArch => env::consts::ARCH.to_string(),
            // Self::Os => env::consts::OS.to_string(),
        };
    }
    pub fn parse_file(&mut self, fname: String)  {
        let file_name = fs::read_to_string(&fname).unwrap_or_else(|err| err.to_string());
        let file_lines = file_name.lines();
        
        for x in file_lines {
            if let Some(val) = x.find(":") {
                let keys = &x[0..val];
                let values = &x[val + 1..x.len()];
                self.value.insert(keys.to_string(), values.to_string());
            } else if let Some(val) = x.rfind(",") {
                let keys = &x[0..val];
                let values = &x[val + 1..x.len()];
                self.value.insert(keys.to_string(), values.to_string());
            } else if let Some(val) = x.find("-") {
                let keys = &x[0..val];
                let values = &x[val + 1..x.len()];
                self.value.insert(keys.to_string(), values.to_string());
            }

            if self.system == System::Uptime {
                self.value.insert("uptime".to_string(), x.to_string());
            } else if self.system == System::LoadAverage {
                self.value.insert("load_average".to_string(),x.to_string() );
            } else if self.system == System::VmStats {
                if let Some(val) = x.find(" ") {
                    let keys = &x[0..val];
                    let values = &x[val + 1..x.len()];
                    self.value.insert(keys.to_string(), values.to_string());
                }

            }
        }
    }

    fn scan_dir(&mut self, dir_name: String) -> io::Result<()> {
        let vec_of_path_buffers = fs::read_dir(dir_name)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result::<Vec<_>, io::Error>>()?;
        Ok(SystemInformation::parse_dir_files(self, vec_of_path_buffers))
    }

    fn parse_dir_files(&mut self, dir_files: Vec<PathBuf>)  {
        for file in &dir_files {
            if file.is_file() {
                let net_dir = file.to_str().unwrap_or_else(|| "No file found.");
                let last_foward_slash = net_dir.rfind("/").unwrap();
                let net_key= net_dir[last_foward_slash+1..net_dir.len()].to_string();

                let net_value = match fs::read_to_string(file) {
                    Ok(val) => val.to_string(),
                    Err(err) => err.to_string(),
                };
                self.value.insert(net_key, net_value);
            }
        };
    }
}