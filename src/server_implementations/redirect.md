# Redirect Servers

Redirect servers use the IANA bootstrapping files to redirect clients
to an RDAP server. See [RDAP Bootstrapping](../bootstrapping/iana.md)
and [Redirects](../bootstrapping/iana.md).

## Services

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

### RDAP.Org

From the GitHub Repository:

>This repository contains the RDAP.org Bootstrap Server, which is implemented in PHP using Openswoole.
>
> You will also find a Dockerfile for building Docker container images, and a TOML file for deploying on Fly.io.

* License: Unknown Open Source
* Repository: <https://github.com/rdap-org/rdap.org>
* Language: PHP
* Related: [Service](#rootrdaporg)
