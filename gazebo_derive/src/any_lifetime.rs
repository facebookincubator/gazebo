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

pub fn derive_any_lifetime(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // We can deal with exactly:
    // Foo<'v> OR Foo
    // So see which case we have and abort otherwise
    if input.generics.const_params().count() > 0
        || input.generics.type_params().count() > 0
        || input.generics.lifetimes().count() > 1
    {
        return syn::Error::new_spanned(
            input.generics,
            "Can't derive AnyLifetime for types with type parameters or more than one lifetime",
        )
        .into_compile_error()
        .into();
    }

    let has_lifetime = input.generics.lifetimes().count() == 1;
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let gen = if has_lifetime {
        quote! {
            unsafe impl #impl_generics gazebo::any::AnyLifetime #impl_generics for #name #ty_generics #where_clause {
                fn static_type_id() -> std::any::TypeId {
                    std::any::TypeId::of::<#name<'static>>()
                }

                fn static_type_of(&self) -> std::any::TypeId {
                    Self::static_type_id()
                }
            }
        }
    } else {
        quote! {
            unsafe impl<'a> gazebo::any::AnyLifetime<'a> for #name #ty_generics #where_clause {
                fn static_type_id() -> std::any::TypeId {
                    std::any::TypeId::of::<#name>()
                }

                fn static_type_of(&self) -> std::any::TypeId {
                    Self::static_type_id()
                }
            }
        }
    };
    gen.into()
}
