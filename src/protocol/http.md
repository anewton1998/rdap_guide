# HTTP

RDAP is intrinsically tied to being used over HTTP. [RFC 7480, Section 3](https://datatracker.ietf.org/doc/html/rfc7480#section-3) 
has this wording:

> This document only describes how RDAP is transported using HTTP with this format.

Some may interpret that sentence to mean RDAP is only defined over HTTP, which is true, but
it could also be intepretted to mean that another RFC may define RDAP over another transport,
which could also be true (in the future).

However, RDAP is intrinsically tied to being used over HTTP. While it may be possible to map RDAP onto
another transport, RDAP's explicit use of `http` and `https` URLs in addition to URL path
definitions and query parameters would likely make it a hard fit for any other mechanism.


## HTTPS

Support for HTTPS by both RDAP clients and servers is a requirement. This stated in RFC 7480
in both Sections 4 and 7.

This wording does not preclude the use of insecure HTTP, though at the time of ratification
of RFC 7480 the [IESG](https://www.ietf.org/about/groups/iesg/) asked for the use of HTTPS
to be manditory. It was not made manditory at the time due to known issues with load balancers.
Should the IETF ever revisit this requirement, it almost certainly would mandate usage of HTTPS.

Therefore, it is best that all production deployments support HTTPS and redirect all insecure HTTP
queries to HTTPS, especially as all RDAP software is to support HTTPS to be compliant with the
standards. [For gTLDs](https://www.icann.org/en/system/files/files/rdap-technical-implementation-guide-15feb19-en.pdf), 
HTTPS is the only allowable form of RDAP.

## HTTP Versions

When RDAP was ratified in 2015, HTTP 1.1 was the most current version of HTTP. RFC 7480 makes a
direct reference to [RFC 7230](https://datatracker.ietf.org/doc/html/rfc7230), which is the clarified
HTTP 1.1 specification. However, at that time HTTP 2.0 was in progress within the IETF and RFC 7480
contains this sentence:

> This protocol is forward compatible with HTTP 2.0.

So which version of HTTP is required by the RDAP specifications? That would be HTTP 1.1.

However, this does not preclude service of RDAP over HTTP 2.0 or 3.0. And practically speaking,
most server and client libraries and frameworks that support HTTP 2.0 and/or 3.0 also support 1.1
at the time of this writing.

## TLS

Since 2021, the [IETF has deprecated versions of TLS below 1.2](https://www.rfc-editor.org/rfc/rfc8996.html) 
(i.e. SSLv1, SSLv2, SSLv3, TLS 1.0 and TLS 1.1). According to [RFC 9325](https://datatracker.ietf.org/doc/html/rfc9325),
both servers and clients MUST NOT negotiate a downgrade to these protocols.

[RFC 9325](https://datatracker.ietf.org/doc/html/rfc9325) also lists a number of issues server operators
must take when using TLS 1.2. As these issues are not present with TLS 1.3, it makes more sense for
most server operators to only support TLS 1.3.

Practically speaking, this means clients must support both TLS 1.2 and TLS 1.3.

Even with TLS 1.3, there are a few precautions both clients and servers should take.

1. Servers should abort a handshake when the server name in the SNI does not match. Clients should do the same unless explicitly commanded to ignore the mismatch by the user.
2. Clients and servers should abort a handshake when there is not agreed upon protocol in the ALPN.
3. Servers should use key sizes no smaller than the following:
   1. DH - 2048 bits
   2. ECDH - 224 bits
   3. RSA - 2048 bits w/ SHA-256

### TLS 1.3 Early Data

TLS 1.3 has a feature known as "Early Data" or zero round-trip time (0-RTT). How this feature is used
with HTTP is defiend in [RFC 8470](https://datatracker.ietf.org/doc/html/rfc8470), which describes
a few measures to thwart replay attacks.

AS RDAP has no PUT, POST, DELETE, and PATCH methods and is only a data retreival protocol for (mostly)
public data, replay attacks are not usually of great concern. However, RDAP server operators using authorization
to provided differentiated access to RDAP data should take the precautions outlined in 
[RFC 8470](https://datatracker.ietf.org/doc/html/rfc8470).
