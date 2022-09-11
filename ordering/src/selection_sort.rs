///
/// # Selection Sort
/// O algoritmo funciona por meio de dois loops `for`.
/// O primeiro possui como variável de contagem `pivo`, o segundo `indice`.
///
/// O primeiro loop `for` percorre o vetor. Em cada interação do primeiro loop for,
/// o segundo loop é executado percorrendo denovo o vetor à procura do índice do menor elemento.
///
/// Ao final do primeiro loop for, é executado uma expressão `if` que verifica se o elemento `pivo`
/// é o menor, caso contrário ele troca o elemento `pivo` pelo menor elemento.
///
#[allow(dead_code)]
pub fn sort<T: PartialOrd>(vector: &mut Vec<T>) {
    for pivo in 0..vector.len() {
        let mut smallest = pivo;

        for indice in pivo+1..vector.len() {
                if vector[indice] < vector[smallest] {
                    smallest = indice;
                }
        }

        if vector[pivo] != vector[smallest] {
            vector.swap(pivo, smallest);
        }
    }
}