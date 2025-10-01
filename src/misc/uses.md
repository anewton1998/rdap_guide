# Uses of RDAP

Being the replacement protocol for Whois, RDAP is used in much the same way for the
two primary use cases[^use_cases]:

* To find information about IP networks and ASNs.
* To find information about domain names.

Similar to Whois, in the RDAP ecosystem there are RDAP clients that talk to RDAP servers. The clients
execute RDAP queries (sometimes for people, but most often as automation), and the servers are
operated by domain registries and registrars (see [DNRs](../misc/glossary.md#dnr)) or 
[Regional Internet Registries](../misc/glossary.md#rir).

Unlike Whois[^official_features], RDAP has standardized and structured output, protocol-level redirects,
links to other resources (including other RDAP resources), formalized methods to 
find authoritative servers, and much more.

## Who Actually Uses RDAP?

There is a common misperception that the main users of RDAP, and Whois, are humans.
While there are no hard numbers for the total number of RDAP or Whois queries
across all authoritative servers, there are a couple of readily available data points
to suggest that most RDAP queries are "bots". 

ARIN, just one of the 5 [RIRs](/misc/glossary.md#rir),
[reports](https://www.arin.net/vault/participate/meetings/reports/ARIN52/materials/friday/arin52_engineeringupdate.pdf) 
upwards of 700 million queries per month. And <https://rdap.org>, which is a redirect service
and not an authoritative server, [reports](https://mailarchive.ietf.org/arch/msg/regext/ElTMpcFDeZ_L43U9UbKqaBowjak/)
daily query rates of 10 million (approximately 300 million per month).
According to ICANN as of September 2025 the gTLD registries report a monthly query rate of 600 million.
Therefore, it is likely there are over 1 billion queries per month across the RDAP ecosystem.

In terms of web traffic, any websites pulling in 300 million monthly hits would be in the top 50 of the world, and
1 billion human interactions for anything would get a lot of attention from enterprising advertisers or others with
monetization efforts.
If RDAP was pulling in anywhere close to this many "eyeballs", we would all know it.

Therefore, these queries are from "bots" or more plainly said, scripts. This is stated in [RFC 7480, Section 3](https://datatracker.ietf.org/doc/html/rfc7480#section-1):

> A client implementation should be possible using 
> common operating system scripting tools 
> (e.g., bash and wget).

While it is well-known there is a lot of automation used for data mining of both RDAP and Whois,
many network operators create scripts to query RDAP for very legitimate, operational reasons
such as log analysis, inventory management, intrusion detection, etc...

## Writing Simple RDAP Clients

Writing custom RDAP clients can be quite easy. Here are some examples in this chapter:

1. [A Bash Shell Example: Finding IP Networks](finding_ip_networks.md) - finding the start, end addresses, and CIDR blocks of public IP networks.
1. [A Python Example: Finding Protected Domains](finding_protected_domains.md) - list domains that are not protected.
1. [A Rust Example: Networks of Nameservers](networks_of_nameservers.md) - get the networks of the nameservers of a list of domain names.

[^use_cases]: Whois is also used for other purposes, but these are the ones that generate most traffic and have the highest uses.

[^official_features]: Whois can do some of these things, but they are not standardized and vary depending on client.
