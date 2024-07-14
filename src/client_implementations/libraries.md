# Libraries

Client libraries can be embedded into other programs to give them
the ability to query RDAP. These libraries are categorized by programming
language.

## Other

### Rapid7

* Package Information: <https://extensions.rapid7.com/extension/rdap>
* Repository: <https://github.com/rapid7/insightconnect-plugins>
* Documentation: <https://extensions.rapid7.com/extension/rdap#Documentation-Setup>
* License: MIT

### Splunkbase

* Package Information: <https://splunkbase.splunk.com/app/5945>
* Repository: <https://github.com/splunk-soar-connectors/whoisrdap>
* Documentation: <https://splunkbase.splunk.com/app/5945>
* License: Splunk General Terms

## Perl

### Net::RDAP

> [Net::RDAP](https://metacpan.org/pod/Net%3A%3ARDAP) provides an interface to the Registration Data Access
> Protocol (RDAP).
> 
> RDAP is replacing Whois as the preferred way of obtainining information
> about Internet resources (IP addresses, autonymous system numbers, and
> domain names). As of writing, RDAP is fully supported by Regional
> Internet Registries (who are responsible for the allocation of IP
> addresses and AS numbers) and generic TLD operators (e.g. .com, .org,
> .xyz) but is still being rolled out among country-code registries.
> 
> [Net::RDAP](https://metacpan.org/pod/Net%3A%3ARDAP) does all the hard work of determining the correct
> server to query ([Net::RDAP::Registry](https://metacpan.org/pod/Net%3A%3ARDAP%3A%3ARegistry) is an interface to the
> IANA registry of RDAP services), querying the server ([Net::RDAP::UA](https://metacpan.org/pod/Net%3A%3ARDAP%3A%3AUA)
> is an RDAP HTTP user agent), and parsing the response
> ([Net::RDAP::Object](https://metacpan.org/pod/Net%3A%3ARDAP%3A%3AObject) and its submodules provide access to the data
> returned by the server). As such, it provides a single unified
> interface to information about all unique Internet identifiers.

* Package Information: <https://metacpan.org/pod/Net%3A%3ARDAP>
* Repository: <https://github.com/gbxyz/perl-net-rdap>
* Documentation: <https://metacpan.org/pod/Net%3A%3ARDAP>
* License: Unknown Open Source
* Related: [CLI Application](cli_applications.md#rdapper)

## PHP

### RDAP Whois Proxy

> A bridge from WHOIS to RDAP

* Repository: <https://github.com/hiqdev/rdap-whois-proxy>
* License: BSD 3-Clause

## Python

### Whoisit

> A Python client to RDAP WHOIS-like services for internet resources (IPs, ASNs, domains,
> etc.). `whoisit` is a simple library that makes requests to the "new" RDAP (Registration
> Data Access Protocol) query services for internet resource information. These services
> started to appear in 2017 and have become more widespread since 2020.
> 
> `whoisit` is designed to abstract over RDAP. While RDAP is a basic HTTP and JSON based
> protocol which can be implemented in a single line of Python with `requests` the
> bootstrapping (which RDAP service to query for what item) and extracting useful
> information from the RDAP responses is extensive enough that a library like this is
> useful.

Packine Information: <https://pypi.org/project/whoisit/>
Repository: <https://github.com/meeb/whoisit>
Documentation: <https://pypi.org/project/whoisit/>
License: BSD 3-Clause

## Rust

### ICANN RDAP Client Library

> This is a command-line interface (CLI) client for the Registration Data Access Protocol (RDAP) written and sponsored
> by the Internet Corporation for Assigned Names and Numbers [(ICANN)](https://www.icann.org). 
> RDAP is standard of the [IETF](https://ietf.org/), and extensions
> to RDAP are a current work activity of the IETF's [REGEXT working group](https://datatracker.ietf.org/wg/regext/documents/).
> More information on ICANN's role in RDAP can be found [here](https://www.icann.org/rdap).

* Package Information: <https://crates.io/crates/icann-rdap-client/0.0.16>
* Repository: <https://github.com/icann/icann-rdap>
* Documentation: <https://docs.rs/icann-rdap-client/latest/icann_rdap_client/>
* License: MIT, Apache License 2.0
* Related: [CLI Application](cli_applications.md#ican-rdap-cli), [Authoritative Server](../server_implementations/authoritative.md#icann-rdap-server), [Redirect Server](../server_implementations/redirect.md#icann-rdap-server)

### rdap_client

> Async and fast RDAP client and parser for Rust.

* Package Information: <https://crates.io/crates/rdap_client>
* Repository: <https://github.com/JakubOnderka/rdap_client>
* Documentation: <https://docs.rs/rdap_client/0.2.0>
* License: BSD 2-Clause
* Features:
  * Supported Standards:
    * RFC 7480: HTTP Usage in the Registration Data Access Protocol (RDAP)
    * RFC 7482: Registration Data Access Protocol (RDAP) Query Format
    * RFC 7483: JSON Responses for the Registration Data Access Protocol (RDAP)
    * RFC 8056: Extensible Provisioning Protocol (EPP) and Registration Data Access Protocol (RDAP) Status Mapping
    * RFC 8521: Registration Data Access Protocol (RDAP) Object Tagging
    * RDAP JSON Values
  * Supported Extensions
    * fred
    * cidr0
    * arin_originas0
    * rdap_objectTag (RFC 8521)

### test_friendly_rdap_client

> This is essentially a version of the original [rdap_client](https://github.com/JakubOnderka/rdap_client)
> with a few extra sprinkles to make it a bit easier for me to wire up `rdap_client` in
> integration tests.
> 
> I initially attempted to keep the code as close to upstream as possible but,  unfortunately,
> crates.io either did not like the way the Cargo.toml was set up or (more likely) I was doing
> something stupid.

* Package Information: <https://crates.io/crates/test_friendly_rdap_client>
* Repository: <https://github.com/rorymckinley/test_friendly_rdap_client.git>
* Documentation: <https://docs.rs/test_friendly_rdap_client/0.1.0>
* License: BSD 2-Clause
* Features:
  * Supported Standards:
    * RFC 7480: HTTP Usage in the Registration Data Access Protocol (RDAP)
    * RFC 7482: Registration Data Access Protocol (RDAP) Query Format
    * RFC 7483: JSON Responses for the Registration Data Access Protocol (RDAP)
    * RFC 8056: Extensible Provisioning Protocol (EPP) and Registration Data Access Protocol (RDAP) Status Mapping
    * RFC 8521: Registration Data Access Protocol (RDAP) Object Tagging
    * RDAP JSON Values
  * Supported Extensions
    * fred
    * cidr0
    * arin_originas0
    * rdap_objectTag (RFC 8521)
    
## Typescript

### RDAP Check

> A simple library and command-line tool to check domain name availability in bulk using the RDAP protocol, a simple protocol meant to replace WHOIS.

* Package Information: <https://deno.land/x/rdapcheck@v0.1.1>
* Repository: <https://github.com/Gadiguibou/rdapcheck>
* Documentation: <https://deno.land/x/rdapcheck@v0.1.1?doc>
* License: AGPL-3.0
* Related: [CLI Application](cli_applications.md#rdap-check)
