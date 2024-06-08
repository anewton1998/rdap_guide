```svgbob
+----------------------------------------+
|                                        |
|          .--. .-+  .--. .--.           |
|          |__| |  \ |__| |__|           |
|          | \  |__/ |  | |              |
|                                        |
| The Registration Data Access Protocol  |
|                                        |
+----------------------------------------+

```

# Introduction

The Registration Data Access Protocol (RDAP) is the successor protocol to the Whois protocol. 
It was first ratified by the Internet Engineering Task Force (IETF) in March 2015 by
their [WEIRDS](/misc/glossary.md#weirds) working group, and initial server and client 
implementations were released shortly thereafter by the many Regional Internet
Registries (RIRs) in June 2015.

In the years since RDAP became standardized, extensions have been added and profiles 
have been specified. While this is a clear sign of the success of the protocol,
the amount of information spread across RFCs, IANA registries and other documents makes 
specification information more difficult to acquire and implementations
harder to develop and deploy.

This book is intended to describe RDAP a in way the RFCs do not, and in many cases cannot
describe the protocol and its ecosystem through the use of [mdbook](https://rust-lang.github.io/mdBook/index.html),
the many mdbook plugins, annotated examples, easier to read language and references to other materials. 

It is also not intended as a replacement for the RFCs and other specifications governing
RDAP. However, it is intended to offer clearer text, more complete context, and more
examples than one might find in the other sources. Additionally, this book will cover
background, the ecosystem, and popular conventions that are not well documented in other
places. This should aid developers and others
involved in the efforts of implementing RDAP clients and servers.

## Version

This book is a living document and is updated on an as-needed basis.

This version built on {{CURRENT_BUILD_TIME}}.

## Contributing

Contributions to this book are welcome. See [Contributing](misc/contributing.md).

## Copyright

Copyright (C) 2024 Andrew Newton

## License

The source for this book is released under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0).
