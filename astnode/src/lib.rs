use syn::{parse_macro_input, DeriveInput};
use libcomp::token::Token;
use quote::*;


fn ty_inner_type<'a>(wrapper: &'a str, ty: &'a syn::Type) -> Option<syn::Type>{
    if let syn::Type::Path(ref p )  = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != wrapper {
            return None;
        }
        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            let inner_ty = inner_ty.args.first().unwrap();
            if let syn::GenericArgument::Type(syn::Type::Path(t)) =  inner_ty {
                return Some(t.clone().into());
            }
        }
    }
    return None;
}


fn build_fn<'a,I>(target_name:&syn::Ident, fields:I) -> proc_macro2::TokenStream 
where I: Iterator<Item = &'a Field> {
    let mut field_names:Vec<&syn::Ident> = vec![];
    let mut match_statements = vec![];
    for f in fields {
        let field_name = &f.field_name;
        field_names.push (field_name);
        let namestr = field_name.to_string();
        let missing_error = format!("field {} is missing",namestr);
        match_statements.push(
            if f.is_optional || f.is_repeatable {
                quote!{
                    let #field_name = self.#field_name.clone();
                }
            } else {
                quote!{
                    let #field_name = self.#field_name.take().ok_or_else(|| #missing_error.to_string())?;
                }
            }
        )
    };

    quote!{
        pub fn build(&mut self) -> Result<#target_name, String> {
            #(#match_statements)*
            Ok(#target_name {
                #(#field_names),*
            })
        }
    }
}




fn get_names(ast: &DeriveInput) -> (syn::Ident, syn::Ident) {
    let name = &ast.ident;
    let mut namestr = name.to_string();
    namestr.push_str("Builder");
    (name.clone(), syn::Ident::new(&namestr,name.span()))
}


struct Field {
    field_name: syn::Ident,
    field_type: syn::Type,
    optional_type: Option<syn::Type>,
    repeatable_type: Option<syn::Type>, 
}

fn error(f: &syn::Field, msg: &'static str)->syn::Error {
    syn::Error::new(f.ident.clone().unwrap().span(),msg)
}

impl Field {
    fn new(f: &syn::Field)->Result<Self, syn::Error>{
        let ident =  match &f.ident {
            Some(i) => i.clone(),
            None => unimplemented!("Fields with no identifiers have not been implemented yet"),
        };
        let ( setter_name, inner_ty, is_optional, is_repeatable ) =  if let Some(setter_name) = repeatable(f){
            let inner_ty = ty_inner_type("Vec", &f.ty).ok_or(error(f,"A repeatable type must always exist inside of a vector"))?;
            ( setter_name , inner_ty,  false, true)
        } else  if is_field_optional(f) {
            let inner_ty = ty_inner_type("Option", &f.ty).ok_or(error(f,"A type must always exist inside of a option"))?;
            (ident.clone(), inner_ty, true, false) 
        } else {
            (ident.clone(), f.ty.clone(), false, false)
        };
        Ok ( Field {
            field_name: ident,
            field_type:f.ty,
            optional_type: optional(f),
            repeatable_type: repeatable(f),
        } )
    }
}


/* impl syn::parse::Parse for BuilderEachAttrName {
    fn parse(tokens: syn::parse::ParseStream) -> Result<BuilderEachAttrName, syn::Error>{

        let a: proc_macro::Group = tokens.parse()?;

    }
} */

// fn repeatable(f: &syn::Field) -> Option<>

// fn repeatable(f: &syn::Field ) -> Option<syn::Ident> {
//     for attr in f.attrs.iter(){
//         if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "builder" {
//             let next = attr.clone().tokens.into_iter().next();
//             if let Some(proc_macro2::TokenTree::Group(g)) = next{
//                 let mut giter = g.stream().into_iter();
//                 let _each = giter.next();
//                 let _equalsign = giter.next();
//                 let arg = match giter.next().unwrap(){
//                     proc_macro2::TokenTree::Literal(l) => l,
//                     tt => panic!("Expected string, found {}", tt),
//                 };
//                 match syn::Lit::new(arg) {
//                     syn::Lit::Str(s) => {
//                         return Some(syn::Ident::new( &s.value(), s.span() ));
//                     },
//                     lit => panic!("Expected string, found {:?}", lit),
//                 };
//
//             }
//         }
//     };
//     return None;
// }
//
fn builder_declaration_field(f: &Field) -> proc_macro2::TokenStream{
        let Field{field_name, inner_type,field_type, is_repeatable, is_optional, ..} = f;
        let assignment = if * is_repeatable {
            quote!{Vec<#inner_type>}
        } else if * is_optional {
            quote!{Option<#inner_type>}
        } else {
            quote!{Option<#field_type>}
        };
        quote!{
            #field_name: #assignment
        }
}

fn builder_declaration<'a,I>(builder_name:&syn::Ident, fields:I) -> proc_macro2::TokenStream where I: Iterator<Item = &'a Field> {
    let field_declarations: Vec<proc_macro2::TokenStream> = fields.map(|f|builder_declaration_field(f)).collect();
    quote!{
        pub struct #builder_name {
            #(#field_declarations),*
        }
    }
}


#[proc_macro_derive(AstNode, attributes(ast))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let (target_name, builder_name) = get_names(&ast);
    let fields = match get_fields(&ast){
        Ok(f) => f,
        Err(r) => panic!("{}",r)
    };  

    let builder_fn = build_fn(&target_name, fields.iter());
    let builder_declaration = builder_declaration(&builder_name, fields.iter());
     quote!{ 

         #builder_declaration


        impl #builder_name {

            #builder_fn
        }

    }.into()
}

fn get_fields<'a>(ast: &DeriveInput) -> Result<Vec<Field>, syn::Error> {
    let raw_fields = match &ast.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(syn::FieldsNamed {
                    named: fields,
                    ..
                }),
                ..
            }) => fields,
            _ => panic!()
        };
    raw_fields.iter().map(|f|Field::new(f)).collect()
}

fn is_field_optional(f:&syn::Field ) -> bool {
    let op = ty_inner_type("Option",&f.ty); 
    op.is_some()
}
