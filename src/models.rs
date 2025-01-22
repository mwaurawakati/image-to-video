
use napi_derive::napi;
#[napi]
pub const DEFAULT_WIDTH: u32 = 1920;
#[napi]
pub const DEFAULT_HEIGHT: u32 = 1080;
#[napi]
pub const DEFAULT_FPS: u32 = 60;

// This is configuration to be used for image to video conversion
#[napi(js_name = "Config")]
pub struct Config {
  // Images to be converted.
  pub images: Vec<String>,
  pub fps: Option<u32>,
  pub height: Option<u32>,
  pub width: Option<u32>,
  pub colorspace: Option<Colorspace>,
  pub output_path: Option<String>,
  pub fpi: Option<u32>,
  pub video_encoding: VideoEncoding
}


#[napi]
impl Config {
  #[napi(constructor)]
  pub fn new() -> Self {
    Config {
      images: vec![],
      fps: Some(30),
      height: Some(1920),
      width: Some(1080),
      colorspace: Some(Colorspace::RGB),
      output_path: None,
      fpi: Some(3),
      video_encoding: VideoEncoding::H264,
    }
  }

  #[napi(factory)]
  pub fn create_config(
    images: Vec<String>,
    fps: Option<u32>,
    height: Option<u32>,
    width: Option<u32>,
    colorspace: Option<Colorspace>,
    output_path: Option<String>,
  ) -> Self {
    Config {
      images,
      fps,
      height,
      width,
      colorspace,
      output_path,
      fpi: Some(3),
      video_encoding: VideoEncoding::H264,
    }
  }

  #[napi(factory)]
  pub fn default_config_with_images(images: Vec<String>) -> Self {
    Config {
      images: images,
      fps: Some(30),
      height: Some(1920),
      width: Some(1080),
      colorspace: Some(Colorspace::RGB),
      output_path: None,
      fpi: Some(3),
      video_encoding: VideoEncoding::H264,
    }
  }

  #[napi(setter)]
  pub fn add_images(&mut self, images: Vec<String>) {
    self.images.extend_from_slice(&images)
  }

  #[napi(setter)]
  pub fn add_image(&mut self, image: String) {
    self.images.push(image)
  }

  #[napi(setter)]
  pub fn fps(&mut self, fps: u32) {
    self.fps = Some(fps);
  }

  #[napi(setter)]
  pub fn output_path(&mut self, output_path: String) {
    self.output_path = Some(output_path);
  }

  #[napi(setter)]
  pub fn colorspace(&mut self, cs: Colorspace) {
    self.colorspace = Some(cs);
  }

  #[napi(setter)]
  pub fn resolustion(&mut self, width: u32, height: u32) {
    self.width = Some(width);
    self.height = Some(height);
  }
}

#[napi(object)]
#[derive(Clone)]
pub struct Resolution {
  pub height: u32,
  pub width: u32,
}
#[napi]
#[repr(u32)]
pub enum Colorspace {
  I420 = 2,
  YV12 = 3,
  NV12 = 4,
  NV21 = 5,
  I422 = 6,
  YV16 = 7,
  NV16 = 8,
  YUYV = 9,
  UYVY = 10,
  V210 = 11,
  I444 = 12,
  YV24 = 13,
  BGR = 14,
  BGRA = 15,
  RGB = 16,
}

#[napi]
#[repr(u32)]
pub enum VideoEncoding{
  AV1 = 1,
  H264 = 2
}
