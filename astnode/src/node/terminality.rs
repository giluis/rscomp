#[derive(Debug)]
pub enum BranchTerminality {
    Reference,
    StatefulLeaf { source: syn::TypePath },
    StatelessLeaf { source: syn::TypePath },
}

pub trait IntoFieldTerminality {
    fn as_field_terminality<'a>(&'a self) -> BranchTerminality
    where
        Self: HasAttributes<'a> + syn::spanned::Spanned + Sized,
    {
        match self
            .get_attrs()
            .find(|attr| /* attr.path.segments.len() == 1 && */ attr.path.segments[0].ident == "stateful_leaf" || attr.path.segments[0].ident == "stateless_leaf" )
        {
            None => BranchTerminality::Reference,
            Some(attr) => {        
                let source = attr.parse_args::<syn::TypePath>()
                         .expect("Could not extract leaf source from attribute");
                if attr.path.segments[0].ident == "stateful_leaf" {
                    BranchTerminality::StatefulLeaf { source }
                } else {
                    BranchTerminality::StatelessLeaf { source }
                }
            }
        }
    }
}

impl<'a> IntoFieldTerminality for &syn::Variant {}
impl<'a> IntoFieldTerminality for &syn::Field {}

pub trait HasAttributes<'a> {
    fn get_attrs(&self) -> std::slice::Iter<'a, syn::Attribute>;
}

impl<'a> HasAttributes<'a> for &'a syn::Field {
    fn get_attrs(&self) -> std::slice::Iter<'a, syn::Attribute> {
        self.attrs.iter()
    }
}

impl<'a> HasAttributes<'a> for &'a syn::Variant {
    fn get_attrs(&self) -> std::slice::Iter<'a, syn::Attribute> {
        self.attrs.iter()
    }
}
