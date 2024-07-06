# HTTP

RDAP is intrinsically tied to being used over HTTP. [RFC 7480, Section 3](https://datatracker.ietf.org/doc/html/rfc7480#section-3) 
has this wording:

> This document only describes how RDAP is transported using HTTP with this format.

Some may interpret that sentence to mean RDAP is only defined over HTTP, which is true, but
it could also be interpreted to mean that another RFC may define RDAP over another transport,
which could also be true (in the future).

However, RDAP is intrinsically tied to being used over HTTP. While it may be possible to map RDAP onto
another transport, RDAP's explicit use of `http` and `https` URLs in addition to URL path
definitions and query parameters would likely make it a hard fit for any other mechanism.


## HTTPS

Support for HTTPS by both RDAP clients and servers is a requirement. This is stated in RFC 7480
in both Sections 4 and 7.

This wording does not preclude the use of insecure HTTP, though at the time of ratification
of RFC 7480 the [IESG](https://www.ietf.org/about/groups/iesg/) asked for the use of HTTPS
to be mandatory. It was not made mandatory at the time due to known issues with load balancers.
Should the IETF ever revisit this requirement, it almost certainly would mandate usage of HTTPS.

Therefore, it is best that all production deployments support HTTPS and redirect all insecure HTTP
queries to HTTPS, especially as all RDAP software is to support HTTPS to be compliant with the
standards. [For gTLDs](/specifications/icann.md), 
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
with HTTP is defined in [RFC 8470](https://datatracker.ietf.org/doc/html/rfc8470), which describes
a few measures to thwart replay attacks.

AS RDAP has no PUT, POST, DELETE, and PATCH methods and is only a data retrieval protocol for (mostly)
public data, replay attacks are not usually of great concern. However, RDAP server operators using authorization
to provided differentiated access to RDAP data should take the precautions outlined in 
[RFC 8470](https://datatracker.ietf.org/doc/html/rfc8470).

### TLSA Records

Authentication, that is the act of verifying the identity of the other party, in TLS uses X.509 (PKIX) cryptographic certificates.
In normal TLS usage, these certificates are transferred during the TLS handshake and verified by a client (and/or server) using
a set of pre-configured certificates that are either embedded in the software or are part of the operating system (or both). TLSA
is a means to use DNS to provide another source of those set of valid certificates in which to validate the other party in a TLS
handshake.

TLSA is part of the [DNS-Based Authentication of Named Entities (DANE)](https://datatracker.ietf.org/doc/html/rfc6698) specifications
and is defined in [RFC 7671](https://datatracker.ietf.org/doc/html/rfc7671).

Though usage of TLSA with RDAP is encouraged by [ICANN](/specifications/icann.md), for all practical purposes TLSA is not used
in RDAP despite its deployment by many gTLD registries. The reason for this is that there are no known HTTP client libraries that
support TLSA nor are there any known RDAP clients using TLSA. Additionally, the IETF has not published any guidelines on usage
of TLSA with HTTP as it has done with [SMTP](https://datatracker.ietf.org/doc/html/rfc7672). As outlined in this
[slide presentation from Shumon Huque](https://indico.dns-oarc.net/event/43/contributions/928/attachments/901/1648/dane-overview-shumon.pdf),
HTTP using protocols have a series of challenges for which TLSA is not ideal. Though not yet a standard, [RFC 9102](https://datatracker.ietf.org/doc/html/rfc9102)
describes an experimental feature of TLS to use DANE without TLSA.

Should an RDAP service wish to use TLSA, the following suggests are made in the absence of any other HTTP or RDAP specific
TLSA guidelines:
* Certificate usage should be 3 (DANE-EE) which identifies the certificate in use by the RDAP server.
* Selector should be 1 (SPKI) which ties the TLSA record to the Subject Public Key Identifier of the 
certificate thus avoiding the need to refresh the TLSA record when the certificate is re-issued unless it is also re-keyed.
* Matching type should be 1 (SHA-256) as SHA-512 is not mandatory to implement by DNSSEC client libraries.

Client implementers wishing to support TLSA should be aware of the requirements to do so, which are:
1. Either the client is to fully validate DNSSEC records or is to have a secure channel to a DNS resolver which does so.
2. There may be multiple TLSA records, so each record must be evaluated until one is found to work.
3. If no TLSA records are found to work, the TLS handshake must fall back to normal certificate validation.


