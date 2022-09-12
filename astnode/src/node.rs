use crate::field::Field;
use syn::DeriveInput;
use syn::DataStruct;

pub struct Node {
    is_enum: bool,
    ident: syn::Ident,
    fields: Fields
}

impl From<syn::DeriveInput> for Node {
    fn from (input: syn::DeriveInput)-> Self {
        let (fields, is_enum) = match input.data {
            syn::Data::Struct(data_struct) => (data_struct.into(), false),
            syn::Data::Enum(data_enum) => (data_enum.into(),true),
            syn::Data::Union(_) => unimplemented!("Nodes from unions are not implemented")
        };
        Node {is_enum, fields, ident:input.ident}
    }
}

struct Fields(Vec<Field>);

impl From<syn::DataStruct> for Fields {
    fn from(ds: syn::DataStruct) -> Self{
        let fields = match ds.fields {
            syn::Fields::Named(syn::FieldsNamed {
                named: fields,
                ..
            }) => fields,
            _ => unimplemented!("What to do when fields are not named")
        }.iter().map(|f| f.into()).collect(); 
        return Fields(fields);
    }

}

impl From<syn::Fields> for Fields {
    fn from(fields: syn::Fields) -> Self{
        let v = match fields {
            syn::Fields::Named(syn::FieldsNamed {
                named: fields_named,
                ..
            }) => fields_named.pairs().map(|f| f.into_value().into()).collect(),
            _ => unimplemented!("What to do when fields are not named")
        };
        Fields(v)
    }
}

impl From<syn::DataEnum> for Fields {
    fn from(ds: syn::DataEnum) -> Self{
        unimplemented!();
    }

}



