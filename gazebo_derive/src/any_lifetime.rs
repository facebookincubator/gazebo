/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::fmt::Display;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn derive_provides_static_type(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    fn _error<T: quote::ToTokens, U: Display>(span: T, message: U) -> proc_macro::TokenStream {
        syn::Error::new_spanned(span, message)
            .into_compile_error()
            .into()
    }

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut lifetimes = Vec::new();
    let mut static_lifetimes = Vec::new();
    let mut type_param_names = Vec::new();
    let mut type_param_bounds = Vec::new();
    let mut type_param_static_type_bounds = Vec::new();
    let mut static_type_params = Vec::new();
    let mut const_params = Vec::new();
    let mut const_param_names = Vec::new();
    for param in &input.generics.params {
        match param {
            syn::GenericParam::Lifetime(param) => {
                lifetimes.push(param.lifetime.clone());
                static_lifetimes.push(quote! {'static});
            }
            syn::GenericParam::Type(param) => {
                let has_static_lifetime_bound = param.bounds.iter().any(|bound| {
                    if let syn::TypeParamBound::Lifetime(lifetime) = bound {
                        lifetime.ident == "static"
                    } else {
                        false
                    }
                });

                let param_name = &param.ident;
                let param_bounds = param.bounds.iter();
                if has_static_lifetime_bound {
                    type_param_bounds.push(quote! {
                        #param_name : #(#param_bounds+)* Sized
                    });
                    let param_bounds = param.bounds.iter();
                    type_param_static_type_bounds.push(quote! {
                        #param_name : #(#param_bounds+)* Sized
                    });
                    static_type_params.push(quote! { #param_name});
                } else {
                    type_param_bounds.push(quote! {
                        #param_name : #(#param_bounds+)* gazebo::any::ProvidesStaticType + Sized
                    });
                    let param_bounds = param.bounds.iter();
                    type_param_static_type_bounds.push(quote! {
                        #param_name :: StaticType : #(#param_bounds+)* Sized
                    });
                    static_type_params.push(quote! { #param_name :: StaticType });
                }
                type_param_names.push(param.ident.clone());
            }
            syn::GenericParam::Const(params) => {
                const_params.push(params.clone());
                const_param_names.push(params.ident.clone());
            }
        }
    }

    let gen = if input.generics.lt_token.is_none() {
        quote! {
            unsafe impl #impl_generics gazebo::any::ProvidesStaticType for #name #ty_generics #where_clause {
                type StaticType = #name #ty_generics;
            }
        }
    } else {
        quote! {
            unsafe impl <
                #(#lifetimes,)*
                #(#type_param_bounds,)*
                #(#const_params,)*
                    > gazebo::any::ProvidesStaticType
            for #name <
                #(#lifetimes,)*
                #(#type_param_names,)*
                #(#const_param_names,)*
                    > #where_clause
            where
                #(#type_param_static_type_bounds,)*
            {
                type StaticType = #name <
                    #(#static_lifetimes,)*
                    #(#static_type_params,)*
                    #(#const_param_names,)*
                        >;
            }
        }
    };

    gen.into()
}
