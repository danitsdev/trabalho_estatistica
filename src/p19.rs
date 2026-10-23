use statrs::distribution::{ContinuousCDF, Normal};

pub fn executar() {
    let media = 48.0;
    let desvio_padrao = 3.0;
    let intervalo = (42.0, 49.0);
    let limite_superior = 52.0;
    let limite_inferior = 45.0;

    let distribuicao = Normal::new(media, desvio_padrao).unwrap();

    let entre_limites = distribuicao.cdf(intervalo.1) - distribuicao.cdf(intervalo.0);
    let acima = distribuicao.sf(limite_superior);
    let abaixo = distribuicao.cdf(limite_inferior);

    println!(
        "a) Entre {:.0} e {:.0} cm: {:.2}%",
        intervalo.0,
        intervalo.1,
        entre_limites * 100.0
    );
    println!(
        "b) Acima de {:.0} cm: {:.2}%",
        limite_superior,
        acima * 100.0
    );
    println!(
        "c) Abaixo de {:.0} cm: {:.2}%",
        limite_inferior,
        abaixo * 100.0
    );
}
