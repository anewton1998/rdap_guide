# CLI Applications

Command-Line Interface (CLI) clients have a heritage that goes back to original WHOIS,
which was accessed via clients run on Unix systems.

## NicInfo

From the GitHub Repository:

> NicInfo is a general purpose, command line Registry Data Access Protocol (RDAP) 
> client released under an open source, ISC derivative BSD style license. 
> RDAP is an HTTP-based RESTful protocol defined by the IETF as a replacement for Whois.

* License: ISC
* Repository: <https://github.com/arineng/nicinfo>
* Language: Ruby
* Operating Systems: MacOS, Linux, Windows
* Features:
  * Query type detection: it will attempt to determine what type of query is needed based on the supplied query value.
  * Plain text output: default output is a text version of the RDAP results.
  * JSON output: the RDAP JSON can be passed directly to a calling program for intergration with scripts with the ability to select specific JSON values.
  * Multiple output controls: the amount of text detail and process execution can be varied and sent to different files.
  * A Built-in cache: RDAP queries are cached.
  * Bootstrapping using the IANA bootstrap files or by using a bootstrap server.
  * Demonstration queries: a set of built-in queries and results are provided for demonstration purposes.

![NicInfo Querying 1.1.1.1](1.1.1.1-NicInfo.png)

## OpenRDAP

From OpenRDAP.org:

> OpenRDAP is a command line client for the Registration Data Access Protocol, written in Go.

> RDAP is a replacement for WHOIS, which provides domain name & IP address registration information in JSON format over HTTP.

* License: MIT
* Repository: <https://github.com/openrdap/rdap>
* Language: Go
* Operating Systems: MacOS, Linux, Windows
* Features:
  * Output formats: text, JSON, WHOIS style
  * Query types supported:
    * ip
    * domain
    * autnum
    * nameserver
    * entity
    * help
    * url
    * domain-search
    * domain-search-by-nameserver
    * domain-search-by-nameserver-ip
    * nameserver-search
    * nameserver-search-by-ip
    * entity-search
    * entity-search-by-handle
  * Automatic server detection for ip/domain/autnum/entities
  * Object tags support
  * Bootstrap cache (optional, uses ~/.openrdap by default)
  * X.509 client authentication

![OpenRDAP Querying Google.com](openrdap-google.com.png)
