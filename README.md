# Proof of Work

## Motivation

[CryptoCTF] 2019 protected most of their online challenges against brute force
by requiring every connection to first submit a proof-of-work. The server
selects a random hash function, and asks for a printable string such that
the last three bytes of the hash match a random constant.

This is of course trivially defeated with the use of a rainbow table (and the
reason why I added a rainbow table implementation to [CCTK].)

[ASISCTF] 2019, organized by the same people, subsequently improved their PoW
scheme by requiring the printable string to be a randomly predetermined length.
One would thus need to build one rainbow table per possible length, which is
quite a bother. 

It is quite easy to write a brute force attack in python, but unfortunately,
it is not fast enough on older hardware; I consistently missed the timeout
a few seconds.

Here's a Rust program to compute the proof-of-work

## Usage

    cargo build --release
    cargo test --release
    ./target/relase/pow sha256 15 1a2b3c

[CRYPTOCTF]: https://cryp.toc.tf
[CCTK]: https://github.com/maugier/cctk
[ASISCTF]: https://asisctf.com
