use std::collections::BTreeMap;

pub fn executar() {
    let clientes = BTreeMap::from([
        ("A", (0.10, 0.01)),
        ("B", (0.15, 0.02)),
        ("C", (0.15, 0.005)),
        ("D", (0.40, 0.02)),
        ("E", (0.20, 0.08)),
    ]);

    let cliente_analisado = "E";
    let mut probabilidade_erro = 0.0;

    for (_, &(prob_pedido, prob_erro)) in &clientes {
        probabilidade_erro += prob_pedido * prob_erro;
    }

    println!(
        "a) Probabilidade do sistema apresentar erro: {:.2}%",
        probabilidade_erro * 100.0
    );

    println!(
        "\nb) Probabilidade do erro vir do cliente {}:",
        cliente_analisado
    );

    if probabilidade_erro == 0.0 {
        println!("Indefinida: a probabilidade de erro é zero.");
        return;
    }

    if let Some(&(prob_pedido, prob_erro)) = clientes.get(cliente_analisado) {
        let probabilidade = prob_pedido * prob_erro / probabilidade_erro;

        println!("{:.2}%", probabilidade * 100.0);
    }
}
