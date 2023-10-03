## What it is about

This a test/experiment to working with async rust using os threads. 
A simple TCP client and server is created, the server spawns a new thread for
each request and processes it. The client send the messages, and it is the one
that sends the termination signal to the server.


## Playing with it

Firstly, clone the repo. Then navigate to the cloned repo and run 
```cargo run --bin server```
to get the server started.

Then to connect a new client run ```cargo run --bin client```.

To exit type `:quit` on the client side terminal.

## Miscellaneous

A promonent bug is on the server side, where when closed forcefull by terminating
the server the client doesn't terminate and crashes after a while.

Ignore the `tokio` dependency.
