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

```
cd examples/swift-example/distributed-tracing
swift build
swift run
```

Check the output on console.