extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse_macro_input, Ident, ItemStruct};

#[proc_macro_attribute]
/// This macro:
/// - Ensures that the input item is a Zero-sized struct
/// - Derives Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Default
/// - Derives Serialize, Deserialize if the feature "serde" is enabled
/// - Derives JsonSchema if the feature "schemars" is enabled
/// - Adds the `unsafe` Denomination trait to the input item
pub fn denom(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let input = parse_macro_input!(item as ItemStruct);

    // Ensure the struct is zero-sized
    if !input.fields.is_empty() {
        return syn::Error::new_spanned(input, "Struct must be zero-sized")
            .to_compile_error()
            .into();
    }

    // Get the struct name
    let name = &input.ident;

    // Get the `monetary` crate
    let found_crate = crate_name("monetary").expect("Failed to find the `monetary` crate");

    // Prepare the list of derives
    let mut derives = vec![
        quote! { Clone },
        quote! { Debug },
        quote! { PartialEq },
        quote! { Eq },
        quote! { PartialOrd },
        quote! { Ord },
        quote! { Copy },
        quote! { Default },
    ];

    // Check for features and conditionally add derives
    #[cfg(feature = "serde")]
    match &found_crate {
        FoundCrate::Itself => {
            derives.push(quote! { crate::__derive_import::serde::Serialize });
            derives.push(quote! { crate::__derive_import::serde::Deserialize });
        }
        FoundCrate::Name(crate_name) => {
            let ident = Ident::new(crate_name, Span::call_site());
            derives.push(quote! { #ident::__derive_import::serde::Serialize });
            derives.push(quote! { #ident::__derive_import::serde::Deserialize });
        }
    }

    #[cfg(feature = "schemars")]
    match &found_crate {
        FoundCrate::Itself => {
            derives.push(quote! { crate::__derive_import::schemars::JsonSchema });
        }
        FoundCrate::Name(crate_name) => {
            let ident = Ident::new(crate_name, Span::call_site());
            derives.push(quote! { #ident::__derive_import::schemars::JsonSchema });
        }
    }

    // Combine all derives into one attribute
    let derives = quote! { #[derive(#(#derives),*)] };

    // Generate the Denomination trait implementation
    let trait_impl = match found_crate {
        FoundCrate::Itself => quote! {
            unsafe impl crate::Denomination for #name {}
        },
        FoundCrate::Name(crate_name) => {
            let ident = Ident::new(&crate_name, Span::call_site());
            quote! {
                unsafe impl #ident::Denomination for #name {}
            }
        }
    };

    // Combine the original struct definition with the derives and trait implementation
    let expanded = quote! {
        #derives
        #input
        #trait_impl
    };

    // Convert the generated code back into a TokenStream
    TokenStream::from(expanded)
}
