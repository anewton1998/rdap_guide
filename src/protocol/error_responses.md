# Error Responses

When a query results in an error, an RDAP server is not required to send an RDAP error response but doing
so is considered good practice.

Responses to errors in RDAP mirror the HTTP status code. Like all other RDAP responses, these are to
have the `content-type: application/rdap+json` header and the 
[`rdapConformance` array](common_data_structures.md#rdapconformance). The following is a very simple example:

```json
{
  "rdapConformance" : [ "rdap_level_0" ],
  "errorCode": 420,
  "title": "Enhance Your Calm",
  "description":
  [
    "Chill out!",
    "Srsly, dude."
  ]
}
```

The response can have the following JSON data structures, including the common response structures of 
[`rdapConformance`](common_data_structures.md#rdapconformance) and [`notices`](common_data_structures.md#notices-and-remarks) array.

| Name              | Value                                                                                      |
| ----------------- | ------------------------------------------------------------------------------------------ |
| `rdapConformance` | (**REQUIRED**) a common type defined [here](common_data_structures.md#rdapconformance)     |
| `notices`         | a common type defined [here](common_data_structures.md#notices-and-remarks)                |
| `errorCode`       | (**REQUIRED**) an integer matching the HTTP status code                                    |
| `title`           | a string specifying the title of the error                                                 |
| `description`     | an array of strings describing the error                                                   |
| `lang`            | a [`lang`](common_data_structures.md#lang) sting                                           |


Though rarely used, the error response may also have a [`notices`](common_data_structures.md#notices-and-remarks) array.
The following is a complete example from [RFC 9083](https://datatracker.ietf.org/doc/html/rfc9083#name-error-response-body):

```json
{
  "rdapConformance" :
  [
    "rdap_level_0"
  ],
  "notices" :
  [
    {
      "title" : "Beverage Policy",
      "description" :
      [
        "Beverages with caffeine for keeping horses awake."
      ],
      "links" :
      [
        {
          "value" : "https://example.net/ip/192.0.2.0/24",
          "rel" : "alternate",
          "type" : "text/html",
          "href" : "https://www.example.com/redaction_policy.html"
        }
      ]
    }
  ],
  "lang" : "en",
  "errorCode": 418,
  "title": "Your beverage choice is not available",
  "description":
  [
    "I know coffee has more ummppphhh.",
    "Sorry, dude!"
  ]
}
```
