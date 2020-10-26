CLI profiling tool

To build, just type cargo build and run the following command:
./target/debug/cf-systems/cli
run with --help command for usage instructions

One of the main libraries I used is rustls, a library to set up TLS connections with servers. Since this was my first time using it, I took inspiration from some example code on how to configure and make tls connections. The repo can be found here: https://github.com/ctz/rustls
