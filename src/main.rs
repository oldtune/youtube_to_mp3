mod err;
mod helpers;
mod models;
mod youtube_explore;
use err::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let video_id = "7OUWGGEphz8";
    let manifest = youtube_explore::YoutubeExplorer::get_manifest(video_id).await?;
    println!("{:?}", manifest);
    //get link
    //let the user choose the quality
    //do some magic
    //save
    Ok(())
}
