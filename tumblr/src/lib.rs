mod client;
mod model;

pub use self::model::ContentBlock;
use self::model::InitialState;
use self::model::ScrapedPost;
use self::model::TimelineObject;
pub use crate::client::Client;

/// Library error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Reqwest http error
    #[error("http error")]
    Reqwest(#[from] reqwest::Error),

    /// Tokio join error
    #[error("tokio join error")]
    TokioJoin(#[from] tokio::task::JoinError),

    /// Json error
    #[error("json error")]
    Json(#[from] serde_json::Error),

    /// Missing ___INITIAL_STATE___ element
    #[error("missing ___INITIAL_STATE___ element")]
    MissingInitialState,

    /// Missing api token
    #[error("missing api token")]
    MissingApiToken,

    /// Missing scraped post
    #[error("missing scraped post")]
    MissingScrapedPost,
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test() {
        let client = Client::new();
        client
            .scrape_api_token()
            .await
            .expect("failed to scrape api token");

        // https://www.tumblr.com/toss-a-coin-to-your-stan-account/746546342405537792
        client
            .scrape_post("toss-a-coin-to-your-stan-account", 746546342405537792)
            .await
            .expect("failed to get post");

        // https://www.tumblr.com/justcatposts/748474744137007104
        client
            .scrape_post("justcatposts", 748474744137007104)
            .await
            .expect("failed to get post");
    }
}
