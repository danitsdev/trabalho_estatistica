use statrs::distribution::{Binomial, DiscreteCDF};

pub fn executar() {
    let bits = 10_000;
    let probabilidade_erro = 0.0002;
    let limite_erros = 4;

    let distribuicao = Binomial::new(probabilidade_erro, bits).unwrap();
    let probabilidade = distribuicao.sf(limite_erros);

    println!(
        "Probabilidade de mais de {} bits com erro: {:.2}%",
        limite_erros,
        probabilidade * 100.0
    );
}
