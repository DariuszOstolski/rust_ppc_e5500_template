# [Rust PowerPC E5500 project template](https://github.com/DariuszOstolski/rust_ppc_e5500_template)

This is a template [Rust](https://www.rust-lang.org/) project to be able to 
cross compile for [PowerPC E5500](https://en.wikipedia.org/wiki/PowerPC_e5500) 
processors.

The problem with powerpc64-unknown-linux-gnu target in Rust is that by default 
it requires [AltiVec](https://github.com/rust-lang/rust/issues/59040), which is
not available on PowerPC E5500. What is more problematic std crate comes with 
AltiVec enabled so std create as well as other creates have to be recompiled.

## Getting Started

Dependencies:

* [rustup](https://rustup.rs/)
* [Xargo](https://github.com/japaric/xargo)
* PowerPC gcc toolchain

### PowerPC GCC toolchain installation

Ubuntu 18.04 instructions to install powerpc cross compiler

```console
sudo apt-get install gcc-7-powerpc64-linux-gnu
sudo update-alternatives --install /usr/bin/powerpc-linux-gnu-gcc powerpc-linux-gnu-gcc /usr/bin/powerpc64-linux-gnu-gcc-7 10
```

### Installing dependencies

You will have to switch to nightly because std/core depends on nightly.

```console
rustup default nightly
rustup component add rust-src
rustup target add powerpc64-unknown-linux-gnu
cargo install xargo
```

### Building project

```console
$ xargo build --target powerpc64-unknown-linux-gnu
```

## Caveats / gotchas

* See .cargo/config for linker configuration
* If you have this error:
```console
error[E0635]: unknown feature `llvm_asm`
 --> /home/rusty/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.29/src/lib.rs:3:12
```
There is a bug in cargo-xbuild (see: https://stackoverflow.com/a/61784361/4175326), execute following command:

```console
cargo install cargo-xbuild --git https://github.com/rust-osdev/cargo-xbuild.git --branch master --debug --force
```


## License

Licensed under:

* MIT license ([LICENSE-MIT](https://opensource.org/licenses/MIT))

### Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit
for inclusion in the work, as defined in the MIT license, shall be
licensed as above, without any additional terms or conditions.
