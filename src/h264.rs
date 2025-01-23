use super::{
  models::{Colorspace, Config, DEFAULT_FPS, DEFAULT_HEIGHT, DEFAULT_WIDTH},
  utils::create_frame,
};
use crate::utils::save_video;
use image::open;
use napi::bindgen_prelude::*;
use std::io::{Cursor, Write};
use x264::{Encoder, Image};

pub fn images_to_h264_video(config: &Config) -> Result<Vec<u8>> {
  // Catch potential panics and wrap them in meaningful errors
  let result = std::panic::catch_unwind(|| {
    let mut frames = vec![];
    let width = config.width.unwrap_or(DEFAULT_WIDTH);
    let height = config.height.unwrap_or(DEFAULT_HEIGHT);
    let fps = config.fps.unwrap_or(DEFAULT_FPS);

    let colorspace = match config.colorspace {
      Some(c) => match c {
        Colorspace::I420 => x264::Colorspace::I420,
        Colorspace::YV12 => x264::Colorspace::YV12,
        Colorspace::NV12 => x264::Colorspace::NV12,
        Colorspace::NV21 => x264::Colorspace::NV21,
        Colorspace::I422 => x264::Colorspace::I422,
        Colorspace::YV16 => x264::Colorspace::YV16,
        Colorspace::NV16 => x264::Colorspace::NV16,
        Colorspace::YUYV => x264::Colorspace::YUYV,
        Colorspace::UYVY => x264::Colorspace::UYVY,
        Colorspace::V210 => x264::Colorspace::V210,
        Colorspace::I444 => x264::Colorspace::I444,
        Colorspace::YV24 => x264::Colorspace::YV24,
        Colorspace::BGR => x264::Colorspace::BGR,
        Colorspace::BGRA => x264::Colorspace::BGRA,
        Colorspace::RGB => x264::Colorspace::RGB,
      },
      None => x264::Colorspace::RGB,
    };

    let frames_per_image = config.fpi.unwrap_or(3) * fps; 

    for image in &config.images {
      let im = open(image).map_err(|e| {
        napi::Error::new(
          napi::Status::InvalidArg,
          format!("Failed to open image: {}", e),
        )
      })?;
      let im1 = im.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
      frames.push(im1.to_rgb8());
    }

    // Configure encoder
    let mut encoder = Encoder::builder()
      .fps(fps, 1)
      .build(colorspace, width as _, height as _)
      .map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to open encoder: {:?}", e),
        )
      })?;

    let mut buffer = Cursor::new(Vec::new());

    {
      let headers = encoder.headers().map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to create headers: {:?}", e),
        )
      })?;
      buffer.write_all(headers.entirety()).map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to write headers: {:?}", e),
        )
      })?;
    }

    // Encode frames
    for (i, frame) in frames.iter().enumerate() {
      let frame_data = create_frame(&frame.clone());
      for _j in 0..frames_per_image {
        let image = Image::rgb(width as _, height as _, &frame_data);
        let (data, _) = encoder
          .encode((fps as i64 * i as i64) as _, image)
          .map_err(|e| {
            napi::Error::new(
              napi::Status::GenericFailure,
              format!("Failed to encode frame: {:?}", e),
            )
          })?;
        buffer.write_all(data.entirety()).map_err(|e| {
          napi::Error::new(
            napi::Status::GenericFailure,
            format!("Failed to write frame: {:?}", e),
          )
        })?;
      }
    }

    // Finalize encoding
    {
      let mut flush = encoder.flush();
      while let Some(result) = flush.next() {
        let (data, _) = result.map_err(|e| {
          napi::Error::new(
            napi::Status::GenericFailure,
            format!("Failed to flush task: {:?}", e),
          )
        })?;
        buffer.write_all(data.entirety()).map_err(|e| {
          napi::Error::new(
            napi::Status::GenericFailure,
            format!("Failed to write flush: {:?}", e),
          )
        })?;
      }
    }

    // Save output to file
    if let Some(output_path) = &config.output_path {
      save_video(
        buffer.clone(),
        output_path.to_string(),
        width as i32,
        height as i32,
      )
      .map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to save video with error: {}", e),
        )
      })?;
    }

    Ok(buffer.into_inner())
  });

  match result {
    Ok(res) => res,
    Err(_) => Err(napi::Error::new(
      napi::Status::GenericFailure,
      "A panic occurred during processing.".to_string(),
    )),
  }
}
