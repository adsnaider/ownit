use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ConstParam, DeriveInput, TypeParam};

#[proc_macro_derive(Burrow)]
pub fn derive_burrow(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let lifetimes: Vec<_> = item.generics.lifetimes().collect();

    let name = item.ident;
    let syn::Data::Struct(strct) = item.data else {
        unimplemented!("Only structs are supported");
    };
    let syn::Fields::Named(fields) = strct.fields else {
        unimplemented!("Only named structs are supported");
    };

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
