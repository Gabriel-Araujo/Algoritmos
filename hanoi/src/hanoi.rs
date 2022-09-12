use crate::tower::Tower;

pub fn hanoi(n: u8, origin: &mut Tower, aux: &mut Tower, destiny: &mut Tower) {
    if n == 1 { // Caso Base
        Tower::move_disk(origin, destiny);
    }
    else {
        hanoi(n - 1, origin, destiny, aux);
        Tower::move_disk(origin, destiny);
        hanoi(n - 1, aux, origin, destiny);
    }
}


pub fn show_towers(a: &Tower, b: &Tower, c: &Tower) {
    println!("A tower:");
    a.show_disks();
    println!("B tower:");
    b.show_disks();
    println!("C tower:");
    c.show_disks();
}
