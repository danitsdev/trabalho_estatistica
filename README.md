# Probabilidade e Estatística — Trabalho 1

Resolução das 19 questões da [Lista de Exercícios](Trabalho1.pdf) em Rust. Cada questão está em seu próprio arquivo, de `src/p01.rs` a `src/p19.rs`, com uma função `executar()`.

Deixei os valores no código para poupar a digitação na correção. Para testar outros dados, basta editar as variáveis, geralmente no início de cada arquivo, salvar e rodar `cargo run` novamente.

## Requisitos

Instale [Rust 1.89 ou superior com Cargo](https://www.rust-lang.org/tools/install), seguindo os pré-requisitos do seu sistema. A biblioteca `statrs` é baixada automaticamente na primeira execução, que precisa de internet.

## Execução

Baixe o repositório e abra o terminal na pasta que contém `Cargo.toml`.

| Comando | Execução |
| --- | --- |
| `cargo run` | Todas as questões |
| `cargo run -- --9` | Somente a questão 9 |

Troque `9` pelo número desejado, de 1 a 19.

## Compilar e executar diretamente

Para gerar o executável, use `cargo build --release`. Depois, execute:

`./target/release/estatistica_problemas`

Acrescente `--9`, por exemplo, para executar somente a questão 9. Se alterar os valores no código, compile novamente antes de usar o executável.
