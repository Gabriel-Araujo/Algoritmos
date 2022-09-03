mod random_vector;
mod insertion_sort;


fn main() {
    let mut vector:Vec<u8> = random_vector::generate(10);

    show_elements(&vector);

    println!("orderly:");

    insertion_sort::sort(&mut vector);

    show_elements(&vector);
}

fn show_elements<T: std::fmt::Display>(vector: &Vec<T>) {
    for element in vector {
        println!("{}", element);
    }
}
