# The Protocol

The core of RDAP is a simple [REST](https://en.wikipedia.org/wiki/REST)-like protocol using [JSON](https://en.wikipedia.org/wiki/JSON).

```svgbob
+-----------------------------+
|  RDAP                       |
+-----------------------------+
|  JSON                       |
+-----------------------------+
|  HTTP / HTTPS               |
+-----------------------------+
```

It defines only the `GET` and `HEAD` HTTP methods, and URL paths are defined as patterns. The following is the output of the
[HTTPie](https://httpie.io/) program invoked to get information about `example.com` from IANA's RDAP servers:
`http https://rdap.iana.org/domain/example.com accept:application/rdap+json`.

```
{{#include http_example_com.out}}    
```

This output shows an HTTP GET request of `/domain/example.com` from the client with a 200 OK response from the server.
The response contains JSON, most of which is directly defined in [RFC 9083](https://datatracker.ietf.org/doc/html/rfc9083).
The parts not defined in RFC 9083 are [jCard](/misc/glossary.md#jcard) which is a JSON encoding of [vCard](/misc/glossary.md#vcard),
the standard most users encounter when exchanging contact data (aka business cards) over email.

Breaking down the output, the following is the HTTP request:

```
{{#include http_example_com.out::6}}    
```

This is all normal HTTP semantics. The RDAP specific parts are the URL, which has the defined `/domain` path (See [Lookups and Searches](#lookups-and-searches)).
The other RDAP specific part is the media type of `application/rdap+json` in the `accept` header.

The next section shows the HTTP response.

```
{{#include http_example_com.out:9:16}}    
```
Here, the important parts to note are the media type in the `content-type` header, which uses the RDAP media type,
and the `access-control-allow-origin` header, which is used to allow web browsers to run JavaScript sourced from one
website to use the RDAP content from an RDAP HTTP server.

Next, the JSON returned can be broken down as follows. The first part is the "entity", which is the contact for the domain:

```json
{{#include http_example_com.out:19:49}}    
```

This is followed by the events, which show the date and time for the major changes to the domain itself:

```json
{{#include http_example_com.out:50:59}}    
```

Next is the domain name itself. This is called `ldhName` where "ldh" is short for "letters, digits, hyphens" referring to
restriction of DNS names to be ASCII letters, digits or hyphens. There is a separate JSON value for Internationalized Domain Names (IDNs).

```json
{{#include http_example_com.out:60:60}}    
```

Next is the link to the domain, used by some clients for caching purposes. Note that the `rel` value is `self`
and the `type` value is the RDAP media type of `application/rdap+json`.

```json
{{#include http_example_com.out:61:68}}    
```

Next are notices from the server operator:

```json
{{#include http_example_com.out:69:96}}    
```

Next, the following is given:
1. The type of the object being returned is given. This is used by clients to determine the type of object being returned.
1. The `rdapConformance` array, which lists the extensions in use by the RDAP server.
1. The data signifying if `example.com` is signed in the DNS (i.e. DNSSEC).
1. The status of the domain, which is active.

```json
{{#include http_example_com.out:97:106}}    
```

## Lookups and Searches

The example above shows a query for a specific domain registration. RDAP breaks down queries into two types: lookups and searches.
Queries for a specific registry item or object, such as the one above, are lookups. Queries for multiple registry objects are searches.
Each query is specified by a unique path.

[RFC 9082](https://datatracker.ietf.org/doc/html/rfc9082) defines six types of lookups:

* `/ip` - lookups of IP addresses (either individual addresses or IP CIDR blocks).
* `/autnum` - lookups of Autonomous System numbers.
* `/domain` - lookups of domain registrations.
* `/nameserver` - lookups of DNS nameserver registrations.
* `/entity` - lookups of entities (aka contacts).
* `/help` - lookup of server help information.

RFC 9082 defines the following searches:

* domain searches
  * `/domains?name=<DOMAIN SEARCH PATTERN>` - searches for domain registrations by a domain name search pattern.
  * `/domains?nsLdhName=<NAMESERVER SEARCH PATTERN>` - searches for domain registrations by a nameserver host name search pattern of a domain.
  * `/domains?nsIp=<NAMESERVER IP ADDRESS>` - searches for domain registrations by a nameserver IP address of a domain.
* nameserver searches
  * `/nameservers?name=<NAMESERVER SEARCH PATTERN>` - searches for nameserver objects by a hostname search pattern.
  * `/nameservers?ip=<NAMESERVER IP ADDRESS>` - searches for nameserver objects by a nameserver IP address.
* entity searches
  * `/entities?fn=<ENTITY NAME SEARCH PATTERN>` - searches for entities by a name pattern.
  * `/entities?handle=<ENTITY HANDLE SEARCH PATTERN>` - searches for entities by a handle pattern.

  These are the core queries defined by RDAP. However, RDAP has an extension mechanism that allows for other
  lookups and searches to be defined.
