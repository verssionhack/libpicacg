#![allow(unused)]
use std::{
    fmt::Debug,
    str::FromStr,
    time, path::PathBuf,
};

use chrono::NaiveDateTime;
use hex::ToHex;
use hmac::{digest::InvalidLength, Hmac, Mac};
use rand::Rng;
use reqwest::header::HeaderMap;

use serde::{Deserialize, Deserializer, Serializer};
use serde_json::Value;
use sha2::Sha256;

use crate::{
    api, header_name,
    r#trait::{Adapt, Pagible},
    r#type::app,
    responses::{
        Docs, GameDownloadInfo, PictureDownloadResounce, GameDownloadInfoP2p, GameDownloadInfoDrive, GameDownloadInfoS3,
    },
    Header, Quality, Response, Sort,
};

pub fn num_correct_deserializer<'de, D>(de: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Value::deserialize(de)?;
    Ok(
        if v.is_string() {
            v.as_str().unwrap().parse().unwrap()
        } else {
            v.as_u64().unwrap()
        }
        )
}

pub fn game_download_info_deserializer<'de, D>(de: D) -> Result<GameDownloadInfo, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Value::deserialize(de)?;
    Ok(GameDownloadInfo {
        node: v
            .as_object()
            .unwrap()
            .get("node")
            .unwrap()
            .as_object()
            .unwrap()
            .values().map(|v| v.as_str().unwrap().to_string()).collect(),
        p2p: GameDownloadInfoP2p::deserialize(v.as_object().unwrap().get("p2p").unwrap()).unwrap(),
        drive: GameDownloadInfoDrive::deserialize(v.as_object().unwrap().get("drive").unwrap()).unwrap(),
        s3: GameDownloadInfoS3::deserialize(v.as_object().unwrap().get("s3").unwrap()).unwrap(),
    })
}

pub fn datetime_deserializer<'de, D>(de: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    Ok(NaiveDateTime::parse_from_str(s.as_str(), "%Y-%m-%dT%H:%M:%S%z").unwrap())
}

pub fn datetime_serializer<S>(datetime: &NaiveDateTime, se: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = datetime.format("%Y-%m-%dT%H:%M:%S%z").to_string();
    se.collect_str(s.as_str())
}

macro_rules! impl_pagible {
    ($target: ty) => {
        impl Pagible for $target {
            fn total(&self) -> u64 {
                self.total
            }

            fn current(&self) -> u64 {
                self.page
            }

            fn has_next(&self) -> bool {
                self.page < self.total()
            }

            fn has_prev(&self) -> bool {
                self.page == 1
            }

            /*
            async fn next_page(self) -> Option<ApiResult<Self>> {
                if self.has_next() {
                    if let Some(params) = &self.params {
                        if let Some(client) = &self.client {
                            let mut uri = params.uri.to_string();
                            for (key, value) in params.iter() {
                                if key == "page" {
                                    uri = uri.replace(key, &(self.current() + 1).to_string());
                                } else {
                                    uri = uri.replace(key, value);
                                }
                            }
                            return Some(client.send(client.get(params.host, &uri)).await);
                        }
                    }
                }
                None
            }

            async fn prev_page(self) -> Option<ApiResult<Self>> {
                if self.has_prev() {
                    if let Some(params) = &self.params {
                        if let Some(client) = &self.client {
                            let mut uri = params.uri.to_string();
                            for (key, value) in params.iter() {
                                if key == "page" {
                                    uri = uri.replace(key, &(self.current() - 1).to_string());
                                } else {
                                    uri = uri.replace(key, value);
                                }
                            }
                            return Some(client.send(client.get(params.host, &uri)).await);
                        }
                    }
                }
                None
            }
            */
        }
    };
}

impl Sort {
    pub fn as_str(&self) -> &str {
        match self {
            Self::DescByDate => "dd",
            Self::AscByDate => "da",
            Self::MaxLike => "ld",
            Self::MaxSearch => "vd",
        }
    }
}

impl FromStr for Sort {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "descbydate" | "dd" => Self::DescByDate,
            "ascbydate" | "da" => Self::AscByDate,
            "maxlike" | "ld" => Self::MaxLike,
            "maxsearch" | "vd" => Self::MaxSearch,
            _ => Err(())?,
        })
    }
}

impl ToString for Sort {
    fn to_string(&self) -> String {
        match self {
            Self::DescByDate => "dd",
            Self::AscByDate => "da",
            Self::MaxLike => "ld",
            Self::MaxSearch => "vd",
        }
        .to_owned()
    }
}

type HmacSha256 = Hmac<Sha256>;

