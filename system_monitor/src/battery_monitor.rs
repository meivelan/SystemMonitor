pub struct BatteryMonitor {
    status: String,
    capacity: String,
    capacity_level: String,
}

impl BatteryMonitor {
    pub fn new(path: String) -> Option<BatteryMonitor> {
        return Some(BatteryMonitor {
            status: std::fs::read_to_string(format!("{}{}", path, "status")).unwrap(),
            capacity: std::fs::read_to_string(format!("{}{}", path, "capacity")).unwrap(),
            capacity_level: std::fs::read_to_string(format!("{}{}", path, "capacity_level")).unwrap(),
        });
    }
    pub fn show(self){
        print!("status:        {}",self.status);
        print!("capacity:      {}",self.capacity);
        print!("capacity_level:{}",self.capacity_level);
    }
}
