#![allow(unused)]

mod error;
mod shared;

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use ::pkarr::PkarrClientAsync;

pub use error::Error;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct PubkyClient {
    http: reqwest::Client,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) pkarr: PkarrClientAsync,
    /// A cookie jar for nodejs fetch.
    #[cfg(target_arch = "wasm32")]
    pub(crate) session_cookies: Arc<RwLock<HashSet<String>>>,
}
