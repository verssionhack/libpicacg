use std::{collections::HashMap, fmt::Debug, sync::Arc, time::Duration};

use reqwest::{Proxy, RequestBuilder, ClientBuilder, Client};


use serde::{de::DeserializeOwned};
use std::sync::RwLock;

use crate::{api_type::Api, nonce, Header, error::Error, api::{self}, Response, responses::{self, ComicMetadata, Comics, PunchIn, Profile, Keywords, Categories, RecommendPicLike, Comments, Eps, Pages, Search, Games, GameInfo, ComicComment, GameComment, Announcements, Favourites, GameDownloadResponse}, ApiResult, Sort, Parmas};


impl Debug for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Api")
    }
}

impl Debug for Parmas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parmas")
    }
}

impl Api {
    pub fn new() -> Self {
        let nonce = String::from_utf8(nonce().to_vec()).unwrap();
        Self {
            client: Arc::new(RwLock::new(Client::new())),
            nonce,
            token: None,
            proxy: None,
            email: None,
            password: None,
        }
    }

    fn reset_client(&mut self) -> Result<(), Error> {
        let mut client_builder = ClientBuilder::new();
        if let Some(v) = self.proxy.as_ref() {
            client_builder = client_builder.proxy(v.clone());
        }
        if let Some(v) = self.timeout.as_ref() {
            client_builder = client_builder.timeout(v.clone());
        }
        *self.client.write().unwrap() = client_builder.build()?;
        Ok(())
    }

    pub fn set_proxy(&mut self, proxy: Option<Proxy>) -> Result<(), Error> {
        self.proxy = proxy;
        self.reset_client()?;
        Ok(())
    }

    pub fn set_timeout(&mut self, timeout: Option<Duration>) -> Result<(), Error> {
        self.timeout = timeout;
        self.reset_client()?;
        Ok(())
    }

    pub fn proxy(&self) -> Option<&Proxy> {
        self.proxy.as_ref()
    }

    pub fn timeout(&self) -> Option<&Duration> {
        self.timeout.as_ref()
    }
    
