use resonate::prelude::*;

/// A workflow that orchestrates a greeting.
/// Workflows receive `&Context` and can call sub-tasks.
#[resonate::function]
async fn greet(ctx: &Context, name: String) -> Result<String> {
    let greeting = ctx.run(format_greeting, name).await?;
    Ok(greeting)
}

/// A leaf function — pure computation, no Context needed.
#[resonate::function]
async fn format_greeting(name: String) -> Result<String> {
    Ok(format!("Hello, {name}! Welcome to durable execution."))
}

#[tokio::main]
async fn main() {
    let resonate = Resonate::new(ResonateConfig {
        url: Some("http://localhost:8001".into()),
        ..Default::default()
    });

    resonate.register(greet).unwrap();
    resonate.register(format_greeting).unwrap();

    // Keep the process alive to receive work from the server.
    println!("Worker started. Waiting for invocations...");
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c");
}
