/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive_maybe_eq(input: proc_macro::TokenStream, should_eq: bool) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let name = &input.ident;
    let gen = if should_eq {
        quote! {
            impl #impl_generics gazebo::cmp::MaybeEq for #name #ty_generics #where_clause {
                fn is_comparable() -> bool {
                    true
                }

                fn get_comparable_any(this: &Self) -> gazebo::cmp::PartialEqAny {
                    gazebo::cmp::PartialEqAny::new(this)
                }
            }
        }
    } else {
        quote! {
            impl #impl_generics gazebo::cmp::MaybeEq for #name #ty_generics #where_clause {}
        }
    };
    gen.into()
}
