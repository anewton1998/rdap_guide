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
many network operators create scripts to query RDAP for very legitimate, operational reasons
such as log analysis, inventory management, intrusion detection, etc...

## Writing A Simple RDAP Client

Let's say you are a network operator and are given a text file with IP addresses with the task
of finding their registered CIDR blocks, perhaps for creating an access control list. 

```
{{#include ip_inventory.txt}}  
```

You have been asked to provide this information in a CSV file.

Using a simple [bash](https://www.gnu.org/software/bash/) script, the [curl](https://curl.se/) command,
and [jq](https://jqlang.github.io/jq/), you come up with this very simple solution:


```bash
{{#include ip_inventory.sh}}  
```

How does this work? First, the script pipes the inventory text file into a loop and executes a curl command
for each IP address. Then `curl` pipes its output to `jq` which appends the information to the CSV file.

Here is what to note about the `curl` command:
1. the `-s` suppresses progress output so only JSON is passed to `jq`.
1. the `accept` header is set to the RDAP media type.
1. the target is a bootstrap server that redirects queries to the appropriate authoritative server.
1. the `-L` instructs `curl` to follow redirects.

These last two items are important to understand. The set of IP addresses are not all registered with the
same RIR, yet the script has no logic to find the right server to query. Instead, all queries are sent
to a "redirect" server which then redirects `curl` to the right place.

The `jq` command does the parsing of the JSON and converting it to CSV. There are a couple of
interesting things to note here:
1. "startAddress" and "endAddress" are defined in [RFC 9083](https://datatracker.ietf.org/doc/html/rfc9083)
1. "cidr0_cidrs" is from an RDAP extension used by [the RIRs for expressing CIDR blocks](https://bitbucket.org/nroecg/nro-rdap-cidr/src/master/nro-rdap-cidr.txt).

The final result is this CSV file:

```csv
{{#include ip_inventory.csv}}  
```
