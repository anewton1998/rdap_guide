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
The parts not defined in RFC 9083 are [jCard](jcard_and_vcard.md) which is a JSON encoding of [vCard](/misc/glossary.md#vcard),
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

Next comes the JSON, which has no strict order (because JSON defines none and RDAP enforces none). To help make sense
of this, the sections of JSON are re-ordered for educational purposes.

The [`rdapConformance` array](common_data_structures.md#rdapconformance) is an RDAP structure containing protocol and extension
compatibility information. At a minimum, it must contain the string "rdap_level_0". This array shows up in every RDAP response.
Here, it is lines 98-100:

```json
{{#include http_example_com.out:98:100}}    
```

Each RDAP response is either a single object (the result of a [lookup](rdap_urls.md#lookups)) or an array of objects
(the result of a [search](rdap_urls.md#searches)). Every object must have an [`objectClassName`](common_data_structures.md#objectclassname)
to inform the client which object(s) is in the response. Here it is on line 97:

```json
{{#include http_example_com.out:97:97}}    
```

Lines 60 and lines 101 to 106 have information specific to [domain objects](object_classes.md#domain). Line 60 describes the 
ASCII version of the domain name:

```json
{{#include http_example_com.out:60:60}}    
```

While lines 101 to 106 describe the state of the domain:

```json
{{#include http_example_com.out:101:106}}
```

And this domain object has other information embedded in it using [common data structures](common_data_structures.md) found in
all the objects classes:

* Lines 19 to 49: [`entities`](object_classes.md#entity-children) or the domain's "contacts".
* Lines 50 to 59: [`events`](common_data_structures.md#events) such as when the domain was first registered.
* Lines 61 to 68: [`links`](common_data_structures.md#links) to other information relevant to the domain.

Finally, there are [`notices`](common_data_structures.md#notices-and-remarks) from the server operator (lines 69 to 96):

```json
{{#include http_example_com.out:69:96}}    
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
