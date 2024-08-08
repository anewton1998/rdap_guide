# Referrals

Referrals are not a formal mechanism in RDAP but are often used to link
a resource in one RDAP server to another. They are in wide-spread use within
the [gTLD](../glossary.md#gtld) services to allow a gTLD registry to
point a client at a domain name in a gTLD registrar.

Referrals work using the [`links`](../protocol/common_data_structures.html#links)
data structure when the `type` attribute of the link is set to the RDAP media
type "application/rdap+json".

```json
{
  "value" : "https://registry.example/domain/foo.example",
  "rel"   : "related",
  "type"  : "application/rdap+json",
  "href"  : "https://registrar.example/domain/foo.example"
}
```

The [ICANN Response Profile](../specifications/icann.md) specifies that these referrals be made using
the `related` relationship type.

The soon-to-be [RIR Search](https://datatracker.ietf.org/doc/html/draft-ietf-regext-rdap-rir-search-09#name-link-relations)
will define referrals for "up", "down", "top", "bottom".
