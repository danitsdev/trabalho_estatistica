use statrs::distribution::{ContinuousCDF, Normal};

pub fn executar() {
    let distribuicao = Normal::new(0.0, 1.0).unwrap();
    let intervalos = [
        ("a", 0.0, 1.73),
        ("b", 0.81, f64::INFINITY),
        ("c", -1.25, -0.63),
    ];

    for (letra, inicio, fim) in intervalos {
        let probabilidade = distribuicao.cdf(fim) - distribuicao.cdf(inicio);

        println!("{}) Probabilidade: {:.2}%", letra, probabilidade * 100.0);
    }
}
