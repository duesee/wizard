#![allow(dead_code)]

use std::{fmt::Display, io::Write, str::FromStr};

use wizard::WizardDerive;

pub trait Wizard: Sized {
    fn prompt(msg: &str, indent: usize) -> Self;
}

impl<T> Wizard for T
where
    T: FromStr,
    T::Err: Display,
{
    fn prompt(msg: &str, indent: usize) -> Self {
        loop {
            print!("{}{msg}: ", "  ".repeat(indent));
            let _ = std::io::stdout().flush();

            let line = {
                let mut line = String::new();
                match std::io::stdin().read_line(&mut line) {
                    Ok(_) => {}
                    Err(error) => {
                        println!("{}[!] {error} (try again)", "  ".repeat(indent));
                        continue;
                    }
                }

                line.trim().to_owned()
            };

            match line.parse() {
                Ok(res) => return res,
                Err(error) => {
                    println!("{}[!] {error} (try again)", "  ".repeat(indent));
                    continue;
                }
            }
        }
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
    let config: Config = Wizard::prompt("Config", 0);

    println!("\nThanks! Here is your value: {config:?}");
}
