pub fn executar() {
    let populacao: u32 = 52;
    let grupos: u32 = 4;

    if populacao == 0 || grupos == 0 {
        println!("Erro: os valores devem ser maiores que zero.");
        return;
    }

    if populacao % grupos != 0 {
        println!("Erro: a população deve ser divisível igualmente pelos grupos.");
        return;
    }

    let elementos_por_grupo = populacao / grupos;
    let probabilidade_por_grupo = elementos_por_grupo as f64 / populacao as f64;
    let probabilidade_nao_pertencer = 1.0 - probabilidade_por_grupo;

    println!("População: {}", populacao);
    println!("Quantidade de grupos: {}", grupos);
    println!("Elementos por grupo: {}", elementos_por_grupo);

    println!(
        "Probabilidade de pertencer ao grupo: {:.2}%",
        probabilidade_por_grupo * 100.0
    );

    println!(
        "Probabilidade de não pertencer ao grupo: {:.2}%",
        probabilidade_nao_pertencer * 100.0
    );
}
