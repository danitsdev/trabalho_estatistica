pub fn executar() {
    let taxa_falha: f64 = 0.012;
    let tempos = [("a", 50.0), ("b", 100.0)];

    for (letra, horas) in tempos {
        let probabilidade = (-taxa_falha * horas).exp();

        println!(
            "{}) Probabilidade de sobreviver a {:.0} horas: {:.2}%",
            letra,
            horas,
            probabilidade * 100.0
        );
    }
}
