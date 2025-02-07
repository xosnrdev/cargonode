use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum PackageEvent {
    Created {
        name: String,
        path: Arc<Path>,
    },
    WorkspaceCreated {
        root: Arc<Path>,
        patterns: Vec<String>,
    },
    GitInitialized {
        path: Arc<Path>,
    },
    Error {
        message: String,
    },
}

pub trait PackageEventHandler: Send + Sync {
    fn handle_event(&self, event: &PackageEvent);
}

#[derive(Default)]
pub struct EventDispatcher {
    handlers: Vec<Box<dyn PackageEventHandler>>,
}

impl EventDispatcher {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_handler(&mut self, handler: Box<dyn PackageEventHandler>) {
        self.handlers.push(handler);
    }

    pub fn dispatch(&self, event: &PackageEvent) {
        for handler in &self.handlers {
            handler.handle_event(event);
        }
    }
}

// Default console event handler
pub struct ConsoleEventHandler;

impl PackageEventHandler for ConsoleEventHandler {
    fn handle_event(&self, event: &PackageEvent) {
        match event {
            PackageEvent::Created { name, path } => {
                println!("Created package '{name}' at {}", path.display());
            }
            PackageEvent::WorkspaceCreated { root, patterns } => {
                println!(
                    "Created workspace at {} with patterns: {patterns:?}",
                    root.display()
                );
            }
            PackageEvent::GitInitialized { path } => {
                println!("Initialized Git repository at {}", path.display());
            }
            PackageEvent::Error { message } => {
                eprintln!("Error: {message}");
            }
        }
    }
}
