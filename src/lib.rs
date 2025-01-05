pub mod check {
    pub mod cnh;
    pub mod cnpj;
    pub mod cpf;
    pub mod passport;
}
pub use check::cnh::cnh;
pub use check::cnpj::cnpj;
pub use check::cpf::cpf;
pub use check::passport::passport;
