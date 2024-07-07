# Bash: Finding IP Networks

Let's say you are a network operator and are given a text file with IP addresses with the task
of finding their registered CIDR blocks, perhaps for creating an access control list. 

```
{{#include ../examples/clients/bash_curl_jq/ip_inventory.txt}}  
```

You have been asked to provide this information in a CSV file.

Using a simple [bash](https://www.gnu.org/software/bash/) script, the [curl](https://curl.se/) command,
and [jq](https://jqlang.github.io/jq/), you come up with this very simple solution
(source code is [here](https://github.com/anewton1998/rdap_guide/tree/main/src/examples/clients/bash_curl_jq)):


```bash
{{#include ../examples/clients/bash_curl_jq/ip_inventory.sh}}  
```

How does this work? First, the script pipes the inventory text file into a loop and executes a curl command
for each IP address. Then `curl` pipes its output to `jq` which appends the information to the CSV file.

Here is what to note about the `curl` command:
1. The `-s` suppresses progress output so only JSON is passed to `jq`.
1. The `accept` header is set to the RDAP media type.
1. The target is a bootstrap server that redirects queries to the appropriate authoritative server.
1. The `-L` instructs `curl` to follow redirects.

These last two items are important to understand. The set of IP addresses are not all registered with the
same RIR, yet the script has no logic to find the right server to query. Instead, all queries are sent
to a "redirect" server which then redirects `curl` to the right place.

The `jq` command does the parsing of the JSON and converting it to CSV. There are a couple of
interesting things to note here:
1. "startAddress" and "endAddress" are defined in [RFC 9083](https://datatracker.ietf.org/doc/html/rfc9083)
1. "cidr0_cidrs" is from an RDAP extension used by [the RIRs for expressing CIDR blocks](https://bitbucket.org/nroecg/nro-rdap-cidr/src/master/nro-rdap-cidr.txt).

The final result is this CSV file:

```csv
{{#include ../examples/clients/bash_curl_jq/ip_inventory.csv}}  
```
