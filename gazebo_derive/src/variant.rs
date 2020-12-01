/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn derive_variant_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Enum(data_enum) = input.data {
        let mut variant_body = Vec::new();
        for variant in data_enum.variants {
            let variant_name = &variant.ident;
            let patterns = match variant.fields {
                Fields::Unit => quote! {},
                Fields::Named(_) => quote! { {..} },
                Fields::Unnamed(_) => quote! { (..) },
            };
            let variant_name_str = variant_name.to_string();
            variant_body.push(quote! {
                Self::#variant_name#patterns => #variant_name_str
            });
        }

        let name = &input.ident;
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        let gen = quote! {
            impl #impl_generics gazebo::variants::VariantName for #name #ty_generics #where_clause {
                fn variant_name(&self) -> &'static str {
                    match self {
                        #(#variant_body,)*
                    }
                }
            }
        };

        gen.into()
    } else {
        panic!("Can only derive variant name on enums")
    }
}
