mod commands;

#[derive(Debug, argh::FromArgs)]
#[argh(description = " a CLI for tumblr")]
struct Options {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, argh::FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Download(self::commands::download::Options),
}

fn main() -> anyhow::Result<()> {
    let options: Options = argh::from_env();

    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    tokio_rt.block_on(async_main(options))?;

    Ok(())
}

async fn async_main(options: Options) -> anyhow::Result<()> {
    let client = tumblr::Client::new();

    match options.subcommand {
        Subcommand::Download(options) => {
            self::commands::download::exec(client, options).await?;
        }
    }
    Ok(())
}
