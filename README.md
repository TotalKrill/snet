#snet

snet is a small tool that connects to a serialport and then forwards everything it collects from there to tcp sockets

Its unsecure and its only one-directional for the moment, there
are currently no plans to fix that

#usage

- build it
- run it (--help) if you do not know how to
- connect to it using telnet on the specified port

#why

We had a pool of raspberry pi that was running gdb servers and
had a lot of printf-debugging going on, this meant we had ssh
access and tmux sessions etc on the raspberries for everyone who
wanted to try something. It got tedious.
