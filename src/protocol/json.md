# JSON

The JSON used in RDAP is defined in [RFC 9083](https://datatracker.ietf.org/doc/rfc9083/).
This JSON is described in two broad categories, common data types and object classes.
An object class defines registrations in a registry and they are definitions commonly
found among those definitions are the common data types.

[RFC 9083](https://datatracker.ietf.org/doc/rfc9083/) also defines arrays for the
results of searches, which are composed of the object classes, error responses, and
the special response to a `/help` query.

Readers should be aware that the JSON used throughout this section is annotated
with comments for illustrative purposes. However, legal JSON does not allow comments.
