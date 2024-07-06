# Finding Servers

RDAP servers exist for various types of Internet registration services:

1. top-level domain registries
1. second-level and below domain registries
1. domain registrars
1. regional internet registries
1. national internet registries
1. local internet registries
1. the IANA

The method to find the appropriate server to query depends on the type
of service being targeted. There are three basic mechanisms to find a server:

1. ["Bootstrapping"](iana.md) - This is the process defined by [RFC 9224](https://datatracker.ietf.org/doc/html/rfc9224) and [RFC 7480](https://datatracker.ietf.org/doc/html/rfc7480#autoid-28).
2. [Redirects](redirects.md) - HTTP redirects by another RDAP server as defined by [RFC 7480](https://datatracker.ietf.org/doc/html/rfc7480#autoid-10).
3. [Referrals](referrals.md) - Links from one RDAP server to another using a [`link`](../protocol/common_data_structures.html#links).

The process of getting to redirects or referrals always begins with "bootstrapping" in some form.
