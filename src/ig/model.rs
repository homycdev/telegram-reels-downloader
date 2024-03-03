use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct VideoUrl {
    video_url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ShortCodeMedia {
    shortcode_media: VideoUrl,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct IGResponse {
    graphql: ShortCodeMedia,
}

impl IGResponse {
    pub fn get_download_url(self) -> String {
        self.graphql.shortcode_media.video_url
    }
}

pub struct ReelUrl(pub String);

impl ReelUrl {
    pub fn new(reel_url: String) -> ReelUrl {
        let replaced_spaces = reel_url.to_string().replace(" ", "");
        ReelUrl(replaced_spaces + "/?__a=1")
    }
}
