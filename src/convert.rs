use super::{
  av1::images_to_av1_video,
  h264::images_to_h264_video,
  models::{Config, VideoEncoding, DEFAULT_FPS},
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn convert_images_to_video(config: &Config) -> Result<Vec<u8>> {
  if config.images.is_empty() {
    return Err(napi::Error::new(
      napi::Status::InvalidArg,
      String::from("No images were provided."),
    ));
  }
  let fps = config.fps.unwrap_or(DEFAULT_FPS);
  if fps < 1 || fps > 60 {
    return Err(napi::Error::new(
      napi::Status::InvalidArg,
      String::from("fps should be between 1 and 60."),
    ));
  }

  match config.video_encoding.unwrap_or(VideoEncoding::H264) {
    VideoEncoding::H264 => return images_to_h264_video(config),
    VideoEncoding::AV1 => return images_to_av1_video(config),
  }
}
