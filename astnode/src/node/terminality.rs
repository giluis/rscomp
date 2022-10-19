
#[derive(Debug)]
pub enum BranchTerminality {
    Reference,
    Leaf { source: syn::TypePath },
}


pub trait IntoFieldTerminality {
    fn into_field_terminality<'a>(&'a self) -> BranchTerminality
    where
        Self: HasAttributes<'a> + syn::spanned::Spanned + Sized,
    {
        match self
            .get_attrs()
            .filter(|attr| attr.path.segments.len() == 1 && attr.path.segments[0].ident == "leaf")
            .nth(0)
        {
            None => BranchTerminality::Reference,
            Some(a) => BranchTerminality::Leaf {
                source: a
                    .parse_args::<syn::TypePath>()
                    .expect("Could not extract leaf source from attribute"),
            },
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
