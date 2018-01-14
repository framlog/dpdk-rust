# dpdk-rust
A dpdk bindings for rust.

Tested with dpdk-stable-17.0.5, 17.0.8 on Ubuntu 16.04.

To use this crate properly, you should:
1. install `libnuma-devel` 
2. export `RTE_SDK` and `RTE_TARGET` properly.
3. compile DPDK with `-fPIC`.
