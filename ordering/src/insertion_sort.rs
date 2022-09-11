///
/// # Insertion Sort
/// Nesse algoritmo existem 2 loops, um `for` e um `while`.
/// ##
/// O loop `for` percorre o vetor no sentido pra direita, enquanto o `while` percorre no sentido
/// pra esquerda.
/// ##
/// Durante o loop while é comparado um elemento com o seu anterior, caso o anterior seja maior que
/// o atual ocorre uma troca da posição dos elementos.
#[allow(dead_code)]
pub fn sort<T: PartialOrd>(vector: &mut Vec<T>) {
    for pivot in 0..vector.len() {
        let mut indice = pivot;

        while indice > 0 && vector[indice] < vector[indice-1] {
            vector.swap(indice, indice-1);
            indice -= 1;
        }
    }
}