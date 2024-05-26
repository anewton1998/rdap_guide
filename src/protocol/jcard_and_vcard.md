# jCard and vCard

RDAP does not define contact data, instead it re-uses a specification called [jCard](https://datatracker.ietf.org/doc/rfc7095/),
which is a programmatic representation of [vCard](https://datatracker.ietf.org/doc/rfc6350/)... and why it looks so different
than the rest of RDAP. When RDAP was going through the specification process, it was believed that using jCard would allow
RDAP to benefit from a multitude of jCard implementations. Those jCard implementations never materialized and every RDAP
server and client must directly implement jCard.

The use of jCard in RDAP is considered to be the biggest mistake in the development of the protocol, and now there are
current efforts in the [REGEXT](../misc/glossary.md#regext) to find a replacement, though there is controversy around
this as well for the same reason... this time the adoption of JSContact which also has no known production implementations.

## Properties

The individual items of vCard and jCard, such as phone numbers, email addresses, postal addresses, etc..., are called
properties. Properties are represented in an array, and each has a name, parameters, a type, and a value or values which
occupy a specific position in the array.

```json
[
  "fn",   // property name
  {},     // object containing property parameters
  "text", // type of the value(s)
  "Bob"   // the value (could be more than one)
]
```

Each property is put into an array, which is the second element of an outer array.

```json
["vcard",
  [
    ["version", {}, "text", "4.0"],
    ["fn", {}, "text", "Alice Allison"],
    ["email", {}, "text", "alice@example.com"]
  ]
]
```

These nested arrays mimic the vCard structure but serve no purpose other than to be a source of
interoperability problems.

Another unfortunate issue is that [RFC 9083](https://datatracker.ietf.org/doc/rfc9083/) does not restrict the types of
properties that may be used, leaving RDAP client implementers to guess which parts of jCard/vCard to implement.
As jCard can support properties such as wedding anniversaries, photos, and arbitrary blobs of XML, this can be
another source of interoperability issues.

In practice, registries only use a subset and clients can limit their implementations to the following:

* adr
* contact-uri
* email
* fn -- required
* kind
* lang
* org
* role
* tel
* title
* version -- required

## "adr" Property

The "adr" property represents a postal address. This property is the most complicated and is a source
of many interoperability issues. Its complications arise from its dual nature of supporting both
unstructured and structured postal addresses. The following is the difference between them:

* An unstructured address does not have any categorized items such as city name or postal code. It is
simply a series of lines of text as would be found on a postal envelope.
* A structured address defines the parts of the address that are separate items, such as city name and postal code.

The following is an example of an unstructured postal address:

```json
["adr",
  {
    "label":"Mail Stop 3\nSuite 3000\n123 Maple Ave\nQuebec\nQC\nG1V 2M2\nCanada\n"
  },
  "text",
  [
    "", "", "", "", "", "", ""
  ]
],
```

This is an example of the same address in structured form:

```json
["adr",
  {},
  "text",
  [
    "Mail Stop 3",
    "Suite 3000",
    "123 Maple Ave",
    "Quebec",
    "QC",
    "G1V 2M2",
    "Canada"
  ]
],
```

Note that the unstructured address must have a 7 element array of empty strings as the value
to conform to the vCard serialization format.

The value array for a structured address must be 7 elements exactly, though some may be empty.
Each position in the array has a specific meaning.

```json
["adr",
  {},
  "text",
  [
    "Mail Stop 3",   // post office box (not recommended for use)
    "Suite 3000",    // apartment or suite (not recommended for use)
    "123 Maple Ave", // street address
    "Quebec",        // locality or city name
    "QC",            // region (can be either a code or full name)
    "G1V 2M2",       // postal code
    "Canada"         // full country name
  ]
],
```

### "adr" Property "cc" Paramter

Using a country code instead of the full country name is a common mistake seen with structured
addresses, though in practice there is no real harm. However, the "cc" parameter is meant
to convey the country code.

gTLD server implementers should take note that the 
[ICANN profile](https://www.icann.org/en/system/files/files/rdap-response-profile-21feb24-en.pdf)
requires that the "cc" parameter be used and the country name is to be empty.

```json
["adr",
  {
    "cc": "CA"  // defined in RFC 8605 as a 2 character code
  },
  "text",
  [
    "Mail Stop 3",   
    "Suite 3000",    
    "123 Maple Ave", 
    "Quebec",        
    "QC",            
    "G1V 2M2",       
    ""         // EMPTY STRING
  ]
],
```


### "adr" Property "type" Paramater

The "adr" property may also have a "type" parameter which can be either "home" or "work". This
parameter applies to both structured and unstructured addresses.

```json
["adr",
  {
    "type": "work",  // allowed values are "home" and "work"
    "label":"Mail Stop 3\nSuite 3000\n123 Maple Ave\nQuebec\nQC\nG1V 2M2\nCanada\n"
  },
  "text",
  [
    "", "", "", "", "", "", ""
  ]
],
```

