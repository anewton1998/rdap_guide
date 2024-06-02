# Common Data Types

The common data types are simply definitions of JSON that are commonly found
in the [object classes](protocol/object_classes.md). These are:

* [`rdapConformance`](#rdapconformance)
* [`links`](#links)
* [`notices` and `remarks`](#notices-and-remarks)
* [`events`](#events)
* [`publicIds`](#publicids)
* [`lang`](#lang)
* [`status`](#status)
* [`port43`](#port43)
* [`objectClassName`](#objectclassname)

## The RDAP Conformance Array {#rdapconformance}

This is a special array that must only appear in the top most JSON of an RDAP
response. It is used by a server to signal to a client the RDAP verion supported
by the server (there is only one) and any extensions.

```json
"rdapConformance" :
[
  "rdap_level_0"
]
```

In the original versions of RDAP ([RFC 7483](https://datatracker.ietf.org/doc/rfc7483/)),
this array was not mandatory. However, [RFC 9083](https://datatracker.ietf.org/doc/rfc9083/)
made it a requirement of all responses.

```json
"rdapConformance" :
[
  "rdap_level_0",     // rdap version 0
  "lunarNIC_level_0"  // an extension
]
```

When used in response to a `/help` query, this array must list all extensions
the server supports. However, when used for any other response it must only list
the extensions used in that response.

## The Links Array {#links}

The links array contains objects which represent a link and its metadata, as
defined by [RFC 8288](https://datatracker.ietf.org/doc/html/rfc8288).

```json
{
  "value" :    "https://example.com/context_uri", // REQUIRED - the context of the link
  "rel" :      "self",                            // REQUIRED - the relationship of the link
  "href" :     "https://example.com/target_uri",  // REQUIRED - the URL
  "hreflang" : [ "en", "ch" ],                    // a list of language tags
  "title" :    "title",                           // presentation name of the link
  "media" :    "screen",                          // display medium 
  "type" :     "application/json"                 // media type
}
```

In the original versions of RDAP ([RFC 7483](https://datatracker.ietf.org/doc/rfc7483/)),
the `value` property was optional. [RFC 9083](https://datatracker.ietf.org/doc/rfc9083/)
made it a requirement but never defined its meaning. There is no consensus what this string
must contain other than it is a URI and somehow relevant to either the object or response
in which it is found.

The `rel` property can only have values found in the [IANA Link Relations](https://www.iana.org/assignments/link-relations/link-relations.xhtml)
protocol parameter registry.

Though not required, the `type` property indicates to clients the type of data to be
found when the link, that is the value of `href`, is followed -- especially for HTTP/HTTPS
URLs. When the media type used is the RDAP JSON media type "application/rdap+json", this
indicates to RDAP clients that the link is a "referral". Note that the concept of a "referral"
is not formally defined in the RDAP specifications though in practice they are treated
this way, such as when a domain registry provides links to a domain registration in a domain
registrar.

## Notices and Remarks

The `notices` and `remarks` arrays are identical except for their names. The difference between
the two is that notices are related to the response as a whole while the remarks are related
to the object in which they are found. Therefore, notices are only found in the top-most
level of an RDAP response while remarks may be found in an RDAP object.


```json
"notices" :    // or "remarks"
[
  {
    "title" : "Enhance Your Calm",
    "type": "result set truncated due to excessive load", // from RDAP JSON Values
    "description" :  // REQUIRED
    [
      "Some data has been truncated because of too many queries.",
      "Calm down and come back later."
    ],
    "links" :  // this is the same structure defined above
    [
      {
        "value" : "https://example.net/entity/XXXX",
        "rel" : "about",
        "type" : "text/html",
        "href" : "https://www.example.com/take-it-easy.html"
      }
    ]
  }
]

```

The only required property of a notice or remark is `description`, which is an array
of strings. Each string is meant to be a separate paragraph.

If present, the value of the `type` property must come from the 
[IANA RDAP JSON Values Registry](https://www.iana.org/assignments/rdap-json-values/rdap-json-values.xhtml).
Clients can use this information to customize how the present the
information to a user.

For completeness, here is the following example from above but as a remark:

```json
"remarks" :
[
  {
    "title" : "Enhance Your Calm",

    // the type changes from "result set truncated due to excessive load" to
    // "object truncated due to excessive load" because a remark is an
    // object class level attribute whereas a notice is a response level
    // attribute.
    "type": "object truncated due to excessive load",

    "description" :  // REQUIRED
    [
      "Some data has been truncated because of too many queries.",
      "Calm down and come back later."
    ],
    "links" :
    [
      {
        "value" : "https://example.net/entity/XXXX",
        "rel" : "about",
        "type" : "text/html",
        "href" : "https://www.example.com/take-it-easy.html"
      }
    ]
  }
]

```

## Events

The `events` data structure is an array of event objects describing what happened at
what time and, optionally, by whom.

```json
"events" :
[
  {
    "eventAction" : "last update of RDAP database", // REQUIRED - from RDAP JSON Values
    "eventActor" : "IANA-ID-2332",
    "eventDate" : "1990-12-31T23:59:59Z",   // REQUIRED - RFC 3339 format
    "links" :  // this is the same structure defined above
    [
      {
        "value" : "https://example.net/domain/foo.example",
        "rel" : "about",
        "type" : "text/html",
        "href" : "https://www.example.com/update-frequency.html"
      }
    ]
  }
]
```
The `eventAction` property must come from the
[IANA RDAP JSON Values Registry](https://www.iana.org/assignments/rdap-json-values/rdap-json-values.xhtml).
And the `eventDate` property is a string containing an [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339)
date and time (see [this](json.md#date-and-time) for caveats and usage).

If the `eventActor` is an RDAP entity, then the `links` array may contain a referral
to the entity by using a `rel` of "related" and a `type` of "application/rdap+json".

## Public IDs {#publicids}

The objects in this array denote non-URI public identifiers.

```json
"publicIds":
[
  {
    "type": "IANA Registrar ID", // REQUIRED
    "identifier": "1"            // REQUIRED
  }
]
```

Both `type` and `identifier` are strings of free form text. In practice, they are
mostly used to identify gTLD registrars by their IANA registration number.

## Language Identifier {#lang}

This is a simple string name "lang", and it contains a language identifier.
The format for language identifiers is defined in [RFC 5646](https://datatracker.ietf.org/doc/html/rfc5646)
which can look like "en" (for English) or "de-AT" for German used in Austria or combinations
of language, script, region and variant subtags that are found in the
[IANA Language Subtag Registry](https://www.iana.org/assignments/lang-subtags-templates/lang-subtags-templates.xhtml).

The format can get very complicated but in practice is usually just a language subtag, such as "en", or
a language subtag followed by a region subtag, such as "en-US".

```json
"lang" : "en-UK"
```

According to [RFC 9083](https://datatracker.ietf.org/doc/html/rfc9083), this property may appear in any
JSON object in RDAP except the jCard structures. In practice, it is seldom used and only sensible inside
a [remark or notice](#notices-and-remarks).

## Status

The status array contains a list of strings, each with a value that must come from the
[IANA RDAP JSON Values Registry](https://www.iana.org/assignments/rdap-json-values/rdap-json-values.xhtml).

```json
"status" : [ "active", "locked" ]
```

## Port 43 Whois Server {#port43}

The "port43" string contains either the hostname or IP address of a [Whois](misc/glossary.md#whois) server
that might contain the same information. The reason this is not represented in a "links" object is that
a URI for Whois was never defined. In practice, this information is useless.

```json
"port43" : "whois.example.com"
```

## Object Class Name {#objectclassname}

Each RDAP object must have a string named "objectClassName". The string must have one of the following
values depending on the object class being represented:

* domain
* ip network
* autnum
* entity
* nameserver

This string is the descriminator used by clients when parsing the JSON object into an RDAP object class.


```json
"objectClassName" : "ip network"
```
