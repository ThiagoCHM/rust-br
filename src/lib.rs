pub mod check {
    pub mod cnh;
    pub mod cnpj;
    pub mod cpf;
    pub mod mail;
    pub mod passport;
    pub mod phone;
    pub mod url;
}
pub use check::cnh::cnh;
pub use check::cnpj::cnpj;
pub use check::cpf::cpf;
pub use check::mail::mail;
pub use check::passport::passport;
pub use check::phone::phone;
pub use check::url::url;
