use systemparser::{SystemInformation, System};

fn main() {
    let system: Vec<System> = vec![
        System::Cpu,
        System::Ram,
        System::Uptime,
        System::LoadAverage,
        System::Drivers,
        System::VmStats,
        System::Power,
        System::Bios,
        System::VulnerabilityCheck,
        System::Ipv4,
        System::Ipv6,
    ];
    for component in system {
        let mut data = SystemInformation::new(component);
        data.display();
    }
}
