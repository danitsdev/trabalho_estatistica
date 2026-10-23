pub fn executar() {
    let media_liquido = 900.0;
    let desvio_liquido: f64 = 10.0;
    let media_embalagem = 100.0;
    let desvio_embalagem: f64 = 4.0;

    let media_bruto = media_liquido + media_embalagem;
    let desvio_bruto = (desvio_liquido.powi(2) + desvio_embalagem.powi(2)).sqrt();

    println!("Média do peso bruto: {:.2} g", media_bruto);
    println!("Desvio-padrão do peso bruto: {:.2} g", desvio_bruto);
}
