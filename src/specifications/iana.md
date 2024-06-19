# IANA Registries

## RDAP JSON Values Registry
This IANA registry contains the values found in various RDAP strings, as denoted by the `type` column.

<https://www.iana.org/assignments/rdap-json-values/rdap-json-values.xhtml>

## RDAP Extensions Registry
This registry holds the list of RDAP extensions and their identifiers, which are found
in the [`rdapConformance` array](../protocol/common_data_structures.md#rdapconformance).

Registering an extension requires that it be published somewhere, though not necessarily
as an IETF RFC.

<https://www.iana.org/assignments/rdap-extensions/rdap-extensions.xhtml>

## RDAP Bootstrap Registries

RDAP bootstrapping starts with these IANA registries:

| Type                      | Link                                          |
| ------------------------- | --------------------------------------------- |
| Forward DNS               | <https://data.iana.org/rdap/dns.json>         |
| IPv4 Addresses            | <https://data.iana.org/rdap/ipv4.json>        |
| IPv6 Addresses            | <https://data.iana.org/rdap/ipv6.json>        |
| Autonomous System Numbers | <https://data.iana.org/rdap/asn.json>         |
| Object Tags               | <https://data.iana.org/rdap/object-tags.json> |

## HTTP Status Codes
This IANA registry holds the HTTP status codes, which are used by RDAP,
and used as the `errorCode` value in [error responses](../protocol/error_responses.md).

<https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml>

## vCard/jCard Elements
Values for the various parameter names, property names, etc... for vCard/jCard
can be found here:

<https://www.iana.org/assignments/vcard-elements/vcard-elements.xhtml>

## Link Relations
The values in the `rel` JSON property of [link objects](../protocol/common_data_structures.md#links) can
be found here:

<https://www.iana.org/assignments/link-relations/link-relations.xhtml>

## Media Types
The values in the `type` JSON property of [link objects](../protocol/common_data_structures.md#links) can
be found here:

<https://www.iana.org/assignments/media-types/media-types.xhtml>


