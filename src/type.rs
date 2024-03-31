use std::fmt::Debug;

use serde::Deserialize;

use std::ops::{Deref, DerefMut};

//pub type ApiResult<T> = Result<Response<T>, Error>;
pub type ApiResult<T> = Result<T, Error>;


use crate::error::Error;

use crate::r#impl::{game_download_info_deserializer, num_correct_deserializer};

pub mod app {
    pub const VERSION: &'static str = "2.2.1.3.3.4";
    pub const CHANNEL: &'static str = "1";
    pub const PLATFORM: &'static str = "android";
    pub const KEY: &'static str = "C69BAF41DA5ABD1FFEDC6D2FEA56B";
    pub const SECRET: &'static str =
        "~d}$Q7$eIni=V)9\\RK/P.RM4;9[7|@/CA}b~OW!3?EV`:<>M7pddUBL5n|0/*Cn";
    pub const BUILD_VERSION: u16 = 45;
}

#[derive(Debug, Clone)]
pub struct Header<'a> {
    pub(super) host: &'static str,
    pub(super) app_uuid: &'static str,
    pub(super) api_key: &'static str,
    pub(super) app_channel: &'static str,
    pub(super) app_platform: &'static str,
    pub(super) accept: &'static str,
    pub(super) nonce: &'a str,
    pub(super) time: String,
    pub(super) content_type: &'static str,
    pub(super) signature: String,
    pub(super) user_agent: &'static str,
    pub(super) image_quality: Quality,
    pub(super) authorization: Option<&'a str>,
}

impl Header<'_> {
    pub const ACCEPT: &'static str = "application/vnd.picacomic.com.v1+json";
    pub const USER_AGENT: &'static str =  "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36";
    pub const CONTENT_TYPE: &'static str = "application/json; charset=UTF-8";
    pub const UUID: &'static str = "webUUID";
}

