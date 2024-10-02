pub mod host {
    use std::sync::RwLock;

    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref DEFAULT: RwLock<String> = RwLock::new("https://api.manhuabika.com".to_owned());
        pub static ref RECOMMEND: RwLock<String> = RwLock::new("https://recommend.manhuabika.com".to_owned());
        //pub static ref PIC: RwLock<String> = RwLock::new("https://img.picacomic.com".to_owned());
        pub static ref PIC: RwLock<Option<String>> = RwLock::new(None);
    }
}
pub mod auth {
    pub const LOGIN: &'static str = "/auth/sign-in";
    // @method: post
    // @post-params email: str
    // @post-params password: str
}
pub mod user {
    pub const PROFILE: &'static str = "/users/profile";
    // @method: get
    pub const FAVOURITES: &'static str = "/users/favourite?page=:page&s=:sort";
    // @method: get
    // @params page: number
    // @params s: Sort
    pub const COMMENTS: &'static str = "/users/my-comments?page=:page";
    // @method: get
    // @params page: number
    pub const PUNCH_IN: &'static str = "/users/punch-in";
    // @method: post
}
pub mod comic {
    pub const RANKING: &'static str = "/comics/leaderboard?tt=H24&ct=VC";
    // @method: get
    pub const COMMENTS: &'static str = "/comics/:cid/comments?page=:page";
    // @method: get
    // @path_params cid: str
    // @params page: number
    pub const EPS: &'static str = "/comics/:cid/eps?page=:page";
    // @method: get
    // @path_params cid: str
    // @params page: number
    pub const RECOMMENDED: &'static str = "/comics/:cid/recommendation";
    // @method: get
    // @params cid: str
    pub const PAGES: &'static str = "/comics/:cid/order/:index/pages?page=:page";
    // @method: get
    // @path_params cid: str
    // @path_params index: number
    // @params page: number
    pub const METADATA: &'static str = "/comics/:cid";
    // @method: get
    // @params cid: str
    pub const SEARCH: &'static str = "/comics/advanced-search?page=:page&s=:sort";
    // @method: post
    // @post-params keyword: str
    // @post-params sort: Sort
    // @params page: number
    // @params s: Sort
}
pub mod game {
    pub const GAMES: &'static str = "/games?page=:page";
    // @method: get
    // @params page: number
    pub const INFO: &'static str = "/games/:cid";
    // @method: get
    // @path_params cid: str
    pub const COMMENTS: &'static str = "/games/:cid/comments?page=:page";
    // @method: get
    // @path_params cid: str
    // @params page: number
}
pub mod comment {
    pub const CHILDRENS: &'static str = "/comments/:cid/childrens?page=:page";
    // @method: get
    // @path_params cid: str
    // @params page: number
}
pub mod other {
    pub const ANNOUNCEMENTS: &'static str = "/announcements?page=:page";
    // @method: get
    // @params page: number
    pub const PIC_LIKE_GET: &'static str = "/pic-like-get/?c=:cid&page=:page";
    // @host: recommended_host
    // @method: get
    // @path_params cid: str
    // @params page: number
    pub const KEYWORDS: &'static str = "/keywords";
    // @method: get
    pub const CATEGORIES: &'static str = "/categories";
    // @method: get
}
