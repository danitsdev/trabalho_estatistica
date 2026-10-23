pub fn executar() {
    let maquinas = [("A", 0.25, 0.05), ("B", 0.35, 0.04), ("C", 0.40, 0.02)];

    let mut probabilidade_defeito = 0.0;

    for &(_, producao, defeitos) in &maquinas {
        probabilidade_defeito += producao * defeitos;
    }

    if probabilidade_defeito == 0.0 {
        println!("Não há defeitos; a probabilidade condicional é indefinida.");
        return;
    }

    for &(nome, producao, defeitos) in &maquinas {
        let probabilidade = producao * defeitos / probabilidade_defeito;

        println!("Máquina {}: {:.2}%", nome, probabilidade * 100.0);
    }
}
