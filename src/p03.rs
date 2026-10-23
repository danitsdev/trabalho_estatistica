use std::collections::BTreeMap;

pub fn executar() {
    let empresas = BTreeMap::from([("A", 20), ("B", 15), ("C", 7), ("D", 5), ("E", 3)]);

    let mut total = 0;

    for (_, funcionarios) in &empresas {
        total += funcionarios;
    }

    if total == 0 {
        println!("Erro: o total de funcionários deve ser maior que zero.");
        return;
    }

    println!("a) Probabilidade de cada empresa:");
    for (nome, funcionarios) in &empresas {
        let probabilidade = *funcionarios as f64 / total as f64;

        println!("Empresa {}: {:.2}%", nome, probabilidade * 100.0);
    }

    let empresa_nao_selecionada = "A";
    println!(
        "\nb) Probabilidade de {} não ser selecionada:",
        empresa_nao_selecionada
    );
    if let Some(funcionarios) = empresas.get(empresa_nao_selecionada) {
        let probabilidade = 1.0 - (*funcionarios as f64 / total as f64);

        println!("{:.2}%", probabilidade * 100.0);
    }
}
