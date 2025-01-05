# Rust-BR

**Rust-BR** é uma crate desenvolvida para validar documentos brasileiros, como CPF e CNPJ, de forma simples e eficiente.

## Descrição

Esta crate fornece funções para validar documentos de CPF, CNPJ, CNH, dentre outros, essenciais para aplicações que lidam com cadastros de pessoas físicas e jurídicas no Brasil. 
Vale a observação, não verificamos se determinado documento pertence a um certo indivíduo, mas se os dados inseridos atendem aos padrões estabelecidos pelos Orgãos Reguladores de cada um deste documentos.

## Lista de Documentos

 • CPF
 • CNPJ
 • CNH
 • Passaporte Brasileiro*
 • CTPS (Em Breve)
 • Título de Eleitor (Em Breve)
 • Certidão de Nascimento (Em Breve)
 • Certidão de Casamento (Em Breve)
 • Certidão de Óbito (Em Breve)

 * Aceita a entrada de caracteres maiúsculos e minusculos.

## Instalação

Para usar o **Rust-BR** em seu projeto, adicione a seguinte dependência ao seu `Cargo.toml`:

```toml
[dependencies]
rust-br = "0.4.0"
```

## Como Usar
A crate oferece funções para verificar a validade de CPF e CNPJ. Aqui estão alguns exemplos de como usá-las:

### Verificação de CPF
```
use rust_br::cpf;

fn main() {
    let numero_cpf = "12345678909";
    if cpf(numero_cpf) {
        println!("CPF válido!");
    } else {
        println!("CPF inválido.");
    }
}
```
### Verificação de CNPJ
```
use rust_br::cnpj;

fn main() {
    let numero_cnpj = "11444777000161";
    if cnpj(numero_cnpj) {
        println!("CNPJ válido!");
    } else {
        println!("CNPJ inválido.");
    }
}
```
### Verificação de CNH
```
use rust_br::cnh;

fn main() {
    let numero_cnh = "12345678910";
    if cnh(numero_cnh) {
        println!("CNH válido!");
    } else {
        println!("CNH inválido.");
    }
}
```
### Verificação de Passaporte Brasileiro
```
use rust_br::passport;

fn main() {
    let passaporte = "AA112233";
    if cnh(passaporte) {
        println!("Passaporte Brasileiro válido!");
    } else {
        println!("Passaporte Brasileiro inválido.");
    }
}
```

## Contribuindo
Contribuições são bem-vindas! Se você encontrar algum problema ou tiver sugestões de melhorias, sinta-se à vontade para abrir uma issue ou enviar um pull request.

## Licença
Este projeto está licenciado sob a Licença MIT

## Contato
Autor: Thiago C H Moreira

LinkedIn: https://www.linkedin.com/in/thiagochmoreira/
