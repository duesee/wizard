# Wizard

The following code ...

```rust
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
```

... provides this terminal interaction ...

![./out.gif](./out.gif)