fn hmac_sha256(data: &[u8], key: &[u8]) -> Result<[u8; 32], InvalidLength> {
    let mut hmac_digest = HmacSha256::new_from_slice(key)?;
    hmac_digest.update(data);
    Ok(hmac_digest.finalize().into_bytes().adapt())
}

pub fn nonce() -> [u8; 32] {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(32)
        .collect::<Vec<u8>>()
        .adapt()
}

pub fn signature(
    uri: &str,
    time: u64,
    nonce: &str,
    method: &str,
) -> Result<[u8; 32], InvalidLength> {
    hmac_sha256(
        format!(
            "{}{}{}{}{}",
            &uri[1..],
            time,
            nonce.to_lowercase(),
            method.to_lowercase(),
            app::KEY
        )
        .to_lowercase()
        .as_bytes(),
        app::SECRET.as_bytes(),
    )
}

impl ToString for Quality {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

impl<'a> Header<'a> {
    pub fn new(method: &str, uri: &str, nonce: &'a str) -> Self {
        let now_time = time::SystemTime::now()
            .duration_since(time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            host: api::host::DEFAULT.read().unwrap()[8..].to_string(),
            app_uuid: Self::UUID,
            api_key: app::KEY,
            app_channel: app::CHANNEL,
            app_platform: app::PLATFORM,
            accept: Self::ACCEPT,
            nonce,
            time: now_time.to_string(),
            content_type: Self::CONTENT_TYPE,
            signature: signature(uri, now_time, nonce, method)
                .unwrap()
                .encode_hex::<String>(),
            user_agent: Self::USER_AGENT,
            image_quality: Quality::default(),
            authorization: None,
        }
    }
}

impl Into<HeaderMap> for Header<'_> {
    fn into(self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(header_name::HOST, self.host.parse().unwrap());
        map.insert(header_name::APP_UUID, self.app_uuid.parse().unwrap());
        map.insert(header_name::API_KEY, self.api_key.parse().unwrap());
        map.insert(header_name::APP_CHANNEL, self.app_channel.parse().unwrap());
        map.insert(
            header_name::APP_PLATFORM,
            self.app_platform.parse().unwrap(),
        );
        map.insert(header_name::ACCEPT, self.accept.parse().unwrap());
        map.insert(
            header_name::NONCE,
            self.nonce.to_lowercase().parse().unwrap(),
        );
        map.insert(header_name::TIME, self.time.parse().unwrap());
        map.insert(
            header_name::CONTENT_TYPE,
            self.content_type.parse().unwrap(),
        );
        map.insert(
            header_name::SIGNATURE,
            self.signature.to_lowercase().parse().unwrap(),
        );
        map.insert(header_name::USER_AGENT, self.user_agent.parse().unwrap());
        map.insert(
            header_name::IMAGE_QUALITY,
            self.image_quality.to_string().parse().unwrap(),
        );
        if let Some(token) = self.authorization {
            map.insert(header_name::AUTHORIZATION, token.parse().unwrap());
        }
        map
    }
}

impl<T: Debug> Response<T> {
    pub const fn code(&self) -> u64 {
        self.code
    }
    pub fn message(&self) -> &str {
        &self.message
    }

    pub const fn is_success(&self) -> bool {
        self.code == 200
    }

    pub const fn is_error(&self) -> bool {
        !self.is_success()
    }

    pub fn detail(&self) -> Option<&str> {
        self.detail.as_ref().map(|v| v.as_str())
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_ref().map(|v| v.as_str())
    }

    pub const fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }
}

impl PictureDownloadResounce {
    pub fn filename(&self) -> &str {
        &self.original_name
    }

    pub fn server(&self) -> &str {
        &self.file_server
    }

    pub fn resource_path(&self) -> &str {
        &self.path
    }

    pub fn download_url(&self) -> reqwest::Url {
        let resource_path = PathBuf::from_str(self.resource_path()).unwrap();
        format!("{}/static/{}", self.server(), resource_path.file_name().unwrap().to_str().unwrap())
            .parse()
            .unwrap()
    }
}

impl<T> Pagible for Docs<T> {
    fn total(&self) -> u64 {
        self.total
    }

    fn current(&self) -> u64 {
        self.page
    }

    fn has_next(&self) -> bool {
        self.page < self.pages
    }

    fn has_prev(&self) -> bool {
        self.page > 1
    }
}


/*
impl_pagible!(Favourites);
impl_pagible!(Search);
impl_pagible!(Comments<GameComment>);
impl_pagible!(Comments<GameChildrenComment>);
impl_pagible!(Pages);
impl_pagible!(Games);
impl_pagible!(Announcements);
*/
