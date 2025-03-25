1. [yrm](#yrm)
2. [Tools Mentioned](#tools-mentioned)
   1. [JustFile](#justfile)
   2. [Cargo ASM](#cargo-asm)
3. [Note Section](#note-section)

# yrm

> Learning about rust memory management from yt vid

# Tools Mentioned 

## JustFile 

> This is an alternative to nmake file 

```sh
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to DEST
``` 
- Above is installation taken from [HERE](https://just.systems/man/en/pre-built-binaries.html)
- Firs run the above install then add to path 
- `fish_add_path DEST` - Note dest is where it gets installed 
- Now the program will work `justfile`

## Cargo ASM 

1. This is for getting the assembly of the program 
2. The tutorial was showin an older crate , we will use a newer crate 

[`cargo-show-asm`](https://lib.rs/crates/cargo-show-asm) - Shows the generated assembly code 

```rs 
cargo install cargo-show-asm
```
- Note this is more modern crate which addresses short comings of `cargo asm`

# Note Section 

> This will have important notes from the tutorial video 



1. Arrays live directly in the stack in rust 
2. `vec![]` - Vector is array of dynamic size , doent live on the stack , but lives on the heap