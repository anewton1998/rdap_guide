# Headers

As RDAP uses HTTP and therefore any of the headers used by HTTP may be applicable. However, there are a few that are manditory to
use and others that are suggested to use. Implementers should be aware that HTTP header names are often capitalized (e.g. `User-Agent`)
but in reality they are case insensitive.

Using the example from earlier, we can see the various headers in both the request and response:

```
{{#include http_example_com.out::16}}    
```

## The Accept and Content-Type Headers

[RFC 9083](https://datatracker.ietf.org/doc/html/rfc9083#name-rdap-json-media-type-regist) defines the media type (formally called a MIME type)
used by RDAP, which is `application/rdap+json`.

Usage of this media type is defined in [RFC 7480](https://datatracker.ietf.org/doc/html/rfc7480#section-4.2):

> To indicate to servers that an RDAP response is desired, clients include an Accept header field with an RDAP-specific JSON media type,
> the generic JSON media type, or both.  Servers receiving an RDAP request return an entity with a Content-Type header containing the
> RDAP-specific JSON media type.

This means clients should use `application/rdap+json`, `application/json` or both in the `accept` header, but servers must return
`application/rdap+json` in the `content-type` header.

## Cross-Origin Resource Sharing (CORS)

This header helps RDAP clients running in a web-browser to query RDAP servers by lifting the "same-origin" restriction browers usually
place on in-browser applications. [RFC 7480](https://datatracker.ietf.org/doc/html/rfc7480#section-5.6) recommends setting it to "*":

    access-control-allow-origin: *

The [ICANN gTLD specifications](/specifications/icann.md) mandate the usage of this header.

## HTTP Strict Transport Security (HSTS) Header

Though not specified in any RDAP specification, usage of the HTTP Strict Transport Security (HSTS) header, defined in
[RFC 6797](https://datatracker.ietf.org/doc/html/rfc6797), is good practice and beneficial to in-browser RDAP clients.
The `max-age`, which is the number of seconds a client should remember a site should always use HTTPS,
should be a very long value as RDAP servers should always use HTTPS. One suggestion would be the number of seconds in a year,
31,541,000.

    strict-transport-security: max-age=3154100

## The Host Header

Also not explicitly stated in any of the RDAP specifications, it is recommended that clients send requests with a `host` header
because many RDAP servers for ccTLDs and gTLDs server more than one TLD. The `host` header is sometimes used by RDAP servers to
route requests to the correct back-end service. This is especially helpful to servers when responding to a `/help` request as
there is nothing the URL of the request that may be used for request routing.
