// Mods
pub mod template;

// Helpers
pub use fehler::throws;
pub use template::Render;
pub type Error = anyhow::Error;
