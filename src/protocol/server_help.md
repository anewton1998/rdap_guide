# Server Help

The [`/help` query](rdap_urls.md#server-help) is a psuedo-lookup for server help. It contains
nothing more than a [`rdapConformance` array](common_data_structures.md#rdapconformance) and
a [`notices` array](common_data_structures.md#notices-and-remarks).

The following is an example from [RFC 9083](https://datatracker.ietf.org/doc/html/rfc9083#name-responding-to-help-queries):

```json
{
  "rdapConformance" :
  [
    "rdap_level_0"
  ],
  "notices" :
  [
    {
      "title" : "Authentication Policy",
      "description" :
      [
        "Access to sensitive data for users with proper credentials."
      ],
      "links" :
      [
        {
          "value" : "https://example.net/help",
          "rel" : "alternate",
          "type" : "text/html",
          "href" : "https://www.example.com/auth_policy.html"
        }
      ]
    }
  ]
}
```

