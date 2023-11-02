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
                    #name: Wizard::prompt(#doc)
                )
            });

            let expanded = quote!(
                impl Wizard for #name {
                    fn prompt(msg: &str) -> Self {
                        Self {
                            #(#fields,)*
                        }
                    }
                }
            );

            expanded.into()
        }
        Data::Enum(data) => {
            let options = data.variants.iter().map(|variant| {
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
                    #doc,
                )
            });

            let arms = data.variants.iter().enumerate().map(|(no, variant)| {
                let name = &variant.ident;

                quote!(
                    #no => { Self::#name },
                )
            });

            let expanded = quote!(
                impl Wizard for #name {
                    fn prompt(msg: &str) -> Self {
                        use dialoguer::Select;

                        let selections = &[
                            #(#options)*
                        ];

                        let num = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt(msg)
                            .items(&selections[..])
                            .interact()
                            .unwrap();

                        match num {
                            #(#arms)*
                            _ => unreachable!(),
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
