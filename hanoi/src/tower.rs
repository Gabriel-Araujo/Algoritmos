pub struct Tower {
    name: String,
    pub disks: Vec<Disk>,
}

pub struct Disk {
    size: u8,
}

impl Tower {
    pub fn new(name: String, disks_qnt: u8) -> Self {
        let disks = Self::create_disks(disks_qnt);
        Tower {name, disks}
    }

    pub fn move_disk(origin: &mut Tower, destiny: &mut Tower) {
        if origin.disks.len() > 0 {
            destiny.disks.insert(0, origin.disks.remove(0));
        }
    }

    fn create_disks(disk_qnt: u8) -> Vec<Disk>{
        let mut new_vector:Vec<Disk> = Vec::with_capacity(disk_qnt as usize);
        let mut hold = 0;
        for _ in 0..disk_qnt {
            new_vector.push(Disk::new(hold+1));
            hold += 1;
        }

        new_vector
    }

    pub fn show_disks(&self) {
        for element in &self.disks {
            println!("disk {}", element.size);
        }
    }
}

impl Disk {
    pub fn new(size: u8) -> Self {
        Disk {size}
    }
}