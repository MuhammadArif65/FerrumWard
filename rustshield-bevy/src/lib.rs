#![allow(warnings)]
use bevy::prelude::*;
use rustshield_core::protection::{protect, ProtectionConfig};
use rustshield_core::rustshield_checkpoint;
use std::sync::{Arc, Mutex};

/// A Bevy Plugin that integrates RustShield anti-piracy protections.
pub struct RustShieldPlugin {
    config: Arc<Mutex<Option<ProtectionConfig>>>,
}

impl RustShieldPlugin {
    /// Creates a new RustShieldPlugin with the given configuration.
    pub fn new(config: ProtectionConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(Some(config))),
        }
    }
}

impl Plugin for RustShieldPlugin {
    fn build(&self, app: &mut App) {
        if let Ok(mut lock) = self.config.lock() {
            if let Some(config) = lock.take() {
                // Initialize the core protection engine.
                // It will spawn a background thread automatically.
                match protect(config) {
                    Ok(_) => {
                        // Register a system that runs every frame or conditionally to trigger checkpoints
                        app.add_systems(Update, rustshield_bevy_checkpoint_system);
                    }
                    Err(_e) => {
                        // If the initial protection fails (e.g. invalid license, tampered files),
                        // the `on_failure` callback inside the config has already been triggered by `protect`.
                        // We do nothing else here to avoid leaking information.
                    }
                }
            }
        }
    }
}

/// A Bevy system that periodically triggers random internal checks.
fn rustshield_bevy_checkpoint_system() {
    // rustshield_checkpoint! is designed to be extremely fast and mostly no-op
    // unless a random trigger condition is met, so it is safe to call per-frame.
    let _ = rustshield_checkpoint!();
}

//
