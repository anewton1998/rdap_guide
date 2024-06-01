# Extensions

RDAP has an extension mechanism in which each extension is signaled by an
extension identifier. These identifiers must start with an alphabetic
character and may contain any combination of alphabetic characters,
numeric characters (digits), and the underscore ("_") character.

These extension identifiers are:
1. prepended to the names of JSON values.
1. preceed path segments in URLs.
1. placed in the RDAP conformance array.

As simple as this sounds, some of these rules have been bent over the
years causing some ambiguity. The [IETF is currently working](https://datatracker.ietf.org/doc/draft-ietf-regext-rdap-extensions/) 
to correct some of these issues.

## JSON

[RFC 9083](https://datatracker.ietf.org/doc/html/rfc9083#name-use-of-json) gives an example of
how these extension identifiers are used in JSON.

```json
{
  "handle" : "ABC123",
  "objectClassName" : "entity"
  "lunarNIC_beforeOneSmallStep" : "TRUE THAT!",    // simple string
  "remarks" :
  [
    {
      "description" :
      [
        "She sells sea shells down by the sea shore.",
        "Originally written by Terry Sullivan."
      ]
    }
  ],
  "lunarNIC_harshMistressNotes" : // an array
  [
    "In space,",
    "nobody can hear you scream."
  ]
}
```

Left unsaid by the RFC, this is an example of the extension defining JSON that
augments JSON defined by the core of RDAP. See [the example](#urls) below for
the definition of a new object class by the extension.

In essense, the extension identifier prepends any JSON the extension defines.

```svgbob
.-----------------------------------------------.
|                                               |
|     +---- "extension identifier"              |
|     |                                         |
|     v                                         |
| "lunarNIC_beforeOneSmallStep"                 |
|                ^                              |
|                |                              |
|                +---- "defined by extension"   |
|                                               |
'-----------------------------------------------'
```

The rule of thumb for clients laid out in [RFC 9083](https://datatracker.ietf.org/doc/html/rfc9083#name-use-of-json)
is to ignore any JSON the client is prepared to process.

## URLs

[RFC 9082](https://datatracker.ietf.org/doc/html/rfc9082#name-extensibility) defines how extensions
are to be used in URLs. Here, the extension identifier is used to prepend URL paths.

```svgbob
.------------------------------------------------.
|                                                |
| "extension identifier"---+                     |
|                          |                     |
|                          v                     |
|   "https://rdap.lunar/lunarNIC_crater/alpha92" |
|                                  ^      ^      |
|                                  |      |      |
|  "extension defined query"-------+      |      |
|                                         |      |
|           "lookup of crater alpha92"----+      |
|                                                |
'------------------------------------------------'
```

To round out this example, the URL `https://rdap.lunar/lunarNIC_crater/alph92` might yield
the following response:

```json
{
  "rdapConformance" :
  [
    "rdap_level_0",
    "lunarNIC"
  ],
  "objectClassName" : "lunarNIC_crater",
  "lunarNIC_handle" : "alpha92",
  "lunarNIC_coordinates" : {
    "quadrant_id": "b0",
    "surveyed" : false
  }
}

```

RFC 9082 does not define the use of extension identifiers in URL query parameters or as the only
part of the path segment. As mentioned above, fixes to this are in active discussions in the IETF.

## RDAP Conformance Array

The [RDAP conformance array](common_data_types.html#the-rdap-conformance-array) is a data structure 
that is a property of the JSON object returned as an RDAP response. It does not appear anywhere but 
as a direct child property of this JSON object (i.e. "top-most").

The extension identifier gets listed in this array:

```json
"rdapConformance" :
[
  "rdap_level_0",
  "lunarNIC"       // lunarNic extension identifier
]
```

The following is the example from RFC 9083 modified to show the RDAP conformance
array.

```json
{
  "rdapConformance" :
  [
    "rdap_level_0",
    "lunarNIC"       // lunarNic extension identifier
  ],
  "handle" : "ABC123",

  // simple string defined by the lunarNIC extension
  "lunarNIC_beforeOneSmallStep" : "TRUE THAT!",   

  "remarks" :
  [
    {
      "description" :
      [
        "She sells sea shells down by the sea shore.",
        "Originally written by Terry Sullivan."
      ]
    }
  ],

  // an array defined by the lunarNIC extension
  "lunarNIC_harshMistressNotes" : 
  [
    "In space,",
    "nobody can hear you scream."
  ]
}

```
