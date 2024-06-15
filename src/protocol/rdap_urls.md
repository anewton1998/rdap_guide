# RDAP Urls

RDAP does not define its own URI scheme. It simply uses the "https" (and sometimes "http") URI schemes because
RDAP takes advantage of all the features HTTP has to offer. This is one of the major contrasts between RDAP and
protocols like WHOIS, which has no URI scheme, and IRIS, which defined its own application transport and therefore
its own URI scheme.

---

The distinction between a URI and a URL is beyond the scope of this document. For simplicity, a URI is a super type
of a URL (and URN). That is, a URL is a type of URI. With regards to HTTP and RDAP specifically, the terms can be
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
Here is an example from [RFC 9082](https://datatracker.ietf.org/doc/html/rfc9082#name-ip-network-path-segment-spe) for an IP network
lookup by a single IP address:

~~~
https://example.com/rdap/ip/192.0.2.0
~~~

The CIDR notation form is similar:

~~~
https://example.com/ip/192.0.2.0/24
~~~

As IP networks are a hierarchy, with larger networks at the top and smaller, enclosed networks going down, this query is for the
smallest IP network that fully encloses either the IP address or CIDR block. The term for this is "most-specific".

#### ASN Lookups

[Autonomous System Numbers (ASN)](https://datatracker.ietf.org/doc/html/rfc1930) lookups are done using the `/autnum` path. This
is simply done using by appending the ASN as an integer to the path:

~~~
https://example.com/autnum/65538
~~~

#### Domain Lookups

Domain lookups use the `/domain` path:

~~~
https://example.com/domain/blah.example.com
~~~

These lookups may be also use IDNs:

~~~
https://example.com/domain/日本語.example.com
~~~

This query can also be used for finding reverse DNS domains by using their full qualification in in-addr.arap or ip6.arpa:

~~~
https://example.com/rdap/domain/2.0.192.in-addr.arpa
https://example.com/rdap/domain/1.0.0.0.8.b.d.0.1.0.0.2.ip6.arpa
~~~

### Searches

### Server Help

## Unknown Query Paramters
