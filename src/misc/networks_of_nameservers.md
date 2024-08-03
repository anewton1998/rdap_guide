# Rust: Networks of Nameservers

Because both DNS registries and IP address registries use RDAP, writing a custom client to look for the networks where
a domain name's nameservers are located is easy to accomplish.

This example uses Rust, and is a bit longer than the previous examples because it loops over a list
of domains to find their IP addresses, then loops over those IP addresses to get the network information.

The complete source code may be found [here](https://github.com/anewton1998/rdap_guide/tree/main/src/examples/clients/rust_domain_ip),
but the meat of the code is below:

```rust,ignore
{{#include ../examples/clients/rust_domain_ip/src/main.rs:18:58}}  
```

This code uses the [ICANN RDAP Client Library](https://github.com/icann/icann-rdap/tree/main/icann-rdap-client), and consists
of three nested loops. Each loop uses the same `Client` and uses the built-in bootstrapping mechanism. The outermost loop
iterates over the domain names querying for each, the next loop gets the IP addresses of the nameservers returned in those
domain name lookups, and the innermost loop then queries for the IP networks of the IP addresses of the nameservers.

When it is run, it takes domain names as command line arguments. Here is the sample output:

```
$ cargo run -- iana.org icann.org
   Compiling rust_domain_ip v0.1.0 (/home/andy/projects/rdap_guide/src/examples/clients/rust_domain_ip)
    Finished dev [unoptimized + debuginfo] target(s) in 2.64s
     Running `target/debug/rust_domain_ip iana.org icann.org`
domain: iana.org
ns.icann.org(2001:500:89::53) is in network 2001:500:89:: - 2001:500:89:ffff:ffff:ffff:ffff:ffff
ns.icann.org(199.4.138.53) is in network 199.4.138.0 - 199.4.138.255
domain: icann.org
ns.icann.org(2001:500:89::53) is in network 2001:500:89:: - 2001:500:89:ffff:ffff:ffff:ffff:ffff
ns.icann.org(199.4.138.53) is in network 199.4.138.0 - 199.4.138.255
```
