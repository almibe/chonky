# chonky
A thicc message bus for Rust.

## Status
This project is very young and unstable but is being actively developed and used in anger by other projects as a testing ground.
The api and decisions around what style of message passing is best vs just supporting multiple styles is yet to be decided.
I'm interested in feed back if you are using this project and having an questions or run into any issues.

## Goals
Chonky is a library that tries to make inter-module communication easier in Rust via message passing.
TBH, most Rust projects won't need to use a project like Chonky, but it can be helpful in certain cases.
Chonky focuses on simplicity and flexibility so it deviates from some Rust-isms to achieve that goal since writing type-safe and dynamic code in Rust can be very difficult.

## Messaging style
Chonky uses a 1:M request-response style messaging.
A sender passes one message to an address and then gets back 0 to infinity responses or an error.
Only a single addressee can have a given address and if a message is passed to an address that doesn't exist a DeadLetter error is returned.
Other messaging styles might be considered later, but this is the main focus of Chonky.

## Basics
Chonky's api is very simple you can only really do two things with it.
 * Register an addressee
 * Post a message to an address

### Addressees
Addressees in Chonky are made up of two different parts.
First is the address with is simply a String.
Users are left to come up with their own naming schemes but I suggest something like `module-name:event-name`.
Chonky tries to fail fast when there could be a potential problem so Chonky panics if more than one addressee has the same address.
This is the only time Chonky panics and most of the time addressee registration happens at startup.
The second part is a function pointer that is called when a message is passed to that address.

### Messages
A message is simply a Vec<u8> payload.
In the same way that users need to come up with their own agreed upon naming scheme, users need to come up with an agreed upon serialization.
My reccomendation would be to use [bincode](https://github.com/servo/bincode) on an agreed upon data strucutre.
