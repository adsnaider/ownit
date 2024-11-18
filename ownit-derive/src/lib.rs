use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, ConstParam, DataStruct, DeriveInput, TypeParam, Variant};

#[proc_macro_derive(Ownit)]
pub fn derive_ownit(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let lifetimes: Vec<_> = item.generics.lifetimes().collect();

    let name = item.ident;
    match item.data {
        syn::Data::Struct(DataStruct { fields, .. }) => match fields {
            syn::Fields::Named(fields) => {
                let fields = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
                generate_for_named_struct(
                    &name,
                    lifetimes.len(),
                    &item.generics.type_params().collect::<Vec<_>>(),
                    &item.generics.const_params().collect::<Vec<_>>(),
                    &fields.collect::<Vec<_>>(),
                )
                .into()
            }
            syn::Fields::Unnamed(fields) => {
                let fields = fields.unnamed.len();
                generate_for_tuple_struct(
                    &name,
                    lifetimes.len(),
                    &item.generics.type_params().collect::<Vec<_>>(),
                    &item.generics.const_params().collect::<Vec<_>>(),
                    fields,
                )
                .into()
            }
            syn::Fields::Unit => {
                generate_for_unit_struct(&name, &item.generics.const_params().collect::<Vec<_>>())
                    .into()
            }
        },
        syn::Data::Enum(enm) => {
            let variants = enm.variants.iter().collect::<Vec<_>>();
            generate_for_enum(
                &name,
                lifetimes.len(),
                &item.generics.type_params().collect::<Vec<_>>(),
                &item.generics.const_params().collect::<Vec<_>>(),
                &variants,
            )
            .into()
        }
        syn::Data::Union(_) => panic!("Ownit may not be implemented for `union` types"),
    }
}

#[derive(Debug, Copy, Clone)]
struct Void;

impl ToTokens for Void {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        // Purposely empty
    }
}

fn generate_for_named_struct(
    name: &Ident,
    lifetimes: usize,
    generics: &[&TypeParam],
    const_params: &[&ConstParam],
    fields: &[&Ident],
) -> TokenStream {
    let life: Vec<_> = std::iter::repeat(Void).take(lifetimes).collect();
    if !const_params.is_empty() {
        unimplemented!("const params are not yet supported");
    }

    let generic_names: Vec<_> = generics.iter().map(|g| &g.ident).collect();
    quote! {
        impl<#(#generics + 'static,)*>::ownit::Ownit for #name<#(#life '_,)*#(#generic_names,)*> {
            type OwnedSelf = #name<#(#life 'static,)*#(#generic_names,)*>;

            fn into_static(self) -> Self::OwnedSelf {
                use ::ownit::Ownit;
                #name {
                    #(#fields: self.#fields.into_static(),)*
                }
            }
        }
    }
}

fn generate_for_tuple_struct(
    name: &Ident,
    lifetimes: usize,
    generics: &[&TypeParam],
    const_params: &[&ConstParam],
    fields: usize,
) -> TokenStream {
    let life: Vec<_> = std::iter::repeat(Void).take(lifetimes).collect();
    if !const_params.is_empty() {
        unimplemented!("const params are not yet supported");
    }
    let fields = (0..fields).map(syn::Index::from);
    let generic_names: Vec<_> = generics.iter().map(|g| &g.ident).collect();
    quote! {
        impl<#(#generics + 'static,)*> ::ownit::Ownit for #name<#(#life '_,)*#(#generic_names,)*> {
            type OwnedSelf = #name<#(#life 'static,)*#(#generic_names,)*>;

            fn into_static(self) -> Self::OwnedSelf {
                use ::ownit::Ownit;
                #name (
                    #(self.#fields.into_static(),)*
                )
            }
        }
    }
}

fn generate_for_unit_struct(name: &Ident, const_params: &[&ConstParam]) -> TokenStream {
    if !const_params.is_empty() {
        unimplemented!("const params are not yet supported");
    }
    quote! {
        impl ::ownit::Ownit for #name {
            type OwnedSelf = #name;

            fn into_static(self) -> Self::OwnedSelf {
                #name
            }
        }
    }
}

struct MatchArm<'a> {
    variant: &'a Variant,
}

impl ToTokens for MatchArm<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.variant.ident;
        let lhs = {
            match &self.variant.fields {
                syn::Fields::Named(f) => {
                    let fields = f.named.iter().map(|f| f.ident.as_ref().unwrap());
                    quote! {
                        {#(#fields,)*}
                    }
                }
                syn::Fields::Unnamed(f) => {
                    let fields = f.unnamed.iter().count();
                    let names =
                        (0..fields).map(|i| syn::Ident::new(&format!("x{i}"), Span::call_site()));
                    quote! {
                        (#(#names,)*)
                    }
                }
                syn::Fields::Unit => quote! {},
            }
        };
        let rhs = {
            match &self.variant.fields {
                syn::Fields::Named(f) => {
                    let fields = f.named.iter().map(|f| f.ident.as_ref().unwrap());
                    quote! {
                        {#(#fields: #fields.into_static(),)*}
                    }
                }
                syn::Fields::Unnamed(f) => {
                    let fields = f.unnamed.iter().count();
                    let names =
                        (0..fields).map(|i| syn::Ident::new(&format!("x{i}"), Span::call_site()));
                    quote! {
                        (#(#names.into_static(),)*)
                    }
                }
                syn::Fields::Unit => quote! {},
            }
        };
        tokens.append_all(quote! {
            Self::#name #lhs => Self::OwnedSelf::#name #rhs
        })
    }
}

fn generate_for_enum(
    name: &Ident,
    lifetimes: usize,
    generics: &[&TypeParam],
    const_params: &[&ConstParam],
    variants: &[&Variant],
) -> TokenStream {
    let life: Vec<_> = std::iter::repeat(Void).take(lifetimes).collect();
    if !const_params.is_empty() {
        unimplemented!("const params are not yet supported");
    }
    let generic_names: Vec<_> = generics.iter().map(|g| &g.ident).collect();
    let matches = variants.iter().map(|variant| MatchArm { variant });
    quote! {
        impl<#(#generics + 'static,)*> ::ownit::Ownit for #name<#(#life '_,)*#(#generic_names,)*> {
            type OwnedSelf = #name<#(#life 'static,)*#(#generic_names,)*>;

            fn into_static(self) -> Self::OwnedSelf {
                use ::ownit::Ownit;

                match self {
                    #(#matches,)*
                }
            }
        }
    }
}
