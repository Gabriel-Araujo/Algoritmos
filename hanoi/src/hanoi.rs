use crate::tower::Tower;

///
/// ## A variável `n`:
/// N guarda a quantidade de discos que estão na torre de origem.
///
/// ## Ideia do Algoritmo:
/// O algoritmo de hanoi foi implementado de forma recursiva. O caso base é
/// quando só há um disco na torre de origem, quando esse caso ocorre, o disco
/// na torre de origem é movido direto para a torre de destino.
///
/// Nos casos em que n > 1, o algoritmo vai mover todos os discos menos a base para uma torre
/// auxiliar. De forma recursiva a torre de origem e auxiliar mudam.
///
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
