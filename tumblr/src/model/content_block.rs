use std::collections::HashMap;

/// Post content
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text {
        /// Text
        text: Box<str>,

        /// Subtype
        subtype: Option<Box<str>>,

        /// Indent level
        indent_level: Option<u16>,

        /// ?
        formatting: Option<Box<[Formatting]>>,
    },
    #[serde(rename = "image")]
    Image {
        /// Media
        media: Box<[Media]>,

        /// Alt text
        alt_text: Option<Box<str>>,

        /// Caption
        caption: Option<Box<str>>,

        /// Exif
        exif: Option<HashMap<Box<str>, Box<str>>>,
    },
    #[serde(rename = "video")]
    Video {
        /// The URL to use for the video block, if no media is present.
        url: Option<Box<str>>,

        /// The Media Object to use for the video block, if no url is present.
        media: Option<Media>,

        /// The provider of the video, whether it's tumblr for native video or a trusted third party.
        provider: Option<Box<str>>,

        /// HTML code that could be used to embed this video into a webpage.
        embed_html: Option<Box<str>>,

        /// An embed iframe object used for constructing video iframes.
        embed_iframe: Option<serde_json::Value>,

        /// A URL to the embeddable content to use as an iframe.
        embed_url: Option<Box<str>>,

        /// An image media object to use as a "poster" for the video, usually a single frame.
        poster: Option<Box<[Media]>>,

        /// Optional provider-specific metadata about the video.
        metadata: Option<serde_json::Value>,

        /// Optional attribution information about where the video came from.
        attribution: Option<serde_json::Value>,

        /// Whether this video can be played on a cellular connection.
        can_autoplay_on_cellular: Option<bool>,

        /// ?
        filmstrip: Option<Media>,
    },
}

/// A formatting object
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
pub enum Formatting {
    /// Bold text
    #[serde(rename = "bold")]
    Bold {
        /// The start of the range
        start: u64,

        /// The end of the range
        end: u64,
    },

    /// Italic text
    #[serde(rename = "italic")]
    Italic {
        /// The start of the range
        start: u64,

        /// The end of the range
        end: u64,
    },

    /// Strikethrough text
    #[serde(rename = "strikethrough")]
    Strikethrough {
        /// The start of the range
        start: u64,

        /// The end of the range
        end: u64,
    },

    /// A link
    #[serde(rename = "link")]
    Link {
        /// The start of the range
        start: u64,

        /// The end of the range
        end: u64,

        /// The url
        url: Option<Box<str>>,
    },

    /// A mention
    #[serde(rename = "mention")]
    Mention {
        /// The start of the range
        start: u64,

        /// The end of the range
        end: u64,

        /// The blog info
        blog: BlogInfo,
    },

    /// Color
    #[serde(rename = "color")]
    Color {
        /// The start of the range
        start: u64,

        /// The end of the range
        end: u64,

        /// The hex color to use
        hex: Box<str>,
    },

    /// Small
    #[serde(rename = "small")]
    Small {
        /// The start of the range
        start: u64,

        /// The end of the range
        end: u64,
    },
}

/// Blog info
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BlogInfo {
    /// ?
    pub uuid: Box<str>,

    /// The name of the blog?
    pub name: Box<str>,

    /// The URL
    pub url: Box<str>,
}

/// Media object
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Media {
    /// A url
    pub url: Box<str>,

    /// The mime type
    #[serde(rename = "type")]
    pub kind: Option<Box<str>>,

    /// The image width
    pub width: Option<u32>,

    /// The image height
    pub height: Option<u32>,

    /// For display purposes, this indicates whether the dimensions are defaults
    pub original_dimensions_missing: Option<bool>,

    /// This indicates whether this media object is a cropped version of the original media
    pub cropped: Option<bool>,

    /// This indicates whether this media object has the same dimensions as the original media.
    #[serde(rename = "hasOriginalDimensions")]
    pub has_original_dimensions: Option<bool>,
}
