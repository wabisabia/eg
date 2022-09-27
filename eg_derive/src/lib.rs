use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse::{Parse, ParseStream},
    parse2, parse_macro_input, parse_quote,
    spanned::Spanned,
    AttrStyle, Attribute, Data, DeriveInput, Field, Fields, GenericParam, Generics, Ident, LitStr,
    Token, Type,
};

#[proc_macro_derive(Eg, attributes(eg))]
pub fn derive_eg(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let example = example(input.data);

    let expanded = quote! {
        impl #impl_generics eg::Eg for #name #ty_generics #where_clause {
            fn eg() -> Self {
                #example
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(eg::Eg));
        }
    }
    generics
}

fn example(data: Data) -> TokenStream {
    match data {
        Data::Struct(strukt) => match strukt.fields {
            Fields::Unit => quote!(Self),
            Fields::Named(fields) => {
                let field_egs = fields.named.iter().map(create_eg);
                quote!(Self {
                    #(#field_egs)*
                })
            }
            Fields::Unnamed(fields) => {
                let examples = fields.unnamed.iter().map(|field| {
                    let ty = &field.ty;
                    quote_spanned!(field.span() => #ty::eg())
                });
                quote!(Self(#(#examples),*)
                )
            }
        },
        Data::Enum(enumeration) => {
            let mut tagged_variants = enumeration.variants.iter().filter(|variant| {
                variant.attrs.iter().any(|attr| {
                    matches!(attr.style, AttrStyle::Outer)
                        && matches!(attr.path.get_ident(), Some(ident) if ident == "eg")
                })
            });
            let eg_variant = match tagged_variants.next() {
                None => &enumeration.variants[0],
                Some(tagged_variant) => tagged_variant,
            };
            if let Some(_) = tagged_variants.next() {
                panic!("Cannot tag more than one enum variant with `eg`.");
            }
            let eg_variant_ident = &eg_variant.ident;
            match &eg_variant.fields {
                Fields::Unit => quote!(Self::#eg_variant_ident),
                Fields::Unnamed(fields) => {
                    let type_idents = fields.unnamed.iter().map(|field| field.ty.clone());
                    quote!(Self::#eg_variant_ident(#(#type_idents::eg()),*))
                }
                Fields::Named(fields) => {
                    let field_egs = fields.named.iter().map(create_eg);
                    quote!(Self::#eg_variant_ident {
                        #(#field_egs)*
                    })
                }
            }
        }
        Data::Union(_) => unimplemented!(),
    }
}

fn create_eg(field: &Field) -> TokenStream {
    let name = field.ident.as_ref().unwrap();
    let expr = if let Some(attr) = get_attr(field) {
        custom_expr(field, attr)
    } else {
        let ty = &field.ty;
        quote! {
            #ty::eg()
        }
    };
    quote! {
        #name: #expr,
    }
}

fn get_attr(field: &Field) -> Option<&Attribute> {
    field.attrs.iter().find_map(|attr| {
        if !matches!(attr.style, AttrStyle::Outer) {
            return None;
        }
        (attr.path.get_ident()?.to_string().as_str() == "eg").then(|| attr)
    })
}

fn custom_expr(field: &Field, attr: &Attribute) -> TokenStream {
    let custom_eg: CustomExpr = parse2(attr.tokens.clone()).unwrap();
    let literal = custom_eg.string;
    if let Type::Path(type_path) = &field.ty {
        if let Some(ty_ident) = type_path.path.get_ident() {
            if ty_ident == "String" {
                return quote! {
                    #literal.to_string()
                };
            }
        }
    };
    let expr: TokenStream = literal.value().parse().unwrap();
    if let Ok(ident) = parse2::<Ident>(expr.clone()) {
        quote! {
            #ident()
        }
    } else {
        expr
    }
}

struct CustomExpr {
    string: LitStr,
}

impl Parse for CustomExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        Ok(Self {
            string: input.parse()?,
        })
    }
}
