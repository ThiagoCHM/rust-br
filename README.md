# Rust-BR

**Rust-BR** é uma crate robusta e versátil desenvolvida para validar uma ampla gama de documentos e informações brasileiras, incluindo CPF, CNPJ, e muito mais, de maneira simples e eficiente.

## Descrição
Inicialmente concebida para lidar com a validação de documentos essenciais como CPF, CNPJ e CNH, a Rust-BR se destaca como uma ferramenta indispensável para aplicações que envolvem o cadastro de pessoas físicas e jurídicas no Brasil.

## Expansão de Funcionalidades
Reconhecendo a crescente demanda por validações abrangentes, expandimos suas capacidades para incluir não apenas documentos, mas também e-mails, telefones e URLs. Todas essas funcionalidades são implementadas puramente com Rust Vanilla, sem a adição de dependências externas.

## Importante
É crucial destacar que a Rust-BR não verifica a associação de documentos a indivíduos específicos. Nossa função é assegurar que os dados fornecidos estejam em conformidade com os padrões estabelecidos pelos órgãos reguladores pertinentes a cada tipo de documento.

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

## Lista de Validadores

 • E-mail
 • Telefone
 • URL

## Instalação

Para usar o **Rust-BR** em seu projeto, adicione a seguinte dependência ao seu `Cargo.toml`:

```toml
[dependencies]
rust-br = "1.0.1"
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
### Verificação de E-mail
```
use rust_br::mail;

fn main() {
    let email1 = "usuario@example.com";
    let email2 = "usuario@exemplo";

    if mail(email1) {
        println!("'{}' é um e-mail válido!", email1);
    } else {
        println!("'{}' é um e-mail inválido.", email1);
    }

    if mail(email2) {
        println!("'{}' é um e-mail válido!", email2);
    } else {
        println!("'{}' é um e-mail inválido.", email2);
    }
}
```
### Verificação de Telefones Válidos (Identifica DDD e Área)
```
 // Número Fixo Válido com DDD de São Paulo

use rust_br::phone;

fn main() {
    let numero_telefone = "1134567890";
    let resultado = phone(numero_telefone);

    if resultado.is_valid {
        println!("Número de telefone válido!");
        println!("Estado: {}", resultado.state.unwrap());
        println!("Região: {}", resultado.region.unwrap());
    } else {
        println!("Número de telefone inválido.");
    }
}
```


```
// Número Móvel Válido com DDD de Santa Catarina

use rust_br::phone;

fn main() {
    let numero_telefone = "48987654321";
    let resultado = phone(numero_telefone);

    if resultado.is_valid {
        println!("Número de telefone válido!");
        println!("Estado: {}", resultado.state.unwrap());
        println!("Região: {}", resultado.region.unwrap());
    } else {
        println!("Número de telefone inválido.");
    }
}
```
### Verificação de URL
```
use rust_br::url;

fn main() {
    let url1 = "https://www.example.com";
    let url2 = "ftp://example.com";
    let url3 = "http://invalid_domain";

    if url(url1) {
        println!("'{}' é uma URL válida!", url1);
    } else {
        println!("'{}' é uma URL inválida.", url1);
    }

    if url(url2) {
        println!("'{}' é uma URL válida!", url2);
    } else {
        println!("'{}' é uma URL inválida.", url2);
    }

    if url(url3) {
        println!("'{}' é uma URL válida!", url3);
    } else {
        println!("'{}' é uma URL inválida.", url3);
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
