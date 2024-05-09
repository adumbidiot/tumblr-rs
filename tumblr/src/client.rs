use crate::Error;
use crate::InitialState;
use crate::ScrapedPost;
use crate::TimelineObject;
use once_cell::sync::Lazy;
use scraper::Html;
use scraper::Selector;
use std::sync::Arc;

const USER_AGENT_STR: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

/// A client
#[derive(Debug, Clone)]
pub struct Client {
    /// The inner http client
    pub client: reqwest::Client,

    /// Client state
    state: Arc<ClientState>,
}

impl Client {
    /// Make a new client.
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT_STR)
            .build()
            .unwrap();
        Self {
            client,
            state: Arc::new(ClientState {
                api_token: std::sync::Mutex::new(None),
            }),
        }
    }

    /// Send a GET request to a page and extract the ___INITIAL_STATE___ value.
    async fn get_initial_state(&self, url: &str) -> Result<InitialState, Error> {
        static INITIAL_STATE_SELECTOR: Lazy<Selector> =
            Lazy::new(|| Selector::parse("#___INITIAL_STATE___").unwrap());

        let text = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let initial_state = tokio::task::spawn_blocking(move || {
            let html = Html::parse_document(text.as_str());
            let initial_state = html
                .select(&INITIAL_STATE_SELECTOR)
                .next()
                .and_then(|el| el.text().next())
                .ok_or(Error::MissingInitialState)?;
            let initial_state: InitialState = serde_json::from_str(initial_state)?;

            Ok::<_, Error>(initial_state)
        })
        .await??;

        Ok(initial_state)
    }

    /// Scrape an api token.
    pub async fn scrape_api_token(&self) -> Result<(), Error> {
        let initial_state = self.get_initial_state("https://www.tumblr.com/").await?;

        *self
            .state
            .api_token
            .lock()
            .unwrap_or_else(|error| error.into_inner()) =
            Some(initial_state.api_fetch_store.api_token.into());

        Ok(())
    }

    /// Scrape a post
    pub async fn scrape_post(
        &self,
        blog_identifier: &str,
        post_id: u64,
    ) -> Result<ScrapedPost, Error> {
        let url = format!("https://www.tumblr.com/{blog_identifier}/{post_id}");
        let initial_state = self.get_initial_state(url.as_str()).await?;
        let mut objects = initial_state
            .peepr_route
            .map(|peepr_route| Vec::from(peepr_route.initial_timeline.objects))
            .ok_or(Error::MissingScrapedPost)?;
        objects.retain(|obj| match obj {
            TimelineObject::Post(post) => *post.blog_name == *blog_identifier && post.id == post_id,
        });
        if objects.is_empty() {
            return Err(Error::MissingScrapedPost);
        }

        // Will add more types later
        #[allow(clippy::infallible_destructuring_match)]
        let post = match objects.swap_remove(0) {
            TimelineObject::Post(post) => post,
        };

        Ok(post)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct ClientState {
    api_token: std::sync::Mutex<Option<Arc<str>>>,
}
