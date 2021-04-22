# wcal
[![Crates.io version shield](https://img.shields.io/crates/v/wcal.svg)](https://crates.io/crates/wcal)
[![Docs](https://docs.rs/wcal/badge.svg)](https://docs.rs/wcal)
[![Crates.io license shield](https://img.shields.io/crates/l/wcal.svg)](https://crates.io/crates/wcal)

A calculator write by rust

Allow operator: `+` `-` `*` `/` `(` `)`.

Result can be `i128` or `f64`. A warning will
occur while result is `i128` and division cast
happened, such as `3/2=1`.

This calculator has three steps:
* Use `logos` to parse the expression to tokens.
* Use a parser to parse tokens to a AST.
* Calculate the result from the AST.

The following parser is available:
* Top-down parser (default)

## Usage
### Command line mode
```shell
$ wcal "2*6+(1/2)" -f "2*6+(1/2)"
i> 2*6+(1/2)
Warning: division will cause a cast
12
f> 2*6+(1/2)
12.5
```
Default mod is `i128`, use `-f` to change to
`f64`, use `-i` to change back.

### Interactive mode
```shell
$ wcal
i> help
i       Enter i128 mod
f       Enter f64 mod
quit
q       Quit
i> f
Enter f64 mod
f> 1/2
0.5
f> quit
Bye!
```