pub mod header_name {
    pub const HOST: reqwest::header::HeaderName = reqwest::header::HeaderName::from_static("host");
    pub const APP_UUID: reqwest::header::HeaderName =
        reqwest::header::HeaderName::from_static("app-uuid");
    pub const API_KEY: reqwest::header::HeaderName =
        reqwest::header::HeaderName::from_static("api-key");
    pub const APP_CHANNEL: reqwest::header::HeaderName =
        reqwest::header::HeaderName::from_static("app-channel");
    pub const APP_PLATFORM: reqwest::header::HeaderName =
        reqwest::header::HeaderName::from_static("app-platform");
    pub const ACCEPT: reqwest::header::HeaderName =
        reqwest::header::HeaderName::from_static("accept");
    pub const NONCE: reqwest::header::HeaderName =
        reqwest::header::HeaderName::from_static("nonce");
    pub const TIME: reqwest::header::HeaderName = reqwest::header::HeaderName::from_static("time");
    pub const CONTENT_TYPE: reqwest::header::HeaderName =
        reqwest::header::HeaderName::from_static("content-type");
    pub const SIGNATURE: reqwest::header::HeaderName =
        reqwest::header::HeaderName::from_static("signature");
    pub const USER_AGENT: reqwest::header::HeaderName =
        reqwest::header::HeaderName::from_static("user-agent");
    pub const IMAGE_QUALITY: reqwest::header::HeaderName =
        reqwest::header::HeaderName::from_static("image-quality");
    pub const AUTHORIZATION: reqwest::header::HeaderName =
        reqwest::header::HeaderName::from_static("authorization");
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Quality {
    Medium,
    #[default]
    Original,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Sort {
    #[default]
    DescByDate,
    AscByDate,
    MaxLike,
    MaxSearch,
}

#[derive(Debug, Deserialize)]
pub struct Response<T: Debug> {
    #[serde(deserialize_with="num_correct_deserializer")]
    pub code: u64,
    pub message: String,
    pub(super) error: Option<String>,
    pub(super) detail: Option<String>,
    pub(super) data: Option<T>,
}

pub mod responses {

    
    

    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct PictureDownloadResounce {
        #[serde(rename = "originalName")]
        pub(crate) original_name: String,
        pub(crate) path: String,
        #[serde(rename = "fileServer")]
        pub(crate) file_server: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Auth {
        pub token: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct PunchIn {
        res: _PunchIn,
    }

    impl Deref for PunchIn {
        type Target = _PunchIn;
        fn deref(&self) -> &Self::Target {
            &self.res
        }
    }

    impl DerefMut for PunchIn {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.res
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct _PunchIn {
        #[serde(rename = "punchInLastDay")]
        pub punch_in_last_day: String,
        pub status: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Creator {
        #[serde(rename = "_id")]
        pub id: String,
        pub avatar: Option<PictureDownloadResounce>,
        pub character: Option<String>,
        pub characters: Vec<String>,
        #[serde(deserialize_with="num_correct_deserializer")]
        pub exp: u64,
        pub gender: String,
        #[serde(deserialize_with="num_correct_deserializer")]
        pub level: u64,
        pub name: String,
        #[serde(default)]
        pub role: String,
        pub slogan: Option<String>,
        #[serde(default)]
        pub title: String,
        #[serde(default)]
        pub verified: bool,
    }

    #[derive(Debug, Deserialize)]
    pub struct ComicMetadata {
        comic: _ComicMetadata,
    }

    impl Deref for ComicMetadata {
        type Target = _ComicMetadata;
        fn deref(&self) -> &Self::Target {
            &self.comic
        }
    }

    impl DerefMut for ComicMetadata {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.comic
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct _ComicMetadata {
        #[serde(flatten)]
        pub metadata: Comic,
        #[serde(rename = "_creator")]
        pub creator: Creator,
        pub description: Option<String>,
        pub tags: Vec<String>,
        #[serde(rename = "chineseTeam")]
        pub chinese_team: Option<String>,
        pub updated_at: String,
        pub created_at: String,
        #[serde(rename = "allowDownload")]
        pub allow_download: bool,
        #[serde(rename = "allowComment")]
        pub allow_comment: bool,
        #[serde(rename = "isFavourite")]
        pub is_favourite: bool,
        #[serde(rename = "isLiked")]
        pub is_liked: bool,
        #[serde(rename = "likesCount", default, deserialize_with="num_correct_deserializer")]
        pub likes_count: u64,
        #[serde(rename = "totalComments", default, deserialize_with="num_correct_deserializer")]
        pub total_comments: u64,
    }

    #[derive(Debug, Deserialize)]
    pub struct Favourites {
        comics: Docs<Comic>,
        /*
        #[serde(skip)]
        pub(crate) client: Option<Arc<Api>>,
        #[serde(skip)]
        pub(crate) params: Option<Parmas>,
        */
    }

    impl Deref for Favourites {
        type Target = Docs<Comic>;
        fn deref(&self) -> &Self::Target {
            &self.comics
        }
    }

    impl DerefMut for Favourites {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.comics
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Comics {
        comics: Vec<Comic>,
    }

    impl Deref for Comics {
        type Target = Vec<Comic>;
        fn deref(&self) -> &Self::Target {
            &self.comics
        }
    }

    impl DerefMut for Comics {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.comics
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Search {
        comics: Docs<SearchRow>,
        /*
        #[serde(skip)]
        pub(crate) client: Option<Arc<Api>>,
        #[serde(skip)]
        pub(crate) params: Option<Parmas>,
        */
    }

    impl Deref for Search {
        type Target = Docs<SearchRow>;
        fn deref(&self) -> &Self::Target {
            &self.comics
        }
    }

    impl DerefMut for Search {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.comics
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct SearchRow {
        #[serde(flatten)]
        comic: Comic,
        #[serde(rename = "chineseTeam")]
        pub chinese_team: Option<String>,
        pub created_at: String,
        pub updated_at: String,
        pub description: Option<String>,
        pub tags: Vec<String>,
    }

    impl Deref for SearchRow {
        type Target = Comic;
        fn deref(&self) -> &Self::Target {
            &self.comic
        }
    }

    impl DerefMut for SearchRow {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.comic
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Comic {
        #[serde(rename = "_id")]
        pub id: String,
        pub author: String,
        pub categories: Vec<String>,
        pub title: String,
        #[serde(rename = "totalViews", default, deserialize_with="num_correct_deserializer")]
        pub total_views: u64,
        #[serde(rename = "totalLikes", default, deserialize_with="num_correct_deserializer")]
        pub total_likes: u64,
        #[serde(rename = "likesCount", default, deserialize_with="num_correct_deserializer")]
        pub likes_count: u64,
        #[serde(rename = "pagesCount", default, deserialize_with="num_correct_deserializer")]
        pub pages_count: u64,
        #[serde(rename = "epsCount", default, deserialize_with="num_correct_deserializer")]
        pub eps_count: u64,
        #[serde(default)]
        pub finished: bool,
        pub thumb: PictureDownloadResounce,
        #[serde(rename = "viewsCount", default, deserialize_with="num_correct_deserializer")]
        pub views_count: u64,
        #[serde(rename = "leaderboardCount", default, deserialize_with="num_correct_deserializer")]
        pub leader_board_count: u64,
    }

    #[derive(Debug, Deserialize)]
    pub struct Profile {
        user: _Profile,
    }

    impl Deref for Profile {
        type Target = _Profile;
        fn deref(&self) -> &Self::Target {
            &self.user
        }
    }

    impl DerefMut for Profile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.user
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct _Profile {
        #[serde(rename = "_id")]
        pub id: String,
        pub birthday: String,
        pub character: Option<String>,
        pub characters: Vec<String>,
        pub created_at: String,
        pub email: String,
        #[serde(deserialize_with="num_correct_deserializer")]
        pub exp: u64,
        pub gender: String,
        #[serde(rename = "isPunched")]
        pub is_punched: bool,
        #[serde(deserialize_with="num_correct_deserializer")]
        pub level: u64,
        pub name: String,
        pub title: String,
        #[serde(default)]
        pub verified: bool,
    }

    #[derive(Debug, Deserialize)]
    pub struct Keywords {
        keywords: Vec<String>,
    }

    impl Deref for Keywords {
        type Target = Vec<String>;
        fn deref(&self) -> &Self::Target {
            &self.keywords
        }
    }

    impl DerefMut for Keywords {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.keywords
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Categories {
        categories: Vec<Categorie>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Categorie {
        #[serde(rename = "_id")]
        pub id: Option<String>,
        #[serde(default)]
        pub active: bool,
        #[serde(rename = "isWeb", default)]
        pub is_web: bool,
        pub link: Option<String>,
        pub description: Option<String>,
        pub thumb: PictureDownloadResounce,
        pub title: String,
    }

    impl Deref for Categories {
        type Target = Vec<Categorie>;
        fn deref(&self) -> &Self::Target {
            &self.categories
        }
    }

    impl DerefMut for Categories {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.categories
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct RecommendPicLike {
        pub id: String,
        pub title: String,
        pub pic: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Ep {
        #[serde(rename = "_id")]
        pub id: String,
        pub order: Option<u64>,
        pub title: String,
        pub updated_at: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Eps {
        eps: Docs<Ep>,
        /*
        #[serde(skip)]
        pub(crate) client: Option<Arc<Api>>,
        #[serde(skip)]
        pub(crate) params: Option<Parmas>,
        */
    }

    impl Deref for Eps {
        type Target = Docs<Ep>;
        fn deref(&self) -> &Self::Target {
            &self.eps
        }
    }

    impl DerefMut for Eps {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.eps
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Docs<T> {
        docs: Vec<T>,
        #[serde(deserialize_with="num_correct_deserializer")]
        pub limit: u64,
        #[serde(deserialize_with="num_correct_deserializer")]
        pub page: u64,
        #[serde(deserialize_with="num_correct_deserializer")]
        pub pages: u64,
        #[serde(deserialize_with="num_correct_deserializer")]
        pub total: u64,
    }

    impl<T> Deref for Docs<T> {
        type Target = Vec<T>;
        fn deref(&self) -> &Self::Target {
            &self.docs
        }
    }

    impl<T> DerefMut for Docs<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.docs
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Comments<T> {
        comments: Docs<T>,
        #[serde(rename = "topComments")]
        pub top_comments: Option<Vec<T>>,
        /*
        #[serde(skip)]
        pub(crate) client: Option<Arc<Api>>,
        #[serde(skip)]
        pub(crate) params: Option<Parmas>,
        */
    }
    impl<T> Deref for Comments<T> {
        type Target = Docs<T>;
        fn deref(&self) -> &Self::Target {
            &self.comments
        }
    }

    impl<T> DerefMut for Comments<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.comments
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct ComicComment {
        #[serde(rename = "_comic")]
        pub comic: String,
        #[serde(rename = "_id")]
        pub id: String,
        #[serde(rename = "_user")]
        pub user: Creator,
        #[serde(rename = "commentsCount", deserialize_with="num_correct_deserializer")]
        pub comments_count: u64,
        #[serde(default)]
        pub content: String,
        pub created_at: String,
        pub hide: bool,
        #[serde(rename = "isLiked")]
        pub is_liked: bool,
        #[serde(rename = "isTop")]
        pub is_top: bool,
        #[serde(rename = "likesCount", deserialize_with="num_correct_deserializer")]
        pub likes_count: u64,
        #[serde(rename = "totalComments", deserialize_with="num_correct_deserializer")]
        pub total_comments: u64,
    }

    #[derive(Debug, Deserialize)]
    pub struct Page {
        #[serde(rename = "_id")]
        pub id: String,
        pub media: PictureDownloadResounce,
    }

    #[derive(Debug, Deserialize)]
    pub struct Pages {
        pages: Docs<Page>,
        pub ep: Ep,
        /*
        #[serde(skip)]
        pub(crate) client: Option<Arc<Api>>,
        #[serde(skip)]
        pub(crate) params: Option<Parmas>,
        */
    }

    impl Deref for Pages {
        type Target = Docs<Page>;
        fn deref(&self) -> &Self::Target {
            &self.pages
        }
    }

    impl DerefMut for Pages {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.pages
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Games {
        games: Docs<Game>,
        /*
        #[serde(skip)]
        pub(crate) client: Option<Arc<Api>>,
        #[serde(skip)]
        pub(crate) params: Option<Parmas>,
        */
    }

    impl Deref for Games {
        type Target = Docs<Game>;
        fn deref(&self) -> &Self::Target {
            &self.games
        }
    }

    impl DerefMut for Games {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.games
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Game {
        #[serde(rename = "_id")]
        pub id: String,
        #[serde(default)]
        pub adult: bool,
        #[serde(default)]
        pub android: bool,
        pub icon: PictureDownloadResounce,
        #[serde(default)]
        pub ios: bool,
        pub publisher: String,
        #[serde(default)]
        pub suggest: bool,
        pub title: String,
        pub version: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct GameInfo {
        game: _GameInfo,
    }

    impl Deref for GameInfo {
        type Target = _GameInfo;
        fn deref(&self) -> &Self::Target {
            &self.game
        }
    }

    impl DerefMut for GameInfo {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.game
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct _GameInfo {
        #[serde(flatten)]
        game: Game,
        #[serde(rename = "androidLinks")]
        pub android_links: Vec<String>,
        #[serde(rename = "androidSize")]
        pub android_size: f64,
        #[serde(rename = "iosLinks")]
        pub ios_links: Vec<String>,
        #[serde(rename = "iosSize")]
        pub ios_size: f64,
        #[serde(rename = "commentsCount", deserialize_with="num_correct_deserializer")]
        pub comments_count: u64,
        pub created_at: String,
        pub description: Option<String>,
        #[serde(rename = "downloadsCount", deserialize_with="num_correct_deserializer")]
        pub downloads_count: u64,
        #[serde(rename = "isLiked")]
        pub is_liked: bool,
        #[serde(rename = "likesCount", deserialize_with="num_correct_deserializer")]
        pub likes_count: u64,
        pub screenshots: Vec<PictureDownloadResounce>,
        pub updated_at: String,
        #[serde(rename = "videoLink")]
        pub video_link: String,
    }

    impl Deref for _GameInfo {
        type Target = Game;
        fn deref(&self) -> &Self::Target {
            &self.game
        }
    }

    impl DerefMut for _GameInfo {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.game
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct GameComment {
        #[serde(rename = "_game")]
        pub game: String,
        #[serde(rename = "_id")]
        pub id: String,
        #[serde(rename = "_user")]
        pub user: Creator,
        #[serde(rename = "commentsCount", deserialize_with="num_correct_deserializer")]
        pub comments_count: u64,
        pub content: String,
        pub created_at: String,
        #[serde(default)]
        pub hide: bool,
        #[serde(rename = "isLiked")]
        pub is_liked: bool,
        #[serde(rename = "isTop")]
        pub is_top: bool,
        #[serde(rename = "likesCount", deserialize_with="num_correct_deserializer")]
        pub likes_count: u64,
        #[serde(rename = "totalComments", deserialize_with="num_correct_deserializer")]
        pub total_comments: u64,
    }

    #[derive(Debug, Deserialize)]
    pub struct GameChildrenComment {
        #[serde(rename = "_game")]
        pub game: String,
        #[serde(rename = "_id")]
        pub id: String,
        #[serde(rename = "_parent")]
        pub parent: String,
        #[serde(rename = "_user")]
        pub user: Creator,
        pub content: String,
        pub created_at: String,
        #[serde(default)]
        pub hide: bool,
        #[serde(rename = "isLiked")]
        pub is_liked: bool,
        #[serde(rename = "isTop")]
        pub is_top: bool,
        #[serde(rename = "likesCount", deserialize_with="num_correct_deserializer")]
        pub likes_count: u64,
        #[serde(rename = "totalComments", deserialize_with="num_correct_deserializer")]
        pub total_comments: u64,
    }

    #[derive(Debug, Deserialize)]
    pub struct ComicChildrenComment {
        #[serde(rename = "_comic")]
        pub comic: String,
        #[serde(rename = "_id")]
        pub id: String,
        #[serde(rename = "_parent")]
        pub parent: String,
        #[serde(rename = "_user")]
        pub user: Creator,
        pub content: String,
        pub created_at: String,
        #[serde(default)]
        pub hide: bool,
        #[serde(rename = "isLiked")]
        pub is_liked: bool,
        #[serde(rename = "isTop")]
        pub is_top: bool,
        #[serde(rename = "likesCount", deserialize_with="num_correct_deserializer")]
        pub likes_count: u64,
        #[serde(rename = "totalComments", deserialize_with="num_correct_deserializer")]
        pub total_comments: u64,
    }

    #[derive(Debug, Deserialize)]
    pub struct Announcement {
        #[serde(rename = "_id")]
        pub id: String,
        pub content: String,
        pub thumb: PictureDownloadResounce,
        pub title: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Announcements {
        announcements: Docs<Announcement>,
        /*
        #[serde(skip)]
        pub(crate) client: Option<Arc<Api>>,
        #[serde(skip)]
        pub(crate) params: Option<Parmas>,
        */
    }

    impl Deref for Announcements {
        type Target = Docs<Announcement>;
        fn deref(&self) -> &Self::Target {
            &self.announcements
        }
    }

    impl DerefMut for Announcements {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.announcements
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct GameDownloadInfoP2p {
        pub bt: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct GameDownloadInfoDrive {
        pub onedrive: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct GameDownloadInfoS3 {
        pub sg: String,
        pub sg2: String,
        pub us: String,
    }

    #[derive(Debug)]
    pub struct GameDownloadInfo {
        pub node: Vec<String>,
        pub p2p: GameDownloadInfoP2p,
        pub drive: GameDownloadInfoDrive,
        pub s3: GameDownloadInfoS3,
    }

    #[derive(Debug, Deserialize)]
    pub struct GameDownloadResponse {
        #[serde(deserialize_with="num_correct_deserializer")]
        pub code: u64,
        pub title: String,
        pub description: String,
        #[serde(deserialize_with="game_download_info_deserializer")]
        pub download: GameDownloadInfo,
    }
}
