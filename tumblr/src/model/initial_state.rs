use crate::ContentBlock;

/// Initial State Json
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct InitialState {
    #[serde(rename = "apiFetchStore")]
    pub api_fetch_store: ApiFetchStore,

    /// ?
    #[serde(rename = "PeeprRoute")]
    pub peepr_route: Option<PeeprRoute>,
}

/// apiFetchStore field.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ApiFetchStore {
    #[serde(rename = "API_TOKEN")]
    pub api_token: Box<str>,
}

/// PeeprRoute field.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PeeprRoute {
    /// ?
    #[serde(rename = "initialTimeline")]
    pub initial_timeline: InitialTimeline,
}

/// initialTimeline field.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct InitialTimeline {
    pub objects: Box<[TimelineObject]>,
}

/// Timeline object
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "objectType")]
pub enum TimelineObject {
    #[serde(rename = "post")]
    Post(Post),
}

/// A post
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Post {
    /// The post type.
    ///
    /// If formatting as NPF, the type will be blocks; if formatting as legacy,
    /// the type will be one of the original legacy types (text, photo, quote, chat, link, video, audio).
    #[serde(rename = "type")]
    pub kind: Box<str>,

    /// The post ID
    #[serde(with = "serde_str_u64")]
    pub id: u64,

    /// Post content
    pub content: Box<[ContentBlock]>,

    /// post layout
    pub layout: Box<[serde_json::Value]>,

    /// The post state
    pub state: Box<str>,

    /// The exact date and time in the past to backdate the post, if desired.
    pub date: Box<str>,

    /// A comma-separated list of tags to associate with the post.
    pub tags: Box<[Box<str>]>,

    /// A custom URL slug to use in the post's permalink URL
    pub slug: Box<str>,

    /// A short url?
    #[serde(rename = "shortUrl")]
    pub short_url: Box<str>,

    /// The post summary.
    pub summary: Box<str>,

    /// The number of notes.
    #[serde(rename = "noteCount")]
    pub note_count: u64,

    /// The number of likes
    #[serde(rename = "likeCount")]
    pub like_count: u64,

    /// Whether this is nsfw
    #[serde(rename = "isNsfw")]
    pub is_nsfw: bool,

    /// The blog name
    #[serde(rename = "blogName")]
    pub blog_name: Box<str>,

    /// The blog?
    pub blog: Blog,
}

/// The blog field.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Blog {
    /// The blog name
    pub name: Box<str>,

    /// Blog avatars
    pub avatar: Box<[Avatar]>,

    /// Blog title
    pub title: Box<str>,

    /// Blog URL
    pub url: Box<str>,

    /// ?
    #[serde(rename = "blogViewUrl")]
    pub blog_view_url: Box<str>,

    /// ?
    #[serde(rename = "isAdult")]
    pub is_adult: bool,

    /// Blog description
    #[serde(rename = "descriptionNpf")]
    pub description_npf: Box<[ContentBlock]>,

    /// ?
    pub uuid: Box<str>,
}

/// A blog avatar
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Avatar {
    /// Avatar width
    pub width: u32,

    /// Avatar height
    pub height: u32,

    /// Avatar url
    pub url: Box<str>,

    /// ?
    pub accessories: Box<[serde_json::Value]>,
}

mod serde_str_u64 {
    use serde::de::Error;
    use std::borrow::Cow;

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        value.parse().map_err(D::Error::custom)
    }

    pub(super) fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(itoa::Buffer::new().format(*value))
    }
}
