use crate::utils;

pub struct BatteryMonitor {
    status: String,
    capacity: String,
    capacity_level: String,
}

impl BatteryMonitor {
    pub fn new() -> Option<BatteryMonitor> {
        return Some(BatteryMonitor {
            status: std::fs::read_to_string(format!("/sys/class/power_supply/BAT1/{}", "status")).unwrap(),
            capacity: std::fs::read_to_string(format!("/sys/class/power_supply/BAT1/{}", "capacity")).unwrap(),
            capacity_level: std::fs::read_to_string(format!("/sys/class/power_supply/BAT1/{}", "capacity_level")).unwrap(),
        });
    }
    pub fn show(self){
        utils::headline(String::from("batinfo"));
        print!("status\t\t:{}",self.status);
        print!("capacity\t:{}",self.capacity);
        print!("capacity_level\t:{}",self.capacity_level);
    }
}
