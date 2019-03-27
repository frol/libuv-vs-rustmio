# About

[Libuv](https://github.com/libuv/libuv) is a super popular cross platform
_Networking Event Loop Engine written in C_ , it's a main Networking engine for
Node.js.

[MIO](https://github.com/carllerche/mio) is a cross platform _Networking Event
Loop Engine written in Rust_ with 0 cost of abstraction principle.

[Romio](https://github.com/withoutboats/romio) combines the powerful futures
abstractions with the nonblocking IO primitives of mio to provide efficient and
ergonomic asynchronous IO primitives for the Rust asynchronous networking
ecosystem.

[Tokio](https://github.com/tokio-rs/tokio) is runtime for writing reliable,
asynchronous, and slim applications with the Rust programming language.

This is a fork of
[tigranbs/libuv-vs-rustmio](https://github.com/tigranbs/libuv-vs-rustmio), which
only wanted to [compare libuv (C) and mio (Rust) raw
performance](https://github.com/tigranbs/libuv-vs-rustmio/issues/4). I bumped
into an issue that my Romio implementation hit a wall sooner than I expected
(~15% less requests per second than mio), so I decided to share my experiments,
so I may get some help from the community
([let's discuss in #1](https://github.com/frol/libuv-vs-rustmio/issues/1)).

# Usage

For making some benchmarking we need 3rd tool for just sending and reading
network traffic, and I chose to use Redis Benchmark utility with PING benchmark.

#### Running MIO, Romio, Tokio, Std, and Std-Threadpool Tcp Echo servers - you will need Rust and Cargo installed

```bash
# this will start TCP echo server on port 8888
cd tcp_mio/ && cargo run --release
```

#### Running Libuv Tcp Echo server

```bash
# building C code
cd tcp_uv && ./build.sh

# Starting Libuv TCP echo server on port 8888
./tcp_uv/build/tcp_uv 8888
```

#### Starting Benchmarking tool

```bash
$ redis-benchmark -t ping_inline -n 1000000 -P 1 -p 8888
```

# Results

I'll add some results for multiple OS's and instances

# Contribution

If you have ideas about what kind of applications we can write in order to make
a real world benchmarking just open an issue for discussion or send ma Pull
Request.
