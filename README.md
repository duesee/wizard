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
    let config: Config = Wizard::prompt("Config", 0);

    println!("\nThanks! Here is your value: {config:?}");
}
```

... leads to this terminal interaction ...

```text
$ cargo run
# Config
  Name: My config
  IPv4: 1.1.1.999
  [!] invalid IPv4 address syntax (try again)
  IPv4: 9.9.9.9
  Port: abc
  [!] invalid digit found in string (try again)
  Port: 143
  Transport Encryption: (choose one)
  0) No encryption (insecure)
  1) TLS encryption
  =>: 2
  [!] No such option
  =>: 1

Thanks! Here is your value: Config { name: "My config", ipv4: 9.9.9.9, port: 143, encryption: Tls }
```
