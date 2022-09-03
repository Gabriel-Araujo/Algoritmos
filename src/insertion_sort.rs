pub fn sort<T: std::cmp::PartialOrd>(vector: &mut Vec<T>) {
    for pivot in 0..vector.len() {
        let mut indice = pivot;

        while indice > 0 && vector[indice] < vector[indice-1] {
            vector.swap(indice, indice-1);
            indice -= 1;
        }
    }
}