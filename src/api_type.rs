use std::{
    ops::{Deref, DerefMut},
    sync::Arc, time::Duration,
};

use reqwest::Proxy;
use std::sync::RwLock;

pub struct Api {
    pub(super) nonce: String,
    pub(super) token: Option<String>,
    pub(super) proxy: Option<Proxy>,
    pub(super) timeout: Option<Duration>,
    pub(super) client: Arc<RwLock<reqwest::Client>>,
    pub(super) email: Option<String>,
    pub(super) password: Option<String>,
}

pub struct Parmas {
    pub(super) _host: &'static str,
    pub(super) _uri: &'static str,
    pub(super) _parmas: Vec<(String, String)>,
}

impl Deref for Parmas {
    type Target = Vec<(String, String)>;
    fn deref(&self) -> &Self::Target {
        &self._parmas
    }
}

impl DerefMut for Parmas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._parmas
    }
}
