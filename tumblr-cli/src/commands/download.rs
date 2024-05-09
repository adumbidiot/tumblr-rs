use anyhow::ensure;
use anyhow::Context;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use url::Url;

#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "download", description = "download a post")]
pub struct Options {
    #[argh(positional, description = "the url of the post to download")]
    url: Url,
}

pub async fn exec(client: tumblr::Client, options: Options) -> anyhow::Result<()> {
    let (blog_identifier, post_id) = parse_url(&options.url).context("failed to parse url")?;
    let post = client
        .scrape_post(blog_identifier, post_id)
        .await
        .context("failed to scrape post")?;

    let mut out_dir = PathBuf::from(".");
    out_dir.extend([blog_identifier, itoa::Buffer::new().format(post_id)]);

    tokio::fs::create_dir_all(&out_dir).await?;

    {
        let metadata_path = out_dir.join("metadata.json");
        let tmp_metadata_path = nd_util::with_push_extension(&metadata_path, "tmp");
        let mut metadata_file = tokio::fs::File::create(&tmp_metadata_path).await?;
        let metadata = serde_json::to_string_pretty(&post)?;
        metadata_file.write_all(metadata.as_bytes()).await?;
        metadata_file.flush().await?;
        metadata_file.sync_all().await?;
        tokio::fs::rename(&tmp_metadata_path, metadata_path).await?;
    }

    for content in post.content.iter() {
        match content {
            tumblr::ContentBlock::Text { .. } => {}
            tumblr::ContentBlock::Image { media, .. } => {
                let media_object = media
                    .iter()
                    .find(|obj| obj.has_original_dimensions.unwrap_or(false))
                    .context("failed to find original media object")?;
                ensure!(
                    media
                        .iter()
                        .filter(|obj| obj.has_original_dimensions.unwrap_or(false))
                        .count()
                        == 1
                );

                let url = Url::parse(&media_object.url)?;
                let file_name = url
                    .path_segments()
                    .and_then(|mut path| path.next_back())
                    .context("missing file name")?;

                let out_path = out_dir.join(file_name);

                nd_util::download_to_path(&client.client, url.as_str(), out_path).await?;
            }
            tumblr::ContentBlock::Video { media, url, .. } => {
                let url = media
                    .as_ref()
                    .map(|media| &*media.url)
                    .or(url.as_deref())
                    .context("missing url")?;

                let url = Url::parse(url)?;
                let file_name = url
                    .path_segments()
                    .and_then(|mut path| path.next_back())
                    .context("missing file name")?;

                let out_path = out_dir.join(file_name);

                nd_util::download_to_path(&client.client, url.as_str(), out_path).await?;
            }
        }
    }

    Ok(())
}

fn parse_url(url: &Url) -> anyhow::Result<(&str, u64)> {
    ensure!(url.host_str() == Some("www.tumblr.com"));

    let mut path_iter = url.path_segments().context("missing url path")?;
    let blog_identifier = path_iter.next().context("missing blog identifier")?;
    let post_id = path_iter
        .next()
        .context("missing post id")?
        .parse()
        .context("invalid post id")?;

    Ok((blog_identifier, post_id))
}
