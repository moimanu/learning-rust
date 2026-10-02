use rand::Rng;
use rayon::prelude::*;

struct Estatisticas {
    recursoes: u64,
}

fn main() {
    let mut estatisticas = Estatisticas { recursoes: 0 };
    let tamanho = 1_000_000;

    let vetor: Vec<i32> = (0..tamanho)
        .into_par_iter()
        .map_init(rand::thread_rng, |rng, _| rng.r#gen())
        .collect();

    println!("Vetor de {} elementos gerado com sucesso!", tamanho);

    if vetor.is_empty() {
        println!("O vetor está vazio!");
        return;
    }

    let (min, max) = maxmin(&vetor, &mut estatisticas);

    println!("O maior valor é: {}", max);
    println!("O menor valor é: {}", min);
    println!("Chamadas recursivas: {}", estatisticas.recursoes);
}

fn maxmin(vetor: &[i32], estatisticas: &mut Estatisticas) -> (i32, i32) {
    estatisticas.recursoes += 1;

    match vetor {
        [unico] => (*unico, *unico),

        [a, b] => (*a.min(b), *a.max(b)),

        _ => {
            let meio = vetor.len() / 2;

            let (esquerda, direita) = vetor.split_at(meio);

            let (min1, max1) = maxmin(esquerda, estatisticas);
            let (min2, max2) = maxmin(direita, estatisticas);

            (min1.min(min2), max1.max(max2))
        }
    }
}
