use statrs::distribution::{Discrete, Hypergeometric};

pub fn executar() {
    let mulheres = 60;
    let homens = 40;
    let sorteios = 2;
    let mulheres_desejadas = 1;

    let distribuicao = Hypergeometric::new(mulheres + homens, mulheres, sorteios).unwrap();
    // Logaritmos evitam overflow das combinações em populações maiores.
    let probabilidade = if mulheres_desejadas > sorteios {
        0.0
    } else {
        distribuicao.ln_pmf(mulheres_desejadas).exp()
    };

    println!(
        "Probabilidade de exatamente {} mulher(es): {:.2}%",
        mulheres_desejadas,
        probabilidade * 100.0
    );
}
