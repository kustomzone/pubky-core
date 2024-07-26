#![allow(unused)]

macro_rules! if_not_wasm {
    ($($item:item)*) => {$(
        #[cfg(not(target_arch = "wasm32"))]
        $item
    )*}
}

macro_rules! if_wasm {
    ($($item:item)*) => {$(
        #[cfg(target_arch = "wasm32")]
        $item
    )*}
}

if_not_wasm! {
    mod client;
    mod client_async;

    use pkarr::{PkarrClient};
    use ureq::{Agent, Response};
    use url::Url;
}

if_wasm! {
    mod wasm;

    pub use wasm::keys::Keypair;
    pub use wasm::PubkyClient;
}

mod error;
pub use error::Error;
