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
[
  "vcard",
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
As jCard can support properties such as wedding anniversaries, photos, sounds, and arbitrary blobs of XML, this can be
another source of interoperability issues.

In practice, registries only use a subset and clients can limit their implementations to the following:

* [adr](#adr-property)
* [contact-uri](#contact-uri-property)
* [email](#email-property)
* [fn](#fn-property) -- **required**
* [kind](#kind-property) -- **required**
* [lang](#lang-property)
* [org](#org-property)
* [role](#role-property)
* [tel](#tel-property)
* [title](#title-property)
* [version](#version-property) -- **required**

With the exception of the `version` property, these properties may be repeated more than once and may have
`pref` parameter.

```json
[
  "vcard",
  [
    [ "version", {}, "text", "4.0"],
    [ "fn", {}, "text", "Alice Allison"],
    [ 
      "email", 
      { "pref": "1" },       // note: "1" is a string, not an integer
      "text", 
      "alice@example.com"
    ],
    [ 
      "email", 
      { "pref": "2" },       // note: "2" is a string, not an integer
      "text", 
      "theboss@example.org"
    ]
  ]
]
```

Though `pref` is an integer between 1 and 100 according to the vCard specification, it is represented
as a string in jCard. This is also another source of interoperability problems.

## `adr` Property

The `adr` property represents a postal address. This property is the most complicated and is a source
of many interoperability issues. Its complications arise from its dual nature of supporting both
unstructured and structured postal addresses. The following is the difference between them:

* An unstructured address does not have any categorized items such as city name or postal code. It is
simply a series of lines of text as would be found on a postal envelope.
* A structured address defines the parts of the address that are separate items, such as city name and postal code.

The following is an example of an unstructured postal address:

```json
[
  "adr",
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
[
  "adr",
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
[
  "adr",
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

### `adr` Property - `cc` Paramter

Using a country code instead of the full country name is a common mistake seen with structured
addresses, though in practice there is no real harm. However, the `cc` parameter is meant
to convey the country code.

gTLD server implementers should take note that the 
[ICANN profile](https://www.icann.org/en/system/files/files/rdap-response-profile-21feb24-en.pdf)
requires that the `cc` parameter be used and the country name is to be empty.

```json
[
  "adr",
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


### `adr` Property - `type` Paramater

The `adr` property may also have a `type` parameter which can be either "home" or "work". This
parameter applies to both structured and unstructured addresses.

```json
[
  "adr",
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

## `contact-uri` Property

The `contact-uri` property is defined in [RFC 8605](https://datatracker.ietf.org/doc/html/rfc8605)
and represents an alternate means of contact either via a web-form or an email address. This property
is purposely defined for use in RDAP.

```json
[
  "contact-uri"
  {},
  "uri",
  "https://example.com/contact-form"
]
```

This may be either an http, https, or mailto URI.

```json
[ 
  "contact-uri"
  {},
  "uri",
  "mailto:contact@example.com"
]
```

## `email` Property

The `email` property has a text value that is an email address.

```json
[ 
  "email"
  {},
  "text",
  "contact@example.com"
]
```

### `email` Property - `type` Parameter

Like the `adr` property, the `email` property may also have a `type` parameter than can be either "work" or "home".

```json
[ 
  "email"
  { "type": "work" },   // can be either "work" or "home"
  "text",
  "contact@example.com"
]
```

## `fn` Property

The `fn` property represents a persons "full name". Unlike most other properties, this is property is required
in jCard because, theoretically, jCard/vCard processors use this information as an index in a contact database.

```json
[ 
  "fn"
  {},
  "text",
  "Alice Allison"
]
```

Some servers are required to redact or omit a person's full name for various, legitmate policy reasons. In
those cases, the value of the `fn` property should be an empty string.

```json
[ 
  "fn"
  {},
  "text",
  ""      // EMPTY STRING
]
```

## `kind` Property

The `kind` property signifies the type of contact, such as an organization or individual. There can be
many values, but in practice it is either "individual", "org", or "group".

```json
[ 
  "kind"
  {},
  "text",
  "individual"  // "org", "individual", or "group"
]
```

At least one of these properties is required by jCard/vCard, however it is quite common for RDAP servers
to omit this property. Unlike the `fn` property, the value for `kind` must not be an empty string which
causes problems for servers which are not allowed to present this information for policy reasons.

Therefore, RDAP clients should be aware that this property may not be present.

## `lang` Property

The `lang` property specifies the language of the contact.

```json
[ 
  "lang"
  {},
  "language-tag",
  "fr" 
]
```

## `org` Property

The `org` property represents the name of the organization to which an individual belongs.

```json
[ 
  "org"
  {},
  "text",
  "Acme Rockets, LTD" 
]
```

## `role` Property

The `role` property represents the role of an individual within an organization or group.

```json
[ 
  "role"
  {},
  "text",
  "Coffee Fetcher" 
]
```

This should not be confused with the `role` array on the [entity](object_classes.md#entity).
This role relates an individual or group to the organization, while the other relates the
entity to the resource (i.e. domain, IP network).

## `tel` Property

The `tel` property represents a telephone number, and its value can be either free-form text
expressing the telphone number or a tel URI.


```json
[ 
  "tel"
  {},
  "text",             // note: type of "text"
  "1 (800) 555-1212" 
]
```

```json
[ 
  "tel"
  {},
  "uri",             // note: type of "uri"
  "tel:+18005551212" // must follow tel URI format
]
```

### `tel` Property - `type` Parameter

The `tel` property can have a `type` parameter, but unlike the `email` and other properties
the set of values describing the capabilities of the phone in addition to the values "work" and "home".

* text - capable of text messages
* voice - traditional telephone
* fax - facsimile machine
* cell - mobile phone
* video - has video/audio capabilities
* pager - old-style pager
* textphone - for those with hearing and speach challenges

```json
[ 
  "tel"
  {
    "type" : "voice"
  },
  "text",             
  "1 (703) 555-8888" 
]
```

When multiple types are needed, then an array of strings is used:

```json
[ 
  "tel"
  {
    "type" : [ "voice", "text", "work" ]
  },
  "text",             
  "1 (703) 555-8888" 
]
```

## `title` Property

Similar to the [role property](#role-property), the `title` property signifies the title or person, such as "Vice President of Sales".
So a person may have a role of "Project Leader" but their title may be "Software Engineer". There really is no good reason
for a contact in a domain or IP network registration database to have either of these types of meta-data about a person, but
some registries/registrars do collect this information.

```json
[ 
  "title"
  {},
  "text",             
  "Sanitation Engineer" 
]
```

## `version` Property

The `version` property is required in all jCards, there must only be one, 
its value is always "4.0" and it must always be the first element in
the "vCard" array.

```json
[ 
  "version"
  {},
  "text",             
  "4.0" 
]
```

Unfortunately, [RFC 9083](https://datatracker.ietf.org/doc/html/rfc9083#name-the-entity-object-class) does not restrict
the jCard/vCard version to "4.0", but in the unlikely event a new version of jCard/vCard were to be standardized it
would almost certainly be ignored as most RDAP server and client implementations of jCard are hard-coded.
