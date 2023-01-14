//!
//! VoiceVox 0.13.3 api implementation.
//!
#[cfg(all(not(feature = "backend_surf"), not(feature = "backend_reqwest")))]
compile_error!("you need to enable backend_surf or backend_reqwest");
#[cfg(not(feature = "backend_surf"))]
pub mod reqwest_api;
#[cfg(feature = "backend_surf")]
pub mod surf_api;

#[cfg(all(not(feature = "backend_surf"), feature = "backend_reqwest"))]
pub use reqwest_api as api;
#[cfg(all(not(feature = "backend_reqwest"), feature = "backend_surf"))]
pub use surf_api as api;

pub mod api_schema;
