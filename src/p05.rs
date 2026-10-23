pub fn executar() {
    let prob_joao = 0.5;
    let prob_maria = 0.7;

    let prob_joao_falhar = 1.0 - prob_joao;
    let prob_maria_falhar = 1.0 - prob_maria;
    let prob_ambos_falhar = prob_joao_falhar * prob_maria_falhar;
    let prob_alguem_resolver = 1.0 - prob_ambos_falhar;

    println!(
        "Probabilidade do problema ser resolvido: {:.2}%",
        prob_alguem_resolver * 100.0
    );
}
