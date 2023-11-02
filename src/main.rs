#![allow(dead_code)]

use std::{fmt::Display, str::FromStr};

use dialoguer::{theme::ColorfulTheme, Input};
use wizard::WizardDerive;

pub trait Wizard: Sized {
    fn prompt(msg: &str) -> Self;
}

impl<T> Wizard for T
where
    T: FromStr + Display + Clone,
    T::Err: Display,
{
    fn prompt(msg: &str) -> Self {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(msg)
            .interact_text()
            .unwrap()
    }
}

/// Config
#[derive(Debug, WizardDerive)]
struct Config {
    /// Name
    name: String,
    /// IPv4
    ipv4: std::net::Ipv4Addr,
    /// Port
    port: u16,
    /// Transport Encryption
    encryption: Encryption,
}

/// Encryption
#[derive(Debug, WizardDerive)]
enum Encryption {
    /// No encryption (insecure)
    Insecure,
    /// TLS encryption
    Tls,
}

fn main() {
    let config: Config = Wizard::prompt("Config");

    println!("\nThanks! Here is your value: {config:?}");
}
