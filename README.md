# snet

snet is a small tool that connects to a serialport and then forwards everything it collects from there to tcp sockets, the main difference to ser2net is that this allows multiple accesses in both directions, and
that this does a lot less and is less mature :)

Its unsecure so only run on trusted networks on non-critical things, its just a raw connection
to the serialport, meaning every byte is forwarded in both directions. Telnet works as a client for
simple things such as watching the output

I might write a client that emulates a serialport on the client side...

[![Latest version](https://img.shields.io/crates/v/snet.svg)](https://crates.io/crates/snet)
![License](https://img.shields.io/crates/l/snet.svg)


# Install and run

First install it using cargo install, or go clone the repo and use `cargo build`

Cargo install:

    cargo install snet
    ~/.cargo/bin/snet -s /dev/ttyUSB0 -p 1234

Git:

    git clone https://github.com/TotalKrill/snet.git
    cargo run --release -- -s /dev/ttyUSB0 -p 1234


# Connect

There should now be a tcp port that you can connect to:

    telnet 127.0.0.1 1234

# Why

We had a pool of raspberry pi that was running gdb servers and
had a lot of printf-debugging going on, this meant we had ssh
access and tmux sessions etc on the raspberries for everyone who
wanted to try something. It got tedious.
