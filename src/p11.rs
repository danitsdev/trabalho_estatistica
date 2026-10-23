use statrs::distribution::{Binomial, Discrete, DiscreteCDF};

pub fn executar() {
    let itens = 20;
    let probabilidade_defeito = 0.05;
    let distribuicao = Binomial::new(probabilidade_defeito, itens).unwrap();

    let algum_defeito = distribuicao.sf(0);
    let exatamente_dois = distribuicao.pmf(2);
    let mais_de_dois = distribuicao.sf(2);
    let esperados_defeituosos = itens as f64 * probabilidade_defeito;
    let esperados_bons = itens as f64 * (1.0 - probabilidade_defeito);
    let variancia = itens as f64 * probabilidade_defeito * (1.0 - probabilidade_defeito);

    println!("a) Pelo menos um defeituoso: {:.2}%", algum_defeito * 100.0);
    println!(
        "b) Exatamente dois defeituosos: {:.2}%",
        exatamente_dois * 100.0
    );
    println!("c) Mais de dois defeituosos: {:.2}%", mais_de_dois * 100.0);
    println!("d) Esperados defeituosos: {:.2}", esperados_defeituosos);
    println!("   Esperados bons: {:.2}", esperados_bons);
    println!("e) Variância: {:.4}", variancia);
}
