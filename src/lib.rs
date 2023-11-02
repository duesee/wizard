use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, Lit, Meta};

#[proc_macro_derive(WizardDerive)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let data = if let syn::Data::Struct(data) = input.data {
        data
    } else {
        unimplemented!();
    };

    let fields = data.fields.iter().map(|field| {
        let name = &field.ident;
        let mut doc = String::from("Unknown");

        for attr in &field.attrs {
            match &attr.meta {
                Meta::NameValue(name_value) => match &name_value.value {
                    Expr::Lit(lit) => match &lit.lit {
                        Lit::Str(s) => doc = s.value().trim().to_string(),
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }

        quote! {
            #name: Wizard::prompt(#doc, indent + 1)
        }
    });

    let expanded = quote! (
        impl Wizard for #name {
            fn prompt(msg: &str, indent: usize) -> Self {
                println!("{}# {msg}", "  ".repeat(indent));

                Self {
                    #(#fields,)*
                }
            }
        }
    );

    expanded.into()
}
