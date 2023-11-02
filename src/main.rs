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

#[derive(Debug, WizardDerive)]
struct Config {
    /// Host
    host: String,
    /// Inner
    inner: Inner,
    /// Port
    port: u16,
}

#[derive(Debug, WizardDerive)]
struct Inner {
    /// IPv4
    ipv4: std::net::Ipv4Addr,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = Wizard::prompt("Config", 0);

    println!("{config:?}");

    Ok(())
}
