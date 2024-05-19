extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
/// This macro:
/// - Ensures that the input item is a Zero-sized struct
/// - Derives Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Default
/// - Derives Serialize, Deserialize if the feature "serde" is enabled
/// - Derives JsonSchema if the feature "schemars" is enabled
/// - Adds the `unsafe` Denomination trait to the input item
pub fn denom(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);

    // 1. Ensure that the input item is a Zero-sized struct
    if !input.fields.is_empty() {
        return syn::Error::new_spanned(input, "Denom struct must be zero-sized")
            .to_compile_error()
            .into();
    }

    // 2. Derive Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Default
    //    Derive Serialize, Deserialize if the feature "serde" is enabled
    //    Derive JsonSchema if the feature "schemars" is enabled
    let mut derive = quote::quote! {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Default)]
    };

    #[cfg(feature = "serde")]
    {
        derive = quote::quote! {
            #derive
            #[derive(::serde::Serialize, ::serde::Deserialize)]
        };
    }

    #[cfg(feature = "schemars")]
    {
        derive = quote::quote! {
            #derive
            #[derive(::schemars::JsonSchema)]
        };
    }

    // 3. Add the `unsafe` Denomination trait to the input item
    let output = quote::quote! {
        #derive
        #input

        unsafe impl ::monetary::Denomination for #input {}
    };

    output.into()
}
