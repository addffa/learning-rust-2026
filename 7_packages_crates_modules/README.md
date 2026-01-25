# Crates

The smalles amount of code that Rust compiler considers at a time.
Crates can contain modules.
Has two form: binary crates and library crates.

# Packages

Bundle of one or more crates that provides a set functionality.
Contains Cargo.toml that describes how to build it's crates.
Must contain at least one crate (binary or library), and can contains one or more binary crates, but only can contain one library crate.

# Modules

module tree in this example crate:

crate
|-- main.rs
|-- garden
    |-- vegetables
        |-- Asparagus

