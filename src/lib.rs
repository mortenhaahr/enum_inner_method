use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Fields, Ident, Token, Type, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct FuncInfo {
    func_name: Ident,
    params: Vec<Type>,
    return_type: Option<Ident>,
}

impl Parse for FuncInfo {
    // Expects the input: funcname(type1, type2...) -> return_type
    // with return_type being optional
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let func_name: Ident = input.parse()?;

        let args_content;
        parenthesized!(args_content in input);
        let params = args_content.parse_terminated(Type::parse, Token![,])?;
        let params: Vec<Type> = params.into_iter().collect();

        let return_type = if input.parse::<Token![->]>().is_ok() {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(FuncInfo {
            func_name,
            params,
            return_type,
        })
    }
}

#[proc_macro_attribute]
pub fn enum_inner_method(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as FuncInfo);
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let func_name = &args.func_name;
    let return_type = &args.return_type.map(|t| quote!(-> #t)).unwrap_or(quote!());
    let params = &args.params;

    let Data::Enum(data_enum) = &input.data else {
        return syn::Error::new_spanned(name, "fgen can only be used on enums")
            .to_compile_error()
            .into();
    };

    let def_params = params
        .iter()
        .enumerate()
        .map(|(n, p)| (format_ident!("var{}", n), p))
        .map(|(n, p)| quote!(, #n: #p));

    let call_params: Vec<_> = params
        .iter()
        .enumerate()
        .map(|(n, _)| format_ident!("var{}", n))
        .collect();

    let match_arms = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let c_params = call_params.clone().into_iter();
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
        impl #name {
            pub fn #func_name(&self #(#def_params)*) #return_type {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    TokenStream::from(quote! {
        #input
        #expanded
    })
}
