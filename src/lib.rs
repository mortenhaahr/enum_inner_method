use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, ItemEnum, Signature, parse_macro_input};

#[proc_macro_attribute]
pub fn enum_inner_method(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);
    let func = parse_macro_input!(attr as Signature);
    let func_name = &func.ident;
    let func_inputs = &func.inputs;
    let enum_name = &input.ident;

    let func_has_self = func_inputs.iter().any(|input| {
        if let syn::FnArg::Receiver(_) = input {
            true
        } else {
            false
        }
    });

    let func_inputs = if func_has_self {
        // Everything except the first argument
        func_inputs.into_iter().skip(1)
    } else {
        panic!("In current version the first argument must be self")
    };

    let match_arms = input.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let c_params = func_inputs.clone().map(|param| {
            if let syn::FnArg::Typed(pat) = param {
                &pat.pat
            } else {
                panic!("Received self twice - something went wrong")
            }
        });

        if let Fields::Unnamed(fields) = &variant.fields {
            if fields.unnamed.len() == 1 {
                return quote! {
                    Self::#variant_name(inner) => inner.#func_name(#(#c_params, )*),
                };
            }
        }
        syn::Error::new_spanned(
            variant_name,
            "Each variant must have exactly one unnamed field",
        )
        .to_compile_error()
    });

    let expanded = quote! {
        #input

        impl #enum_name {
            pub #func{
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
