default:
    just --list

[doc("Install mdbook and plugins using cargo")]
cargo_install:
    cargo install mdbook \
    && cargo install mdbook-mermaid \
    && cargo install mdbook-variables \
    && cargo install mdbook-svgbob2

[doc("Local build the book")]
local_build:
    CURRENT_BUILD_TIME=$(date) mdbook build    
