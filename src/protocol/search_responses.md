# Search Responses

Responses to RDAP searches use a named array that matches the object class found
in the array.

| Array Name              | Object Class |
| ----------------------- | ------------ |
| domainSearchResults     | domain       |
| nameserverSearchResults | nameserver   |
| entitySearchResults     | entity       |

Note that each search result is **required** to have a [`rdapConformance` array](common_data_structures.md#rdapconformance)
and may have a [`notices` array](common_data_structures.md#notices-and-remarks).

## Domain Search Results

Domain search results are the proper response to a [domain search query](rdap_urls.md#domain-searches):

```json
{
  "rdapConformance": [ "rdap_level_0" ],
  "domainSearchResults" : [  
    {
      "objectClassName" : "domain",
      "handle" : "result_1",
      "ldhName" : "1.com"
      // ...
    },
    { 
    {
      "objectClassName" : "domain",
      "handle" : "result_2",
      "ldhName" : "2.com"
      // ...
    },
    }
  ]
}
```

## Nameserver Search Results

Nameserver search results are the proper response to a [nameserver search query](rdap_urls.md#nameserver-searches):

```json
{
  "rdapConformance": [ "rdap_level_0" ],
  "nameserverSearchResults" : [  
    {
      "objectClassName" : "nameserver",
      "handle" : "result_1",
      "ldhName" : "ns1.foo.com"
      // ...
    },
    { 
    {
      "objectClassName" : "nameserver",
      "handle" : "result_2",
      "ldhName" : "ns2.foo.com"
      // ...
    },
    }
  ]
}
```

## Entity Search Results

Entity search results are the proper response to an [entity search query](rdap_urls.md#entity-searches):

```json
{
  "rdapConformance": [ "rdap_level_0" ],
  "entitySearchResults" : [  
    {
      "objectClassName" : "entity",
      "handle" : "entity_1",
      // ...
    },
    { 
    {
      "objectClassName" : "entity",
      "handle" : "entity_2",
      // ...
    },
    }
  ]
}
```
