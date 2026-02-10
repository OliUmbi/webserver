# Features

## Overview

| Topic                     | Details     | Notes                                                   |
|---------------------------|-------------|---------------------------------------------------------|
| Supported version         | HTTP/1.1    | Other versions get rejected                             |
| Host header               | Required    | 400 if missing                                          |
| Chunked encoding          | Unfinished  | Structure implemented, logic missing                    |
| Persisten connections     | Unsupported | Default connection close                                |
| Pipelining                | Unsupported | Sequential only                                         |
| Duplicate Headers         | Supported   | Validated for Host and Content-Length                   |
| Methods                   | Supported   |                                                         |
| Unknown method            | Unsupported | Rejects with 405                                        |
| Message framing           | Supported   |                                                         |
| Message validation        | Supported   | Message get parsed and rejected if structure is invalid |
| Static file serving       | Supported   | Defined via configuration                               |
| Redirects                 | Supported   | Defined via configuration                               |
| Reverse Proxy             | Supported   | Defined via configuration, implentation is blocking     |
| Directory listing         | Unsupported |                                                         |
| Timeout                   | Supported   |                                                         |
| Large body streaming      | Partially   | Body gets read into buffer if needed                    |
| Request size limiting     | Supported   | Defined via configuration                               |
| Routing                   | Supported   | Prefic, Exact, Regex                                    |
| Host based routing        | Unsupported |                                                         |
| Multithreading            | Supported   |                                                         |
| Backpressure              | Supported   |                                                         |
| Path Traversal Protection | Supported   |                                                         |
| Caching                   | Unsupported |                                                         |
| SSL                       | Unsupported |                                                         |

## Limitations

The thread model is blocking and waits until message head is received or timeout.
Body get loaded into memory for reverse proxy but will only be read if needed.
Connections do not get reused after the first request and will be closed afterwards.

