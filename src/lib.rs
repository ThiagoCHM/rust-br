pub mod check {
    pub mod cnpj;
    pub mod cpf;
}
pub use check::cnpj::cnpj;
pub use check::cpf::cpf;
