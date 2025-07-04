use std::{
    cmp::Eq,
    collections::HashMap,
    hash::Hash,
    io::Write,
    net::{Ipv4Addr, Ipv6Addr},
};

use dialoguer::{theme::ColorfulTheme, Input};

pub use wizard_derive::WizardDerive;

pub trait Wizard: Sized {
    fn prompt(msg: &str) -> Self;
}

macro_rules! impl_wizard {
    ($t:ty) => {
        impl Wizard for $t {
            fn prompt(msg: &str) -> Self {
                Input::with_theme(&ColorfulTheme::default())
                    .with_prompt(msg)
                    .allow_empty(true)
                    .interact_text()
                    .unwrap()
            }
        }
    };
}

impl_wizard!(i8);
impl_wizard!(i16);
impl_wizard!(i32);
impl_wizard!(i64);
impl_wizard!(i128);
impl_wizard!(isize);

impl_wizard!(u8);
impl_wizard!(u16);
impl_wizard!(u32);
impl_wizard!(u64);
impl_wizard!(u128);
impl_wizard!(usize);

impl_wizard!(f32);
impl_wizard!(f64);

impl_wizard!(char);

impl_wizard!(bool);

// -----

impl_wizard!(String);

impl_wizard!(Ipv4Addr);
impl_wizard!(Ipv6Addr);

// -----

impl<T> Wizard for Vec<T>
where
    T: Wizard,
{
    fn prompt(msg: &str) -> Self {
        let mut out = Vec::new();

        loop {
            let line = {
                print!("Add item to `{msg}`? (y/N): ");
                std::io::stdout().flush().unwrap();
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();

                line.trim().to_ascii_lowercase()
            };

            match line.as_str() {
                "y" | "Y" => {
                    let item = T::prompt((out.len() + 1).to_string().as_str());
                    out.push(item);
                }
                "n" | "N" | "" => {
                    return out;
                }
                _ => {
                    println!("Invalid input");
                }
            }
        }
    }
}

impl<K, V> Wizard for HashMap<K, V>
where
    K: Wizard + Hash + Eq,
    V: Wizard,
{
    fn prompt(msg: &str) -> Self {
        let mut out = HashMap::new();

        loop {
            let line = {
                print!("Add entry to `{msg}`? (y/N): ");
                std::io::stdout().flush().unwrap();
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();

                line.trim().to_ascii_lowercase()
            };

            match line.as_str() {
                "y" | "Y" => {
                    let key = K::prompt(format!("{} (key)", out.len() + 1).as_str());
                    let value = V::prompt(format!("{} (value)", out.len() + 1).as_str());
                    out.insert(key, value);
                }
                "n" | "N" | "" => {
                    return out;
                }
                _ => {
                    println!("Invalid input");
                }
            }
        }
    }
}
