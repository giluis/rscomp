use crate::field::Field;
use syn::DataStruct;
use syn::DeriveInput;
use syn::TypePath;

pub enum NodeType {
    ProductNode,
    SumNode,
}

struct Fields(Vec<Field>);

pub struct Node {
    node_type: NodeType,
    ident: syn::Ident,
    fields: Fields,
}

impl From<syn::DeriveInput> for Node {
    fn from(derive_input: syn::DeriveInput) -> Self {
        let (fields, node_type) = match derive_input.data {
            syn::Data::Struct(data_struct) => (data_struct.into(), NodeType::ProductNode),
            syn::Data::Enum(data_enum) => (data_enum.into(), NodeType::SumNode),
            _ => unimplemented!("Nodes from unions are not implemented"),
        };
        Node {
            fields,
            node_type,
            ident: derive_input.ident,
        }
    }
}



impl From<syn::DataStruct> for Fields {
    fn from(ds: syn::DataStruct) -> Self {
        let fields = match ds.fields {
            syn::Fields::Named(syn::FieldsNamed { named: fields, .. }) => fields,
            _ => unimplemented!("What to do when fields are unnamed"),
        }
        .iter()
        .map(|f| f.into())
        .collect();
        return Fields(fields);
    }
}

impl From<syn::Fields> for Fields {
    fn from(fields: syn::Fields) -> Self {
        let v = match fields {
            syn::Fields::Named(syn::FieldsNamed {
                named: fields_named,
                ..
            }) => fields_named
                .pairs()
                .map(|f| f.into_value().into())
                .collect(),
            _ => unimplemented!("What to do when fields are not named"),
        };
        Fields(v)
    }
}

impl From<syn::DataEnum> for Fields {
    fn from(ds: syn::DataEnum) -> Self {
        unimplemented!();
    }
}
