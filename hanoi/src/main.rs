#[allow(dead_code)]
mod tower;
mod hanoi;

use tower::Tower;
use hanoi::{hanoi, show_towers};

fn main() {
    let mut a_tower = Tower::new("A".to_string(), 5);
    let mut b_tower = Tower::new("B".to_string(), 0);
    let mut c_tower = Tower::new("C".to_string(), 0);

    show_towers(&a_tower, &b_tower, &c_tower);

    println!("\nMoving\n");

    hanoi(a_tower.disks.len() as u8, &mut a_tower, &mut b_tower, &mut c_tower);

    show_towers(&a_tower, &b_tower, &c_tower);
}
