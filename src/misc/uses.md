# Uses of RDAP

There is common misperception that the main users of RDAP, and Whois, are humans.
While there are no hard numbers for the total number of RDAP or Whois queries
across all authoritative servers, there are a couple of readily available data points
to suggest that most RDAP queries are "bots". 

ARIN, just one of the 5 [RIRs](/misc/glossary.md#rir),
[reports](https://www.arin.net/vault/participate/meetings/reports/ARIN52/materials/friday/arin52_engineeringupdate.pdf) 
upwards of 700 million queries per month. And <https://rdap.org>, which is a redirect service
and not an authoritative server, [reports](https://mailarchive.ietf.org/arch/msg/regext/ElTMpcFDeZ_L43U9UbKqaBowjak/)
daily query rates of 10 million (approximately 300 million per month).

In terms of web traffic, any web sites pulling in 300 million monthly hits would be in the top 50 of the world.
If RDAP was pulling in anywhere close to this many eyeballs, we would all know it.

Therefore these queries are from "bots" or more plainly said, scripts. This is stated in [RFC 7480, Section 3](https://datatracker.ietf.org/doc/html/rfc7480#section-1):

> A client implementation should be possible using 
> common operating system scripting tools 
> (e.g., bash and wget).

While it is well-known there is a lot of automation used for data mining of both RDAP and Whois,
many network operators create scripts to query RDAP for very legitimate, operational reasons.
