pub fn executar() {
    let a: f64 = 0.0;
    let b: f64 = 10.0;
    let c: f64 = 3.0;
    let d: f64 = 7.0;

    if a >= b {
        println!("O limite inferior da distribuição deve ser menor que o superior.");
        return;
    }

    let valor_esperado = (a + b) / 2.0;
    let variancia = (b - a).powi(2) / 12.0;
    let probabilidade = (d.min(b) - c.max(a)).max(0.0) / (b - a);

    println!("Valor esperado: {:.2}", valor_esperado);
    println!("Variância: {:.4}", variancia);
    println!("P({} < X < {}): {:.2}%", c, d, probabilidade * 100.0);
}
