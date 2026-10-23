mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;
mod p07;
mod p08;
mod p09;
mod p10;
mod p11;
mod p12;
mod p13;
mod p14;
mod p15;
mod p16;
mod p17;
mod p18;
mod p19;

fn main() {
    let mut questao = None;
    for argumento in std::env::args().skip(1) {
        match argumento
            .strip_prefix("--")
            .unwrap_or(&argumento)
            .parse::<usize>()
        {
            Ok(numero) if (1..=19).contains(&numero) && questao.is_none() => {
                questao = Some(numero);
            }
            _ => {
                eprintln!("Informe uma questão de --1 a --19.");
                std::process::exit(1);
            }
        }
    }

    println!("Probabilidade e Estatística - Trabalho 1");

    let questoes: [fn(); 19] = [
        p01::executar,
        p02::executar,
        p03::executar,
        p04::executar,
        p05::executar,
        p06::executar,
        p07::executar,
        p08::executar,
        p09::executar,
        p10::executar,
        p11::executar,
        p12::executar,
        p13::executar,
        p14::executar,
        p15::executar,
        p16::executar,
        p17::executar,
        p18::executar,
        p19::executar,
    ];

    for (indice, executar) in questoes.iter().enumerate() {
        let numero = indice + 1;
        if questao.is_none() || questao == Some(numero) {
            println!("\nProblema {:02}:", numero);
            executar();
        }
    }
}
