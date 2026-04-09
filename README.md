![resonate banner](./assets/resonate-banner.png)

# Hello World — Resonate Rust SDK

A minimal example showing how to define a durable workflow and a leaf function using the Resonate Rust SDK, register them, and execute them with crash recovery.

## What this demonstrates

- Defining a workflow with `#[resonate::function]` and `&Context`
- Defining a leaf function with `#[resonate::function]`
- Registering functions with `resonate.register()`
- Invoking a durable execution via the Resonate CLI

## Prerequisites

- [Rust toolchain](https://rustup.rs/) (1.75+)
- Resonate Server & CLI: `brew install resonatehq/tap/resonate`

## Setup

```bash
git clone https://github.com/resonatehq-examples/example-hello-world-rs.git
cd example-hello-world-rs
```

## Run it

### 1. Start the Resonate Server

```bash
resonate dev
```

### 2. Start the worker

In a second terminal:

```bash
cargo run
```

You should see:

```
Worker started. Waiting for invocations...
```

### 3. Invoke the function

In a third terminal:

```bash
resonate invoke greet-1 --func greet --arg '"World"'
```

### 4. Observe the result

The worker terminal will show the greeting being processed.
Check the execution tree:

```bash
resonate tree greet-1
```

## What to observe

- The worker processes the greeting workflow, which calls the `format_greeting` leaf function
- Try killing the worker mid-execution and restarting — the execution picks up where it left off
- Use `resonate tree greet-1` to visualize the durable call graph

## The code

```rust
use resonate::prelude::*;

#[resonate::function]
async fn greet(ctx: &Context, name: String) -> Result<String> {
    let greeting = ctx.run(format_greeting, name).await?;
    Ok(greeting)
}

#[resonate::function]
async fn format_greeting(name: String) -> Result<String> {
    Ok(format!("Hello, {name}! Welcome to durable execution."))
}
```

Two functions. One workflow. Fully durable.

## File structure

```
example-hello-world-rs/
├── Cargo.toml          # Dependencies
├── src/
│   └── main.rs         # Workflow + leaf function + worker setup
├── LICENSE             # Apache-2.0
└── README.md           # This file
```

## License

Apache-2.0
