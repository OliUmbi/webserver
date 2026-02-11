# Knowledge

## Goals

| Area                | Goal                                                  |
|---------------------|-------------------------------------------------------|
| HTTP/1.1            | Understand full request structure and message framing |
| Connection Handling | Understand persistent connections and lifecycle       |
| Concurrency         | Implement multithreaded request processing            |
| Rust                | Gain practical experience building a network service  |

## HTTP/1.1

Learned:

- Exact structure of request line, headers and body
- Difference between Content-Length and Transfer-Encoding
- Persistent connection behaviour
- Request framing edge cases

Reflection:
The specification is precise but complex.
Practical implementation reveals ambiguities that are not obvious when reading the RFC alone.

## Connection Handling

Learned:

- TCP stream lifecycle
- Incremental parsing
- Handling partial reads
- Keep-alive vs connection close semantics

Reflection:
Correct framing and lifecycle management is more difficult than request parsing itself.

## Multithreading

Learned:

- Thread pool design
- Work distribution strategies
- Backpressure using bounded channels
- Trade-offs between synchronous threads and async runtimes

Reflection:
Concurrency design strongly impacts performance and complexity.
A asynchronous approach would have been better for the project but too complex to implement in the short time.

## Rust

Learned:

- The language is a lot of fun
- Ownership and borrowing in network programming
- Lifetime implications in parsers
- Error handling patterns
- Building modular systems without heavy frameworks

Reflection:
Rust enforces correctness early but increases initial complexity.
For low-level protocol work it provides strong safety guarantees.
Confidence in the code / implementation is much higher than other languages
