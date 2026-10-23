use statrs::distribution::{ContinuousCDF, Normal};

pub fn executar() {
    let media_liquido = 900.0;
    let desvio_liquido: f64 = 10.0;
    let media_embalagem = 100.0;
    let desvio_embalagem: f64 = 4.0;
    let referencia = 1020.0;
    let intervalo = (980.0, 1020.0);

    let media_bruto = media_liquido + media_embalagem;
    let desvio_bruto = (desvio_liquido.powi(2) + desvio_embalagem.powi(2)).sqrt();
    let distribuicao = Normal::new(media_bruto, desvio_bruto).unwrap();

    let acima_referencia = distribuicao.sf(referencia);
    let entre_limites = distribuicao.cdf(intervalo.1) - distribuicao.cdf(intervalo.0);

    println!(
        "a) Peso bruto maior que {:.0} g: {:.2}%",
        referencia,
        acima_referencia * 100.0
    );
    println!(
        "b) Peso bruto entre {:.0} g e {:.0} g: {:.2}%",
        intervalo.0,
        intervalo.1,
        entre_limites * 100.0
    );
}
