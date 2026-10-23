pub fn executar() {
    let civis: u32 = 30;
    let outras_engenharias: u32 = 10;
    let selecoes = [(2), (4)];
    let total: u32 = civis + outras_engenharias;

    for n in selecoes {
        let mut probabilidade = 1.0;

        if n > civis {
            println!("n = {}", n);
            println!("Probabilidade = 0.00%\n");
            continue;
        }

        for i in 0..n {
            probabilidade *= (civis - i) as f64 / (total - i) as f64;
        }

        println!("n = {}", n);
        println!("Probabilidade = {:.2}%\n", probabilidade * 100.0);
    }
}
