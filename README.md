# Wizard

The following code ...

```rust
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
    ipv4: Ipv4Addr,
}

fn main() {
    let config: Config = Wizard::prompt("Config", 0);

    println!("{config:?}");
}
```

... generates this terminal interaction ...

```text
$ cargo run
# Config
  Host: example.org
  # Inner
    IPv4: 1.1.1.999
    [!] invalid IPv4 address syntax (try again)
    IPv4: -1.-1.-1.1^?-1
    [!] invalid IPv4 address syntax (try again)
    IPv4: 9.9.9.9
  Port: Fooooooooo!
  [!] invalid digit found in string (try again)
  Port: 143
Config { host: "example.org", inner: Inner { ipv4: 9.9.9.9 }, port: 143 }
```
