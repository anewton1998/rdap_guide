# RDAP Bootstrapping

"Bootstrapping" is, according to [WikiPedia](https://en.wikipedia.org/wiki/Bootstrapping):

> In general, bootstrapping usually refers to a self-starting process that is supposed to continue or grow without external input.

In RDAP, it is the first process to finding an RDAP server, and the last when the RDAP server being sought is that for either
a TLD ([ccTLD](../glossary.md#cctld) or [gTLD](../glossary.md#gtld)) or the [IANA](../glossary.md#iana). This process is defined 
by [RFC 9224](https://datatracker.ietf.org/doc/html/rfc9224), which a client may execute explicitly or via a redirect server
as described in [RFC 7480](https://datatracker.ietf.org/doc/html/rfc7480#autoid-28).

---

There is no official list of RDAP bootstrap services, however two of the most popular are the ones run by [ARIN](https://rdap-bootstrap.arin.net/bootstrap)
and [RDAP.ORG](https://rdap.org), the latter of which [reports](https://mailarchive.ietf.org/arch/msg/regext/ElTMpcFDeZ_L43U9UbKqaBowjak/)
10 million query redirects daily.

---

The title of [RFC 9224](https://datatracker.ietf.org/doc/html/rfc9224), "Finding the Authoritative Registration Data Access Protocol (RDAP) Service"
is misleading in that the RFC only describes the discovery of RDAP sources registered with [IANA](../glossary.md#iana). Important as those sources
may be, they are not "authoritative" for second and third level domain registries (e.g. co.uk), IP addresses and Automous System Numbers transfered
between [RIRs](../glossary.md#rir), number resources of [NIRs](../glossary.md#nir) and [LIRs](../glossary.md#lir), and domain contacts of "thin" TLDs
where the information resides solely at a domain registrar.

## The Boostrap Files

Bootstrapping starts with one of 5 IANA files, depending on the information being sought.

| Type                      | Link                                          |
| ------------------------- | --------------------------------------------- |
| Forward DNS               | <https://data.iana.org/rdap/dns.json>         |
| IPv4 Addresses            | <https://data.iana.org/rdap/ipv4.json>        |
| IPv6 Addresses            | <https://data.iana.org/rdap/ipv6.json>        |
| Autonomous System Numbers | <https://data.iana.org/rdap/asn.json>         |
| Object Tags               | <https://data.iana.org/rdap/object-tags.json> |

For all but Object Tags, the files are structured as illustrated in [RFC 9224](https://datatracker.ietf.org/doc/html/rfc9224#name-structure-of-the-rdap-boots):

```json
{
  "version": "1.0",
  "publication": "YYYY-MM-DDTHH:MM:SSZ",
  "description": "Some text",
  "services": [
    [
      ["entry1", "entry2", "entry3"],
      [
        "https://registry.example.com/myrdap/",
        "http://registry.example.com/myrdap/"
      ]
    ],
    [
      ["entry4"],
      [
        "https://example.org/"
      ]
    ]
  ]
}
```

The entries are either domain names (e.g. ".com", ".xyz"), IP address CIDR blocks (either v6 or v4),
or Autonomous System Number (ASN) ranges, and the arrays are the [RDAP Base URLs](../protocol/rdap_urls.md#base-urls) 
of the services for those resources.

Each entry type as a specific matching strategy:

| Type       | Example Entries                                | Evaluation               |
| ---------- | ---------------------------------------------- | ------------------------ |
| DNS        | `["net", "com"]`                               | Longest Label-wise match |
| IPv4       | `["198.51.100.0/24", "192.0.0.0/8"]`           | Most specific match      |
| IPv6       | `["2001:db8:4000::/36", "2001:db8:ffff::/48"]` | Most specific match      |
| Object Tag | `["YYYY", "XXXX"]`                             | Exact match              |

## The IANA is the Root

[IANA](../glossary.md#iana) is represented as an empty string ("") in the `dns.json` file. Though IANA's base URLs
are only represented in the `dns.json` file, this should not be misconstrued to mean IANA's RDAP services only contain
TLD information.

---

As of this writing, IANA has not placed their RDAP URLs into the RDAP `dns.json` bootstrap file as root (""), though
there is an entry for ".int".

---

## Object Tags

Object Tags were introduced as a mechanism to provide bootstrapping to things in RDAP that had no
natural bootstrapping process, mostly RDAP entities. That is, [RFC 8521](https://datatracker.ietf.org/doc/html/rfc8521)
defines how tags can be applied to objects which can also be used to find the servers where those
objects are defined.

In more simple terms, this means identifying a suffix on a string is an object tag and then querying
the server for it. For example, an entity with a handle of "FOO-ARIN" can be found in the ARIN RDAP server
because the handle name is suffixed by "-ARIN".

Object tags can also be used to direct clients at RDAP services using "short names". Neither usage is in wide spread use.
