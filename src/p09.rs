pub fn executar() {
    let configuracao = 1;
    let [c1, c2, c3, c4] = [0.90, 0.95, 0.85, 0.80];

    let probabilidade = match configuracao {
        1 => 1.0 - (1.0 - c1 * c2) * (1.0 - c3 * c4),
        2 => (1.0 - (1.0 - c1) * (1.0 - c3)) * (1.0 - (1.0 - c2) * (1.0 - c4)),
        _ => {
            println!("Configuração inválida.");
            return;
        }
    };

    println!("Configuração {}", configuracao);
    println!(
        "Probabilidade de funcionamento: {:.2}%",
        probabilidade * 100.0
    );
}
