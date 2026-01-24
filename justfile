default:
    just --list

[doc("Install mdbook and plugins using cargo")]
cargo_install:
    cargo install --version 0.4.52 mdbook \
    && cargo install --version 0.16.2 mdbook-mermaid \
    && cargo install --version 0.3.0 mdbook-variables \
    && cargo install --version 0.3.0 mdbook-svgbob2

[doc("Local build the book")]
local_build:
    CURRENT_BUILD_TIME=$(date) mdbook build    
