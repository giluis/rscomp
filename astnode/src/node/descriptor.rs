use crate::util::ty_inner_type;

#[derive(Debug)]
pub enum Descriptor {
    Optional(syn::Type),   //inner option type
    Repeatable(syn::Type), // inner vec type
    Bare(syn::Type),
}

impl From<syn::Type> for Descriptor {
    fn from(ty: syn::Type) -> Self {
        match ty.extract_if_optional()
        .or_else(||ty.extract_if_repeatable()){
            Some(t) => t,
            None => Descriptor::Bare(ty.clone())
        }
    }
}

impl From<&syn::Type> for Descriptor {
    fn from(ty: &syn::Type) -> Self {
        match ty.extract_if_optional()
        .or_else(||ty.extract_if_repeatable()){
            Some(t) => t,
            None => Descriptor::Bare(ty.clone())
        }
    }
}
trait ModifierExtractable<'a> {
    fn extract_if_repeatable(self) -> Option<Descriptor>;

    fn extract_if_optional(self) -> Option<Descriptor>;
}

impl <'a> ModifierExtractable<'a> for &syn::Type {
    // TODO: how to remove clone from here
    fn extract_if_repeatable(self) -> Option<Descriptor> {
        ty_inner_type("Vec", self.clone())
        .and_then(|t|Some(Descriptor::Repeatable(t.clone())))
        .or(None)

    }

    // TODO: how to remove clone from here
    fn extract_if_optional(self) -> Option<Descriptor> {
        ty_inner_type("Option", self.clone())
        .and_then(|t|Some(Descriptor::Optional(t.clone())))
        .or(None)
    }
}