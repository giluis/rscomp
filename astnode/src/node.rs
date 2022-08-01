use crate::field::{Field, FieldType};

pub struct Node {
    is_enum: bool,
    ident: syn::Ident,
    fields: Vec<Field>
}

