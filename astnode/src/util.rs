pub fn ty_inner_type<'a>(wrapper: &str, ty: syn::Type) -> Option<syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != wrapper {
            None
        } else if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None
            }

            let inner_ty = inner_ty.args.first().unwrap();
            if let syn::GenericArgument::Type(ref t) = inner_ty {
                Some(t.clone())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

pub trait UnzippableToVec<T, P> {
    fn unzip_to_vec(self) -> (Vec<T>, Vec<P>);
}

impl<T, P, I> UnzippableToVec<T, P> for I
where
    I: Iterator<Item = (T, P)>,
{
    fn unzip_to_vec(self) -> (Vec<T>, Vec<P>) {
        let a: (Vec<_>, Vec<_>) = self.unzip();
        return a;
    }
}
