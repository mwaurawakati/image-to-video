use std::io::Cursor;
use std::io::Write;

use crate::utils::save_video;

use super::models::{Config as cConfig, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use image::open;
use image::ImageBuffer;
use napi::bindgen_prelude::*;
use rav1e::{config::SpeedSettings, *};

pub fn images_to_av1_video(config: &cConfig) -> Result<Vec<u8>> {
  let width = config.width.unwrap_or(DEFAULT_WIDTH);
  let height = config.height.unwrap_or(DEFAULT_HEIGHT);
  //let fps = config.fps.unwrap_or(DEFAULT_FPS);
  let enc = EncoderConfig {
    width: width as usize,
    height: height as usize,
    speed_settings: SpeedSettings::from_preset(9),
    ..Default::default()
  };

  let cfg = Config::new().with_encoder_config(enc.clone());
  let mut ctx: Context<u8> = cfg.new_context().map_err(|e| {
    napi::Error::new(
      napi::Status::GenericFailure,
      format!("Failed to create context: {:?}", e),
    )
  })?;
  let mut output = Cursor::new(Vec::new());
  let mut frames = vec![];
  for image in &config.images {
    let mut im = open(image).map_err(|e| {
      napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to open image: {}", e),
      )
    })?;
    im = im.resize_exact(width, height, image::imageops::FilterType::CatmullRom);
    frames.push(im.into_rgba8());
  }
  let frames_per_image = config.fpi.unwrap_or(3);
  let frame = ctx.new_frame();
  for (_i, image) in frames.iter().enumerate() {
    let frame = create_frame(frame.clone(), enc.clone(), image);

    for _i in 0..frames_per_image {
      ctx.send_frame(frame.clone()).map_err(|e| {
        napi::Error::new(
          napi::Status::InvalidArg,
          format!("Failed to send frame: {}", e),
        )
      })?;
    }
  }

  // Flush any remaining frames
  ctx.flush();

  // Collect encoded packets
  loop {
    match ctx.receive_packet() {
      Ok(pkt) => {
        
        output.write_all(&pkt.data).unwrap();
      }
      Err(e) => match e {
        EncoderStatus::LimitReached => {
          break;
        }
        EncoderStatus::Encoded => println!("  Encoded"),
        EncoderStatus::NeedMoreData => println!("  Need more data"),
        _ => {
          println!("Unable to receive packet ");
          return Err(napi::Error::new(
            napi::Status::InvalidArg,
            format!("Failed to read packet {:?}", e),
          ))
        }
      },
    }
  }
  // Save output to file
  if let Some(output_path) = &config.output_path {
    save_video(
      output.clone(),
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
  Ok(output.into_inner())
}

fn create_frame(
  mut f: Frame<u8>,
  encoder_config: EncoderConfig,
  img_rgba: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
) -> Frame<u8> {
  for (i, plane) in f.planes.iter_mut().enumerate() {
    let stride = (encoder_config.clone().width + plane.cfg.xdec) >> plane.cfg.xdec;
    let plane_data = match i {
      0 => img_rgba.pixels().map(|p| p[0]).collect::<Vec<_>>(), // Y plane
      1 => img_rgba.pixels().map(|p| p[1]).collect::<Vec<_>>(), // U plane
      2 => img_rgba.pixels().map(|p| p[2]).collect::<Vec<_>>(), // V plane
      _ => panic!("Unexpected plane index"),
    };
    plane.copy_from_raw_u8(&plane_data, stride, 1);
  }
  f
}
