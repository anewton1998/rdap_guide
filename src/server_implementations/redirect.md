# Redirect Servers

Redirect servers use the IANA bootstrapping files to redirect clients
to an RDAP server. See [RDAP Bootstrapping](../bootstrapping/iana.md)
and [Redirects](../bootstrapping/iana.md).

## Services


### ARIN RDAP Bootstrap Server

* Base RDAP URL: <https://rdap-bootstrap.arin/net/bootstrap>
* More Information: <https://www.arin.net/resources/registry/whois/rdap/>
* Related: [Software](#rdap-bootstrap-server)

### RDAP.Net

* Base RDAP URL: <https://www.rdap.net>
* More Information: <https://www.openrdap.org/api>
* Related: [Software](../client_implementations/cli_applications.md#openrdap)

### Root.RDAP.Org

* Base RDAP URL: <https://root.rdap.org>
* Additional services: <http://registrars.rdap.org/entity/{NNNN}-iana> where NNNN is the numeric [IANA ID](https://www.iana.org/assignments/registrar-ids/).
* More information: <https://about.rdap.org/>
* Related: [Software](#rdaporg)

## Software

### APNIC rdap-ingressd

> *rdap-ingressd* is a proxy and redirection server for the RDAP
> protocol.  It receives RDAP queries from clients and routes the
> requests to the appropriate RDAP service by either:
> 
> 1. HTTP 301 redirect; or
> 2. Proxying the request to another RDAP service and returning the result.

* License: BSD
* Repository: <https://github.com/APNIC-net/rdap-ingressd>
* Language: Java

### ICANN RDAP Server

* License: MIT, Apache License 2.0
* Repository: <https://github.com/icann/icann-rdap>
* Language: Rust
* Related: [Client Library](../client_implementations/libraries.md#icann-rdap-client-library), [CLI Application](../client_implementations/cli_applications.md#ican-rdap-cli), [Authoritative Server](authoritative.md#icann-rdap-server)

### RDAP Bootstrap Server

From the GitHub Repository:

> The Registration Data Access Protocol (RDAP) defines a bootstrapping process in RFC 7484. 
> A bootstrap server aids clients by reading the bootstrapping information published by IANA 
> and using it to send HTTP redirects to RDAP queries. Clients utilizing a bootstrap server 
> will not need to conduct their own bootstrapping.

* License: ISC License
* Repository: <https://github.com/arineng/rdap_bootstrap_server>
* Language: Java
* Related: [Service](#arin-rdap-bootstrap-server)

### RDAP.Org

From the GitHub Repository:

>This repository contains the RDAP.org Bootstrap Server, which is implemented in PHP using Openswoole.
>
> You will also find a Dockerfile for building Docker container images, and a TOML file for deploying on Fly.io.

* License: Unknown Open Source
* Repository: <https://github.com/rdap-org/rdap.org>
* Language: PHP
* Related: [Service](#rootrdaporg)
