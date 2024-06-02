# Summary

[Introduction](README.md)
[History of RDAP](misc/history.md)

---

- [Uses of RDAP](misc/uses.md)
    // TODO scripting examples
    // put bash scripting example under here
    // python example to accept a list of domain names and find the ones not DNSSEC
    // rust example to take list of domain names and get the networks of the nameservers
- [The Protocol](protocol/README.md)
    - [HTTP](protocol/http.md)
        - [Headers](protocol/headers.md)
        - [Response Types](protocol/response_types.md)
        // TODO combine query parameters and paths into "URLs"
        // TODO cover base URLs
        // TODO cover 9082 queries
        - [Query Parameters](protocol/query_parameters.md)
        - [URL Paths](protocol/url_paths.md)
    - [The JSON Data Model](protocol/json.md)
        - [Common Data Structures](protocol/common_data_structures.md)
        - [Object Classes](protocol/object_classes.md)
        - [jCard and vCard](protocol/jcard_and_vcard.md)
        // TODO searches
        // TODO errors
        // TODO help
    - [Extensions](protocol/extensions.md)
- [Finding Servers](bootstrapping/README.md)
    // TOOD mention redirect servers
    - [The IANA Files](bootstrapping/iana.md)
    - [Redirects](bootstrapping/redirects.md)
    - [Referrals](bootstrapping/referrals.md)

---

- [Client Implementations](client_implementations/README.md)
    - [Web Applications](client_implementations/web_applications.md)
    - [Mobile Applications](client_implementations/mobile_applications.md)
    - [CLI Applications](client_implementations/cli_applications.md)
    - [Libraries](client_implementations/libraries.md)
- [Server Implementations](server_implementations/README.md)
    - [Authoritative Servers](server_implementations/authoritative.md)
    - [Redirect Servers](server_implementations/redirect.md)
    - [Conformance Tools](server_implementations/conformance_tools.md)
- [Specifications](specifications/README.md)
    - [RFCs](specifications/rfcs.md)
    - [ICANN](specifications/icann.md)
    - [NRO](specifications/nro.md)
    - [Other](specifications/other.md)
    - [IANA Registries](specifications/iana.md)
- [Other Documents](misc/other_documents.md)
- [Glossary](misc/glossary.md)

---

[Contributing](misc/contributing.md)
