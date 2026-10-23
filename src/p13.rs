use statrs::distribution::{Discrete, Poisson};

pub fn executar() {
    let taxa_por_minuto = 5.0;
    let casos = [("a", 60.0, 2), ("b", 30.0, 1)];

    for (letra, segundos, chegadas) in casos {
        let media = taxa_por_minuto * segundos / 60.0;
        let distribuicao = Poisson::new(media).unwrap();
        let probabilidade = distribuicao.pmf(chegadas);

        println!(
            "{}) {} chegada(s) em {:.0} s: {:.2}%",
            letra,
            chegadas,
            segundos,
            probabilidade * 100.0
        );
    }
}
