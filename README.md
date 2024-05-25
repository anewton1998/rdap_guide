This is the repository for *An Implementers Guide to the Registration Data Access Protocol (RDAP)* book.

The book may be viewed [here](https://anewton1998.github.io/rdap_guide/). 

## Building Locally

To build locally, you'll need to have the Rust toolchain installed. If you do not have Rust installed,
go [here](https://www.rust-lang.org/tools/install).

Then install mdbook and the necessary mdbook preprocessors:

~~~
cargo install mdbook
cargo install mdbook-mermaind
cargo install mdbook-variables
cargo install mdbook-svgbob2
~~~

Build the book with `CURRENT_BUILD_DATE=$(date) mdbook build`.

More information on mdbook can be found [here](https://rust-lang.github.io/mdBook/index.html).
