[![Crates.io](https://img.shields.io/crates/v/dltree.svg)](https://crates.io/crates/dltree)
[![Docs](https://docs.rs/dltree/badge.svg)](https://docs.rs/crate/dltree/)
[![Actions Status](https://github.com/VilNeo/dltree/workflows/Test/badge.svg)](https://github.com/VilNeo/dltree/actions)
[![grcov](https://img.shields.io/codecov/c/github/VilNeo/dltree)](https://app.codecov.io/gh/VilNeo/dltree)

# dltree

## About

***dltree* is a library that provides a doubly linked tree with leaf-considering typing.**

## Current state

The current implementation is well tested but far from being final.
Several future changes will be implemented until the first stable version 1.0:

 - [ ] Add documentation to all public structures and methods
 - [x] Remove all panicking elements and return Result<T,E> instead
   - Inserting and removing elements may panic. This will only happen if the tree integrity is violated.
     This is a library bug and therefore a panic is appropriate here.
 - [x] Add missing methods like insertion of children (in addition to just pushing)
 - [ ] Reevaluate the mutability of all methods
 - [ ] Add concise documentation how to use ***dltree*** in README.md

So please keep in mind that the interface and behaviour may change over time until version 1.0.

## Donation

***dltree*** is a purely private project and does not follow any commercial interests.
If you still want to support my work, you are welcome to do so with a small donation:

___
You can make donations via IOTA anonymously, instantly and without any fees:

<p style="text-align: center;">
<img src="resources/donation_address_iota.svg" width="200" height="200">

[`iota1qpmzj0hwmykcxwj9s80e5u7tpeuwpzh9ta8q5k5fj754wsxfq0afu8hrzax`](https://explorer.iota.org/mainnet/addr/iota1qpmzj0hwmykcxwj9s80e5u7tpeuwpzh9ta8q5k5fj754wsxfq0afu8hrzax)

</p>

___
