use crate::util::ty_inner_type;

#[derive(Debug)]
pub enum Descriptor {
    Optional(syn::Type),   //inner option type
    Repeatable(syn::Type), // inner vec type
    Bare(syn::Type),
}
impl Descriptor {
    pub fn get_inner_ty(&self) -> &syn::Type {
        match self {
            Self::Optional(ty) => ty,
            Self::Repeatable(ty) => ty,
            Self::Bare(ty) => ty,
        }
    }
}

impl From<syn::Type> for Descriptor {
    fn from(ty: syn::Type) -> Self {
        match ty.extract_if_optional()
        .or_else(||ty.extract_if_repeatable()){
            Some(t) => t,
            None => Descriptor::Bare(ty)
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
        .map(|t|Descriptor::Repeatable(t))
        .or(None)

    }

    // TODO: how to remove clone from here
    fn extract_if_optional(self) -> Option<Descriptor> {
        ty_inner_type("Option", self.clone())
        .map(|t|Descriptor::Optional(t))
        .or(None)
    }
}