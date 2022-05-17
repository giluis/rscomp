// Resources:
//
//   - The Quote crate for putting together output from a macro:
//     https://github.com/dtolnay/quote
//
//   - Joining together the type name + "Builder" to make the builder's name:
//     https://docs.rs/syn/1.0/syn/struct.Ident.html

use astnode::AstNode;


pub struct AssignStatement {
    type: Type,
    ident: Ident,

    #[ast(from = Token::EqualSign)]
    : String



}


#[derive(AstNode)]
pub struct Ident {
    #[ast(from = Token::Ident)]
    value: String
}

fn main() {
}
