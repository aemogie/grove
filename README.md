# grove

a magical content-addressable filesystem library

## planned features
- on-disk (also async maybe?)
- in-memory (how do i do allocations, nicely?)
- bring your own hashing algorithm ([supported algorithms](https://github.com/RustCrypto/hashes#supported-algorithms))
- some form of git compatiblity? could use dumb https, but github doesnt support that. options are,
  - figure out and use smart https
  - use libgit2
    - see if i can fetch individual objects
    - clone the entire tree to a temporary directory and repack it back into the store

## why?
[acid-store](https://github.com/lostatc/acid-store) has a big ol \[UNMAINTAINED\] banner.

it may still work but i dont like scary words.
