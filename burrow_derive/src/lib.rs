use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ConstParam, DataStruct, DeriveInput, TypeParam};

#[proc_macro_derive(Burrow)]
pub fn derive_burrow(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
            syn::Fields::Unit => todo!(),
        },
        syn::Data::Enum(enm) => todo!(),
        syn::Data::Union(_) => panic!("Burrow may not be implemented for `union` types"),
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
    quote! {
        impl ::burrow::Burrow for #name<#(#life '_,)*#(#generics,)*> {
            type OwnedSelf = #name<#(#life 'static,)*#(#generics,)*>;

            fn into_static(self) -> Self::OwnedSelf {
                use ::burrow::Burrow;
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
    quote! {
        impl ::burrow::Burrow for #name<#(#life '_,)*#(#generics,)*> {
            type OwnedSelf = #name<#(#life 'static,)*#(#generics,)*>;

            fn into_static(self) -> Self::OwnedSelf {
                use ::burrow::Burrow;
                #name (
                    #(self.#fields.into_static(),)*
                )
            }
        }
    }
}
