use etcetera::AppStrategyArgs;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static APP_STRATEGY: Lazy<AppStrategyArgs> = Lazy::new(|| AppStrategyArgs {
    top_level_domain: "Block".to_string(),
    author: "Block".to_string(),
    app_name: "goose".to_string(),
});

#[cfg(feature = "autovisualiser")]
pub mod autovisualiser;
#[cfg(feature = "computercontroller")]
pub mod computercontroller;
pub mod mcp_server_runner;
#[cfg(feature = "memory")]
mod memory;
#[cfg(target_os = "macos")]
pub mod peekaboo;
pub mod subprocess;
#[cfg(feature = "tutorial")]
pub mod tutorial;

#[cfg(feature = "autovisualiser")]
pub use autovisualiser::AutoVisualiserRouter;
#[cfg(feature = "computercontroller")]
pub use computercontroller::ComputerControllerServer;
#[cfg(feature = "memory")]
pub use memory::MemoryServer;
#[cfg(feature = "tutorial")]
pub use tutorial::TutorialServer;

/// Type definition for a function that spawns and serves a builtin extension server
pub type SpawnServerFn = fn(tokio::io::DuplexStream, tokio::io::DuplexStream);

#[cfg(any(
    feature = "autovisualiser",
    feature = "computercontroller",
    feature = "memory",
    feature = "tutorial"
))]
fn spawn_and_serve<S>(
    name: &'static str,
    server: S,
    transport: (tokio::io::DuplexStream, tokio::io::DuplexStream),
) where
    S: rmcp::ServerHandler + Send + 'static,
{
    use rmcp::ServiceExt;
    tokio::spawn(async move {
        match server.serve(transport).await {
            Ok(running) => {
                let _ = running.waiting().await;
            }
            Err(e) => tracing::error!(builtin = name, error = %e, "server error"),
        }
    });
}

macro_rules! builtin {
    ($name:ident, $server_ty:ty) => {{
        fn spawn(r: tokio::io::DuplexStream, w: tokio::io::DuplexStream) {
            spawn_and_serve(stringify!($name), <$server_ty>::new(), (r, w));
        }
        (stringify!($name), spawn as SpawnServerFn)
    }};
}

pub static BUILTIN_EXTENSIONS: Lazy<HashMap<&'static str, SpawnServerFn>> = Lazy::new(|| {
    let mut map = HashMap::new();
    #[cfg(feature = "autovisualiser")]
    {
        let entry = builtin!(autovisualiser, AutoVisualiserRouter);
        map.insert(entry.0, entry.1);
    }
    #[cfg(feature = "computercontroller")]
    {
        let entry = builtin!(computercontroller, ComputerControllerServer);
        map.insert(entry.0, entry.1);
    }
    #[cfg(feature = "memory")]
    {
        let entry = builtin!(memory, MemoryServer);
        map.insert(entry.0, entry.1);
    }
    #[cfg(feature = "tutorial")]
    {
        let entry = builtin!(tutorial, TutorialServer);
        map.insert(entry.0, entry.1);
    }
    map
});
