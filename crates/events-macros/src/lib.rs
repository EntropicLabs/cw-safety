extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Ident, Item, LitStr};

struct EventAttr {
    event_type: LitStr,
}

impl Parse for EventAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let event_type = input.parse()?;
        Ok(EventAttr { event_type })
    }
}

#[proc_macro_attribute]
/// This macro:
/// - Accepts one argument: a string
/// - Ensures that the input item is a struct or enum
/// - Derives Clone, Debug, Serialize, Deserialize, JsonSchema
/// - Implements TypedEvent with fn type_name(&self) implemented using the argument
pub fn event(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let input = parse_macro_input!(item as Item);
    // Parse inputs
    let EventAttr { event_type } = parse_macro_input!(attr as EventAttr);

    // Ensure the input is a struct or enum
    let ident = match &input {
        Item::Struct(item_struct) => &item_struct.ident,
        Item::Enum(item_enum) => &item_enum.ident,
        _ => {
            return syn::Error::new_spanned(
                input,
                "Can only derive TypedEvent for structs and enums",
            )
            .to_compile_error()
            .into()
        }
    };
    // Prepare the list of derives
    let found_crate = crate_name("events").expect("Failed to find the `events` crate");
    let mut derives = vec![quote! { Clone }, quote! { Debug }];
    let derive_ident = match found_crate {
        FoundCrate::Itself => {
            quote! { crate }
        }
        FoundCrate::Name(crate_name) => {
            let ident = Ident::new(&crate_name, Span::call_site());
            quote! { #ident }
        }
    };
    let serde_path = quote! { #derive_ident::serde };
    let schemars_path = quote! { #derive_ident::schemars };
    derives.push(quote! { #serde_path::Serialize });
    derives.push(quote! { #serde_path::Deserialize });
    derives.push(quote! { #schemars_path::JsonSchema });

    // Combine all derives into one attribute
    let derives = quote! { #[derive(#(#derives),*)] };

    // Create a trait impl for TypedEvent
    let trait_impl = quote! {
        impl #derive_ident::TypedEvent for #ident {
            fn type_name(&self) -> String {
                #event_type.to_string()
            }
        }
    };

    // Combine the original item definition with the derives and trait implementation
    let serde_path_str = serde_path.to_string();
    let schemars_path_str = schemars_path.to_string();
    let expanded = quote! {
        #derives
        #[serde(crate = #serde_path_str)]
        #[schemars(crate = #schemars_path_str)]
        #input

        #trait_impl
    };

    // Convert the generated code back into a TokenStream
    TokenStream::from(expanded)
}
