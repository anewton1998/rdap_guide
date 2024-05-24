# Common Data Types

The common data types are simply definitions of JSON that are commonly found
in the [object classes](protocol/object_classes.md).

## The RDAP Conformance Array

This is a special array that must only appear in the top most JSON of an RDAP
response. It is used by a server to signal to a client the RDAP verion supported
by the server (there is only one) and any extensions.

```javascript
"rdapConformance" :
[
  "rdap_level_0"
]
```

In the original versions of RDAP ([RFC 7483](https://datatracker.ietf.org/doc/rfc7483/)),
this array was not mandatory. However, [RFC 9083](https://datatracker.ietf.org/doc/rfc9083/)
made it a requirement of all responses.

```javascript
"rdapConformance" :
[
  "rdap_level_0",     // rdap version 0
  "lunarNIC_level_0"  // an extension
]
```

When used in response to a `/help` query, this array must list all extensions
the server supports. However, when used for any other response it must only list
the extensions used in that response.
