# Rust-BR

**Rust-BR** é uma crate desenvolvida para validar documentos brasileiros, como CPF e CNPJ, de forma simples e eficiente.

## Descrição

Esta crate fornece funções para validar números de CPF, CNPJ, CNH, dentre outros, essenciais para aplicações que lidam com cadastros de pessoas físicas e jurídicas no Brasil.

## Instalação

Para usar o **Rust-BR** em seu projeto, adicione a seguinte dependência ao seu `Cargo.toml`:

```toml
[dependencies]
rust-br = "0.3.2"
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

## Contribuindo
Contribuições são bem-vindas! Se você encontrar algum problema ou tiver sugestões de melhorias, sinta-se à vontade para abrir uma issue ou enviar um pull request.

## Licença
Este projeto está licenciado sob a Licença MIT

## Contato
Autor: Thiago C H Moreira

LinkedIn: https://www.linkedin.com/in/thiagochmoreira/
