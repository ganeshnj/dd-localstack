# How to use

## Tracers

### Local server

```
cargo build
cargo run
```

### Examples

#### Go example

```
cd examples/go-example
go build
go run .
```

Check server logs and `/traces` directory.

## Distributed tracing

### Local server

```
cargo build
cargo run
```

Hit `localhost:8126/httpbin/get` to inspect sent headers

### Examples

#### Swift example

Debug app with Xcode

```
cd examples/swift-example/distributed-tracing
open Package.swift
```


Check the output on console.