# Authoritative Servers

## CoCCA RDAP

<https://cocca.org.nz/srs/rdap.html>

* License: Unknown
* Repository: Unknown
* Language: Java

## DNSBelgium RDAP

> This java library makes it very easy to build an RDAP server that talks with your registry back-end.

* License: Apache 2.0
* Repository: <https://github.com/DNSBelgium/rdap>
* Language: Java

## Domain Cocoon

<https://domaincocoon.com/products/whois-server-installation-configuration-jwhoisserver>

* License: Unknown
* Repository: Unknown
* Language: Java

## ICANN RDAP Server

> This server was created to aid in the development of the ICANN RDAP Command Line Interface client.
> It can be used as a library or as a server started within its own process. It currently has in-memory
> storage, though its storage layer is architected to accommodate a PostgreSQL backend if that is needed
> in the future.

* License: MIT, Apache License 2.0
* Repository: <https://github.com/icann/icann-rdap>
* Language: Rust
* Related: [Client Library](../client_implementations/libraries.md#icann-rdap-client-library), [CLI Application](../client_implementations/cli_applications.md#ican-rdap-cli), [Redirect Server](redirect.md#icann-rdap-server)

## Namingo Registry

> Namingo is a state-of-the-art open-source domain registry platform, diligently 
> crafted to serve ccTLD, gTLD, brand and private domain registries. Written 
> from scratch in 2023/2024, it adheres to the latest standards, ensuring a cutting-edge experience.

* License: MIT
* Repository: <https://github.com/getnamingo/registry>
* Language: PHP
* Related: [Authoritative Server](#namingo-registrar-platform)

## Namingo Registrar Platform

> **Namingo Registrar Platform**, built atop [FOSSBilling](https://fossbilling.org/), 
> transforms the system into a comprehensive and open-source [ICANN](https://icann.org/) 
> accredited registrar management system. It provides a powerful and flexible solution 
> for managing domain names as an ICANN accredited registrar, adhering to the stringent standards set by ICANN.

* License: MIT
* Repository: <https://github.com/getnamingo/registrar>
* Language: PHP
* Related: [Authoritative Server](#namingo-registry)


## Nomulus

> Nomulus is an open source, scalable, cloud-based service for operating
> [top-level domains](https://en.wikipedia.org/wiki/Top-level_domain) (TLDs). It
> is the authoritative source for the TLDs that it runs, meaning that it is
> responsible for tracking domain name ownership and handling registrations,
> renewals, availability checks, and WHOIS requests. End-user registrants (i.e.
> people or companies that want to register a domain name) use an intermediate
> domain name registrar acting on their behalf to interact with the registry.

* License: Apache 2.0
* Repository: <https://github.com/google/nomulus>
* Language: Java

## RDAPd

> rdapd takes historical data imported from the RIPE database `last` and
> `history` tables and exposes an RDAP API for current state, as well as
> RDAP-like responses for historical state.

* License: BSD
* Repository: <https://github.com/APNIC-net/rdapd>
* Language: Java

## Red Dog

> Red Dog is an Open Source RDAP (Registration Data Access Protocol) Server implementation built with Java, funded and developed by [NIC Mexico](http://www.nic.mx).
> 
> The server is a RESTful service expected to provide HTTP content in accordance with:
> - [RFC 7480 - HTTP Usage in the Registration Data Access Protocol (RDAP)](https://tools.ietf.org/html/rfc7480)
> - [RFC 7481 - Security Services for the Registration Data Access Protocol (RDAP)](https://tools.ietf.org/html/rfc7481)
> - [RFC 7482 - Registration Data Access Protocol (RDAP) Query Format](https://tools.ietf.org/html/rfc7482)
> - [RFC 7483 - JSON Responses for the Registration Data Access Protocol (RDAP)](https://tools.ietf.org/html/rfc7483).
> 
> Beside the RFCs accordance, Red Dog has the following features:
> - Response render can be customized by implementing a set of interfaces; e.g. beside JSON responses, a TEXT/HTML/XML or any other response type can be returned if the implementer wishes to.
> - Reference database and data access implementation to ease Red Dog's use.
> - A set of Java interfaces to implement any kind of data access according to the implementer needs (e.g. data can be obtained from the implementer data repository).
> - Optional Basic Authentication and the possibility to implement/customize the authentication type if needed.
> - Response data privacy using general settings (e.g. everybody can see X data, nobody can see Y data, etc.) or specific settings (e.g. only the owner can see X data, certain custom user roles can see Y data).

* License: Apache 2.0
* Repository: <https://github.com/NICMx/rdap-server>
* Language: Java
