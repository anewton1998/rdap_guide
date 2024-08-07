# Summary

[Introduction](README.md)
[History of RDAP](misc/history.md)

---

- [Uses of RDAP](misc/uses.md)
    - [Bash: Finding IP Networks](misc/finding_ip_networks.md)
    - [Python: Finding Protected Domains](misc/finding_protected_domains.md)
    - [Rust: Networks of Nameservers](misc/networks_of_nameservers.md)
- [The Protocol](protocol/README.md)
    - [HTTP](protocol/http.md)
        - [Headers](protocol/headers.md)
        - [Response Types](protocol/response_types.md)
        - [RDAP Urls](protocol/rdap_urls.md)
    - [The JSON Data Model](protocol/json.md)  // TODO add better descriptions of LDH and IDNs
        - [Common Data Structures](protocol/common_data_structures.md)
        - [Object Classes](protocol/object_classes.md)
        - [jCard and vCard](protocol/jcard_and_vcard.md)
        - [Search Responses](protocol/search_responses.md)
        - [Error Responses](protocol/error_responses.md)
        - [Server Help](protocol/server_help.md)
    - [Extending RDAP](protocol/extensions.md)
- [Finding Servers](bootstrapping/README.md)
    - [Bootstrapping](bootstrapping/iana.md)
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
    - [IANA Registries](specifications/iana.md)
    // - [Other](specifications/other.md)
- [Other Documents](misc/other_documents.md)
- [Glossary](misc/glossary.md)
  // TODO examples

---

[Contributing](misc/contributing.md)
