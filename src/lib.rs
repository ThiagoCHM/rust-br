pub mod check {
    pub mod cnh;
    pub mod cnpj;
    pub mod cpf;
}
pub use check::cnh::cnh;
pub use check::cnpj::cnpj;
pub use check::cpf::cpf;
