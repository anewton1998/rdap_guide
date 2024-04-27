# The Protocol

The core of RDAP is a simple [REST](https://en.wikipedia.org/wiki/REST)-like protocol using [JSON](https://en.wikipedia.org/wiki/JSON).
It defines only the `GET` and `HEAD` HTTP methods, and URL paths are defined as patterns. The following is the output of the
[HTTPie](https://httpie.io/) program invoked to get information about `example.com` from IANA's RDAP servers:
`http https://rdap.iana.org/domain/example.com accept:application/rdap+json`.

```
GET /domain/example.com HTTP/1.1
Accept-Encoding: gzip, deflate, br
Connection: keep-alive
Host: rdap.iana.org
User-Agent: HTTPie/3.2.2
accept: application/rdap+json



HTTP/1.1 200 OK
Strict-Transport-Security: max-age=48211200; preload
access-control-allow-origin: *
content-length: 984
content-type: application/rdap+json
date: Sat, 27 Apr 2024 19:44:49 GMT
server: uvicorn

{
    "entities": [
        {
            "objectClassName": "entity",
            "roles": [
                "registrant"
            ],
            "vcardArray": [
                "vcard",
                [
                    [
                        "version",
                        {},
                        "text",
                        "4.0"
                    ],
                    [
                        "fn",
                        {},
                        "text",
                        "Internet Assigned Numbers Authority"
                    ],
                    [
                        "role",
                        {},
                        "text",
                        "Registrant"
                    ]
                ]
            ]
        }
    ],
    "events": [
        {
            "eventAction": "last changed",
            "eventDate": "1992-01-01T00:00:00+00:00"
        },
        {
            "eventAction": "registration",
            "eventDate": "1992-01-01T00:00:00+00:00"
        }
    ],
    "ldhName": "example.com",
    "links": [
        {
            "href": "https://rdap.iana.org/domain/example.com",
            "rel": "self",
            "type": "application/rdap+json",
            "value": "https://rdap.iana.org/domain/example.com"
        }
    ],
    "notices": [
        {
            "description": [
                "Terms of Service"
            ],
            "links": [
                {
                    "href": "https://www.icann.org/privacy/tos",
                    "rel": "alternate",
                    "type": "text/html"
                }
            ],
            "title": "Terms of Service"
        },
        {
            "description": [
                "Privacy Policy"
            ],
            "links": [
                {
                    "href": "https://www.icann.org/privacy/policy",
                    "rel": "alternate",
                    "type": "text/html"
                }
            ],
            "title": "Privacy Policy"
        }
    ],
    "objectClassName": "domain",
    "rdapConformance": [
        "rdap_level_0"
    ],
    "secureDNS": {
        "delegationSigned": false
    },
    "status": [
        "active"
    ]
}  
```

This output shows an HTTP GET request of `/domain/example.com` from the client with a 200 OK response from the server.
The response contains JSON, most of which is directly defined in [RFC 9083](https://datatracker.ietf.org/doc/html/rfc9083).
The parts not defined in RFC 9083 are [jCard](/misc/glossary.md#jcard) which is a JSON encoding of [vCard](/misc/glossary.md#vcard),
the standard most users encounter when exchanging contact data (aka business cards) over email.

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
