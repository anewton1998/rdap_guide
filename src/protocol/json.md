# JSON

The JSON used in RDAP is defined in [RFC 9083](https://datatracker.ietf.org/doc/rfc9083/).
This JSON is described in two broad categories, common data types and object classes.
An object class defines registrations in a registry, and they are definitions commonly
found among those definitions are the common data types.

[RFC 9083](https://datatracker.ietf.org/doc/rfc9083/) also defines arrays for the
results of searches, which are composed of the object classes, error responses, and
the special response to a `/help` query.

---

Readers should be aware that the JSON used throughout this section is annotated
with comments for illustrative purposes. However, legal JSON does not allow comments.

---


## Common Data Types

[RFC 9083](https://datatracker.ietf.org/doc/rfc9083/) defines the formats for
various string types used in the JSON for things like IP addresses, URIs, etc...
Most are just references to the canonical RFCs defining their formats.
However, there are two definitions that implementers and users of RDAP should
take note:

### handle

A handle is just a reference, often an opaque one (i.e. without meaning). This
term comes for the older Whois era in which registrations at the InterNIC were
identified by a "handle".

[RFC 9083](https://datatracker.ietf.org/doc/rfc9083/) defines them as so:

> [DNRs](../misc/glossary.md#dnr) and [RIRs](../misc/glossary.md#rir) have 
> registry-unique identifiers that may be used to 
> specifically reference an object instance. The semantics of this data type 
> as found in this document are to be a registry-unique reference to the 
> closest enclosing object where the value is found. The data type names 
> "registryId", "roid", "nic-handle", "registrationNo", etc., are terms 
> often synonymous with this data type. In this document, the term "handle" 
> is used. The term exposed to users by clients is a presentation issue 
> beyond the scope of this document. This value is a simple character string.

### date and time

Date and time values in RDAP are expressed using [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339)
format, which is a simplified profile of ISO 8601 date and time. Therefore,
if an RFC 3339 specific library is not available, then an ISO 8601 can be used
for parsing and, with some wrapping, serializing.

Here are some examples:

* `1990-12-31T23:59:59Z` - 31st of December 1990, at 1 second before midnight.
* `1990-12-31T23:59:60Z` - a leap second at the end of 1990.
* `1996-12-19T16:39:57-08:00` - a date and time offset from UTC by -8 hours.

The benefit of using [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339)
date and time values is that they are somewhat readable to most humans, but they
also can be sorted alphabetically. 
