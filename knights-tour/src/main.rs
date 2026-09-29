use std::time::Instant;

const N: usize = 8;

struct Estatisticas {
    recursoes: u64,
    retrocessos: u64,
}

fn main() {
    let (linha_inicial, coluna_inicial) = (0, 0);
    let h = [2, 1, -1, -2, -2, -1, 1, 2];
    let v = [1, 2, 2, 1, -1, -2, -2, -1];

    let mut tabuleiro = [[0i32; N]; N];
    let mut sucesso = false;
    let mut estatisticas = Estatisticas {
        recursoes: 0,
        retrocessos: 0,
    };

    tabuleiro[linha_inicial][coluna_inicial] = 1;
    let inicio = Instant::now();

    tenta(
        2,
        linha_inicial,
        coluna_inicial,
        &mut sucesso,
        &mut tabuleiro,
        &h,
        &v,
        &mut estatisticas,
    );

    let tempo = inicio.elapsed();

    if sucesso {
        println!("Solução encontrada:");
        for linha in tabuleiro {
            println!("{:?}", linha);
        }
    } else {
        println!("Nenhuma solução foi encontrada.");
    }

    println!("Tempo total: {:?}", tempo);
    println!("Chamadas recursivas: {}", estatisticas.recursoes);
    println!(
        "Backtrackings (passos desfeitos): {}",
        estatisticas.retrocessos
    );
}

fn tenta(
    k: i32,
    x: usize,
    y: usize,
    sucesso: &mut bool,
    tabuleiro: &mut [[i32; N]; N],
    h: &[i32; 8],
    v: &[i32; 8],
    estatisticas: &mut Estatisticas,
) {
    estatisticas.recursoes += 1;
    let mut m = 0;

    while m < 8 && !(*sucesso) {
        let xn = x as i32 + h[m];
        let yn = y as i32 + v[m];

        if xn >= 0 && xn < N as i32 && yn >= 0 && yn < N as i32 {
            let xn = xn as usize;
            let yn = yn as usize;

            if tabuleiro[xn][yn] == 0 {
                tabuleiro[xn][yn] = k;

                if k < (N * N) as i32 {
                    tenta(k + 1, xn, yn, sucesso, tabuleiro, h, v, estatisticas);
                    if !(*sucesso) {
                        tabuleiro[xn][yn] = 0;
                        estatisticas.retrocessos += 1;
                    }
                } else {
                    *sucesso = true;
                }
            }
        }

        m += 1;
    }
}
