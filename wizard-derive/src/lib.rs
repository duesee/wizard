use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Fields, Lit, Meta};

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
                    #name: wizard::Wizard::prompt(#doc)
                )
            });

            let expanded = quote!(
                impl wizard::Wizard for #name {
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

                match &variant.fields {
                    Fields::Named(named) => {
                        let fields = named.named.iter().map(|field| {
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
                                #name: wizard::Wizard::prompt(#doc)
                            )
                        });

                        quote!(
                            #no => {
                                Self::#name {
                                    #(#fields,)*
                                }
                            },
                        )
                    }
                    Fields::Unnamed(unnamed) => {
                        let fields = unnamed.unnamed.iter().map(|field| {
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
                                wizard::Wizard::prompt(#doc)
                            )
                        });

                        quote!(
                            #no => {
                                Self::#name(#(#fields,)*)
                            },
                        )
                    }
                    Fields::Unit => quote!(#no => Self::#name,),
                }
            });

            let expanded = quote!(
                impl wizard::Wizard for #name {
                    fn prompt(msg: &str) -> Self {
                        use dialoguer::{Select, theme::ColorfulTheme};

                        let selections = &[
                            #(#options)*
                        ];

                        let num = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt(msg)
                            .items(&selections[..])
                            .default(0)
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
        Data::Union(_) => {
            panic!("Wizard is not implemented yet for `union`");
        }
    }
}
