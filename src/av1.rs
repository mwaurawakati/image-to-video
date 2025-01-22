use super::models::{Config as cConfig, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use image::open;
use image::DynamicImage;
use napi::bindgen_prelude::*;
use rav1e::{config::SpeedSettings, *};

pub fn images_to_av1_video(config: &cConfig) -> Result<Vec<u8>> {
  let mut frames1: Vec<DynamicImage> = vec![];
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
  let mut output = Vec::new();
  //let mut frames = vec![];
  for image in &config.images {
    let mut im = open(image).map_err(|e| {
      napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to open image: {}", e),
      )
    })?;
    im = im.resize_exact(width, height, image::imageops::FilterType::CatmullRom);
    frames1.push(im);
  }
  for (i, image) in frames1.iter().enumerate() {
    // Resize the image to match the encoder dimensions if necessary
    let resized_image = image.resize_exact(
      width as u32,
      height as u32,
      image::imageops::FilterType::Triangle,
    );

    // Extract pixel data
    let pixels = resized_image.to_rgb8();
    let _stride = pixels.width() as usize * 3;

    // Create a new frame
    let mut frame = ctx.new_frame();
    for (_plane_idx, plane) in frame.planes.iter_mut().enumerate() {
      let xdec = plane.cfg.xdec as usize;
      let stride = (width + xdec as u32) >> xdec;
      plane.copy_from_raw_u8(&pixels, stride as usize, 1);
    }

    println!("Sending frame {}", i);
    match ctx.send_frame(frame) {
      Ok(_) => {}
      Err(EncoderStatus::EnoughData) => {
        println!("Frame {} dropped due to queue limit", i);
      }
      Err(e) => {
        return Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Error sending frame {}: {:?}", i, e),
        ))
      }
    }
  }

  // Flush any remaining frames
  ctx.flush();

  // Collect encoded packets
  while let Ok(packet) = ctx.receive_packet() {
    println!("Packet {}", packet.input_frameno);
    output.extend_from_slice(&packet.data);
  }

  Ok(output)
}
