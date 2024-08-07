# Server Implementations

RDAP servers come in two basic flavors, authoritative server and redirect servers. [Authoritative servers](authoritative.md) contain the
information for domain names, IP addresses and ASNs, whereas [redirect servers](redirect.md) issue HTTP level redirects to clients
to assist them to get to the right authoritative server. Sometime, as in the case of the [RIRs](../misc/glossary.md#rir), servers
can be both authoritative for their own information and issue redirects for other information.

Listed under server implementations are [conformance tools](conformance_tools.md), which are actually clients developed to make sure
servers conform to standards. They are listed here because they are of little use to the general-public and typically only used by
server operators.
