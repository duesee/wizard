use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Lit, Meta};

#[proc_macro_derive(WizardDerive)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    match input.data {
        Data::Struct(data) => {
            let fields = data.fields.iter().map(|field| {
                let name = &field.ident;
                let mut doc = String::from("<Unknown>");

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

                quote!(
                    #name: Wizard::prompt(#doc, indent + 1)
                )
            });

            let expanded = quote!(
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
        Data::Enum(data) => {
            let options = data.variants.iter().enumerate().map(|(no, variant)| {
                let mut doc = String::from("<Unknown>");

                for attr in &variant.attrs {
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

                quote!(
                    println!("{indent_str}{}) {}", #no, #doc);
                )
            });

            let arms = data.variants.iter().enumerate().map(|(no, variant)| {
                let name = &variant.ident;

                quote!(
                    #no => { break Self::#name },
                )
            });

            let expanded = quote!(
                impl Wizard for #name {
                    fn prompt(msg: &str, indent: usize) -> Self {
                        let indent_str = "  ".repeat(indent);

                        println!("{indent_str}{msg}: (choose one)");
                        #(#options)*

                        loop {
                            let index: usize = Wizard::prompt("=>", indent);

                            match index {
                                #(#arms)*
                                _ => {
                                    println!("{indent_str}[!] No such option");
                                    continue;
                                }
                            }
                        }
                    }
                }
            );

            expanded.into()
        }
        Data::Union(_data) => {
            panic!("Wizard is not implemented yet for `union`");
        }
    }
}