    pub fn header(&self, method: &str, uri: &str) -> Header<'_> {
        let mut header = Header::new(method, uri, &self.nonce);
        if let Some(token) = self.token.as_ref() {
            header.authorization = Some(&token);
        }
        //println!("{:#?}", header);
        header
    }

    pub async fn login(&mut self, email: &str, password: &str) -> Result<(), Error> {
        self.email = Some(email.to_string());
        self.password = Some(password.to_string());

        let mut payload = HashMap::new();
        payload.insert("email", email);
        payload.insert("password", password);
        let res: responses::Auth = self.send(self.post(api::host::DEFAULT, api::auth::LOGIN)
            .json(&payload)).await?;
        self.token = Some(res.token);
        Ok(())
    }

    pub async fn favorites(&self, page: u64, sort: Sort) -> ApiResult<Favourites> {
        self.send(
            self.get(api::host::DEFAULT, &api::user::FAVOURITES
                .replace(
                    ":page", &page.to_string()
                )
                .replace(
                    ":sort", sort.as_str()
                ))
        ).await
    }

    pub async fn comic_ranking(&self) -> ApiResult<Comics> {
        self.send(
            self.get(api::host::DEFAULT, api::comic::RANKING)
        ).await
    }

    
    pub async fn comic_metadata(&self, cid: &str) -> ApiResult<ComicMetadata> {
        self.send(
            self.get(api::host::DEFAULT, &api::comic::METADATA.replace(":cid", cid))
        ).await
    }

    pub async fn comic_comments(&self, cid: &str, page: u64) -> ApiResult<Comments<ComicComment>> {
        self.send(
            self.get(api::host::DEFAULT, &api::comic::COMMENTS.replace(":cid", cid)
                .replace(
                    ":page", &page.to_string()
                ))
        ).await
    }

    pub async fn comic_recommended(&self, cid: &str) -> ApiResult<Comics> {
        self.send(
            self.get(api::host::DEFAULT, &api::comic::RECOMMENDED.replace(":cid", cid))
        ).await
    }

    pub async fn comic_eps(&self, cid: &str, page: u64) -> ApiResult<Eps> {
        self.send(
            self.get(api::host::DEFAULT, &api::comic::EPS.replace(":cid", cid)
                .replace(
                    ":page", &page.to_string()
                ))
        ).await
    }

    pub async fn comic_pages(&self, cid: &str, index: u64, page: u64) -> ApiResult<Pages> {
        self.send(
            self.get(api::host::DEFAULT, &api::comic::PAGES
                .replace(":cid", cid)
                .replace(":index", &index.to_string())
                .replace(
                    ":page", &page.to_string()
                ))
        ).await
    }

    pub async fn game_comments(&self, cid: &str, page: u64) -> ApiResult<Comments<GameComment>> {
        self.send(
            self.get(api::host::DEFAULT, &api::game::COMMENTS
                .replace(":cid", cid)
                .replace(
                    ":page", &page.to_string()
                ))
        ).await
    }

    pub async fn children_comments<T>(&self, cid: &str, page: u64) -> ApiResult<Comments<T>>
    where T: DeserializeOwned + Debug
    {
        self.send(
            self.get(api::host::DEFAULT, &api::comment::CHILDRENS
                .replace(":cid", cid)
                .replace(
                    ":page", &page.to_string()
                ))
        ).await
    }

    pub async fn games(&self, page: u64) -> ApiResult<Games> {
        self.send(
            self.get(api::host::DEFAULT, &api::game::GAMES
                .replace(
                    ":page", &page.to_string()
                ))
        ).await
    }

    pub async fn game_info(&self, cid: &str) -> ApiResult<GameInfo> {
        self.send(
            self.get(api::host::DEFAULT, &api::game::INFO
                .replace(":cid", cid))
        ).await
    }

    pub async fn game_download_info_get(&self, url: &str) -> Result<GameDownloadResponse, Error> {
        let mut game_url = reqwest::Url::parse(url).unwrap();
        game_url.set_path(&format!("/api/v1{}", game_url.path()));
        Ok(self.client.read().unwrap().get(game_url)
            .header("referer", url)
            .send().await?.json().await?)
    }

    pub async fn punch_in(&self) -> ApiResult<PunchIn> {
        self.send(
            self.post(api::host::DEFAULT, &api::user::PUNCH_IN)
        ).await
    }

    pub async fn profile(&self) -> ApiResult<Profile> {
        self.send(
            self.get(api::host::DEFAULT, &api::user::PROFILE)
        ).await
    }

    pub async fn keywords(&self) -> ApiResult<Keywords> {
        self.send(
            self.get(api::host::DEFAULT, &api::other::KEYWORDS)
        ).await
    }

    pub async fn announcements(&self, page: u64) -> ApiResult<Announcements> {
        self.send(
            self.get(api::host::DEFAULT, &api::other::ANNOUNCEMENTS
                .replace(
                    ":page", &page.to_string()
                ))
        ).await
    }

    pub async fn categories(&self) -> ApiResult<Categories> {
        self.send(
            self.get(api::host::DEFAULT, &api::other::CATEGORIES)
        ).await
    }

    pub async fn pic_like_get(&self, cid: &str, page: u64) ->  Result<Vec<RecommendPicLike>, Error> {
        Ok(self.client.read().unwrap().get(&format!("{}{}", api::host::RECOMMEND, &api::other::PIC_LIKE_GET
                .replace(
                    ":cid", cid 
                )
                .replace(
                    ":page", &page.to_string()
                )
                    )).send().await?.json().await?)
    }

    pub async fn search(&self, keyword: &str, page: u64, sort: Sort) -> ApiResult<Search> {
        let mut payload = HashMap::new();
        payload.insert("keyword", keyword);
        payload.insert("sort", sort.as_str());
        self.send(
            self.post(api::host::DEFAULT, &api::comic::SEARCH
                .replace(
                    ":page", &page.to_string()
                )
                .replace(
                    ":sort", sort.as_str()
                ))
            .json(&payload)
        ).await
    }


    pub fn get(&self, host: &str, uri: &str) -> RequestBuilder {
        self.client.read().unwrap().get(format!("{}{}", host, uri))
            .headers(self.header("get", uri).into())
    }

    pub fn post(&self, host: &str, uri: &str) -> RequestBuilder {
        self.client.read().unwrap().post(format!("{}{}", host, uri))
            .headers(self.header("post", uri).into())
    }


    /*
    pub async fn send<T: Debug + DeserializeOwned>(&self, builder: RequestBuilder) -> ApiResult<T> {
        let req =  builder.build().unwrap();
        //println!("Request {}", req.url().as_str());
        //println!("RequestHeader {:#?}", req.headers());
        let res = self.client.read().unwrap().execute(req).await?;
        let text = res.text().await?;
        println!("Parsing {}", &text);
        Ok(serde_json::from_str::<Response<T>>(&text)
            .map(|response| {
                if response.is_success() {
                    Ok::<T, Error>(response.data.unwrap())
                } else {
                    Err(response.into())
                }
            })??
            )
    }
    */


    pub async fn send<T: Debug + DeserializeOwned>(&self, builder: RequestBuilder) -> ApiResult<T> {
        Ok(builder.send().await?.json::<Response<T>>().await
            .map(|response| {
                if response.is_success() {
                    Ok::<T, Error>(response.data.unwrap())
                } else {
                    Err(response.into())
                }
            })??
            )
    }
}
