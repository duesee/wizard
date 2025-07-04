#![allow(dead_code)]

use std::collections::HashMap;

use wizard::{Wizard, WizardDerive};

/// Config
#[derive(Debug, WizardDerive)]
struct Config {
    /// Hostname
    hostname: String,
    /// Addresses (IPv4)
    ipv4s: Vec<std::net::Ipv4Addr>,
    /// Port
    port: u16,
    /// Transport Encryption
    encryption: Encryption,
    /// Services (subdomain: service)
    services: HashMap<String, Service>,
}

/// Encryption
#[derive(Debug, WizardDerive)]
enum Encryption {
    /// No encryption (insecure)
    Insecure,
    /// TLS encryption
    Tls,
}

/// Service
#[derive(Debug, WizardDerive)]
enum Service {
    /// Caddy
    Caddy {
        /// Workers
        workers: u16,
    },
    /// Zola
    Zola {
        /// Content
        content: Vec<String>,
    },
}

fn main() {
    let config: Config = Wizard::prompt("Config");

    println!("{config:#?}");
}
