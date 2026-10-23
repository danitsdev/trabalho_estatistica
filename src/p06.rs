pub fn executar() {
    let total_lavadoras = 813;
    let lavadoras_com_reparo = 504;

    let probabilidade_reparo = lavadoras_com_reparo as f64 / total_lavadoras as f64;
    let probabilidade_nao_reparo = 1.0 - probabilidade_reparo;
    println!(
        "Probabilidade de não exigir reparo: {:.2}%",
        probabilidade_nao_reparo * 100.0
    );
}
