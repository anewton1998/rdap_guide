# RDAP URLs

RDAP does not define its own URI scheme. It simply uses the "https" (and sometimes "http") URI schemes because
RDAP takes advantage of all the features HTTP has to offer. This is one of the major contrasts between RDAP and
protocols like WHOIS, which has no URI scheme, and IRIS, which defined its own application transport and therefore
its own URI scheme.

---

The distinction between a URI and a URL is beyond the scope of this document. For simplicity, a URI is a super type of 
URL (and URN). That is, a URL is a type of URI. With regard to HTTP and RDAP specifically, the terms can be
used interchangeably.

---

## Base URLs

RDAP defines URLs starting with a base URL. This is, a server operator defines a base URL such as
`https://foo.example/` or `https://bar.example/v1` or `https://baz.example/rdap` and the clients know
that queries can be appended to these URLs.

For example, if a server advertises its base URL as `https://baz.example/rdap` then a client can formulate
a domain query by appending `/domain/some-domain.example` to get the final URL `https://baz.example/rdap/domain/some-domain.example`.

While some RDAP clients can take base URLs as arguments, most RDAP clients learn of base URLs via
[bootstrapping](../bootstrapping/index.md).

## RFC 9082 and Paths

[RFC 9082](https://datatracker.ietf.org/doc/html/rfc9082) defines the URL queries for RDAP. Each query in RDAP has a defined
pattern of URL paths and query parameters that are appended to [base URLs](#base-urls).

Queries are broken into two types, [lookups](#lookups) and [searches](#searches). And there is a psuedo-lookup for [server help](#server-help).

### Lookups

Lookups are queries that return one object for a specific item in the registry.
[RFC 9082](https://datatracker.ietf.org/doc/html/rfc9082) defines six types of lookups:

| Path          | Returned Object Class                      |
| ------------- | ------------------------------------------ |
| `/ip`         | [IP Network](object_classes.md#ip-network) |
| `/autnum`     | [Autnum](object_classes.md#autnum)         |
| `/domain`     | [Domain](object_classes.med#domain)        |
| `/nameserver` | [Nameserver](object_classes.md#nameserver) |
| `/entity`     | [Entity](object_classes.md#entity)         |

#### IP And CIDR Lookups

The `/ip` path can be used to lookup IP networks either using a single IP address or using [CIDR notation](https://datatracker.ietf.org/doc/html/rfc4632).
Here are examples from [RFC 9082](https://datatracker.ietf.org/doc/html/rfc9082#name-ip-network-path-segment-spe) for an IP network
lookup by a single IP address and CIDR block:

| Lookup            | Example                               |
| ----------------- | ------------------------------------- |
| Single IP Address | https://example.com/rdap/ip/192.0.2.0 |
| CIDR notation     | https://example.com/ip/192.0.2.0/24   |

As IP networks are a hierarchy, with larger networks at the top and smaller, enclosed networks going down, this query is for the
smallest IP network that fully encloses either the IP address or CIDR block. The term for this is "most-specific".

#### ASN Lookups

[Autonomous System Numbers (ASN)](https://datatracker.ietf.org/doc/html/rfc1930) lookups are done using the `/autnum` path. This
is simply done using by appending the ASN as an integer to the path:

~~~
https://example.com/autnum/65538
~~~

#### Domain Lookups

Domain lookups use the `/domain` path for fully qualified ASCII and IDN domains and IPv4 and IPv6 reverse DNS domains:

| Lookup                | Example                                                          |
| --------------------- | ---------------------------------------------------------------- |
| Fully Qualified ASCII | https://example.com/domain/blah.example.com                      |
| Fully Qualified IDN   | https://example.com/domain/日本語.example.com                    |
| IPv4 Reverse DNS      | https://example.com/rdap/domain/2.0.192.in-addr.arpa             |
| IPv6 Reverse DNS      | https://example.com/rdap/domain/1.0.0.0.8.b.d.0.1.0.0.2.ip6.arpa |

#### Nameserver Lookups

For registries and registrars following the "host object" model (see [nameserver server children](object_classes.html#nameserver-children)), nameservers
may be queried using their fully qualified host name with the `/nameserver` path:

~~~
http://example.com/nameserver/ns1.example.com
~~~

#### Entity Lookups

RDAP entities are the only registration objects that do not have a "natural key". Therefore, an entity
lookup uses the entities handle using the `/entity` path:

~~~
https://example.com/entity/foo-bar
~~~

### Searches

[RFC 9082](https://datatracker.ietf.org/doc/html/rfc9082#name-search-path-segment-specifi) defines 3 types of domain searches,
2 types of nameserver searches, and 2 types of entity searches.

Each search query yields an array of object instances corresponding to the search type (i.e. domain searches produce an
array of domains, etc...).

Searches are not as thoroughly implemented as lookups.

#### Domain Searches

Domain searches use the `/domains` path, and each search type uses a specific query parameter.

* `/domains?name=<DOMAIN SEARCH PATTERN>` - searches for domain registrations by a domain name search pattern.
* `/domains?nsLdhName=<NAMESERVER SEARCH PATTERN>` - searches for domain registrations by a nameserver host name search pattern of a domain.
* `/domains?nsIp=<NAMESERVER IP ADDRESS>` - searches for domain registrations by a nameserver IP address of a domain.

The `<DOMAIN SEARCH PATTERN>` and `<NAMESERVER SEARCH PATTERN>` are partial string searches as defined by 
[RFC 9082](https://datatracker.ietf.org/doc/html/rfc9082#name-partial-string-searching). These patterns use the '\*' (asterisk) character
to signify zero or more of any character, such "ex\*.com". In other words, "ex\*.com" would match domains "examiner.com" and "extraordinary.com"
if both were in the RDAP server. However, the '\*' does not cross the domain zone boundary (i.e. the '.' character), so "ex\*.com" would not
match "examiner.org" or "extraordinary.net".

There is no requirement for the '\*' character to appear at the end of the domain label, so these search patterns are not strictly suffix
searches. However, the `\*` may only appear once in a domain label.

The `<NAMESERVER IP ADDRESS>` is what it sounds like, either an IPv4 or IPv6 address.

The expected response to a domain search query is a [domainSearchResult](search_responses.md#domain-search-results).

#### Nameserver Searches

Nameserver searches use the `/nameservers` path, and each search type uses a specific query parameter.

* `/nameservers?name=<NAMESERVER SEARCH PATTERN>` - searches for nameserver objects by a hostname search pattern.
* `/nameservers?ip=<NAMESERVER IP ADDRESS>` - searches for nameserver objects by a nameserver IP address.

Here, `<NAMESERVER SEARCH PATTERN>` and `<NAMESERVER IP ADDRESS>` have the same meaning as they do [above](#domain-searches).

The expected response to a nameserver search query is a [nameserverSearchResult](search_responses.md#nameserver-search-results).

#### Entity Searches

Entity searches use the `/entities` path with the following query parameters:

* `/entities?fn=<ENTITY NAME SEARCH PATTERN>` - searches for entities by a name pattern.
* `/entities?handle=<ENTITY HANDLE SEARCH PATTERN>` - searches for entities by a handle pattern.

Like the search patterns above, the `<ENTITY NAME SEARCH PATTERN>` and the `<ENTITY HANDLE SEARCH PATTERN>` are partial string
searches. The only difference is that these are searches are not bounded by domain labels.

The expected response to an entity search query is an [entitySearchResult](search_responses.md#entity-search-results).

### Server Help

RDAP defines one psuedo-lookup for server help. This simply uses the `/help` path and produces a [help response](server_help.md).

## Unknown Query Parameters

In RDAP, clients may attach "unknown" query parameters (i.e. undefined query parameters) for the purposes of "cache busting"
(see [RFC 7480 Appendix B](https://datatracker.ietf.org/doc/html/rfc7480#appendix-B)). Servers are to ignore unknown query
parameters, which also means they should not put them in redirects unless for a well-defined and intentional purpose.
