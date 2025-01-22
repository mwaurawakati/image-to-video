#![deny(clippy::all)]

use image::open; // To load images
use x264::Encoder; // For H.264 encoding
use napi::bindgen_prelude::*; // For Node.js bindings
use std::fs::File;
use std::io::Write;


#[macro_use]
use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn convert_images_to_video(
    images: Vec<String>,
    fps: Option<u32>,
    resolution: Option<(u32, u32)>,
) -> Result<Vec<u8>> {
    // Load and validate images
    let frames = images
        .iter()
        .map(|path| open(path).map_err(|e| format!("Failed to open image: {}", e)))
        .collect::<Result<Vec<_>, _>>()?;

    // Configure encoder
    let mut encoder = Encoder::new(/* ... configuration based on FPS, resolution ... */)?;

    // Encode frames into video
    let mut video_buffer = Vec::new();
    for frame in frames {
        // Convert each image into a frame and encode
        encoder.encode_frame(&frame.to_rgba8(), &mut video_buffer)?;
    }

    // Finalize encoding
    encoder.finish(&mut video_buffer)?;

    Ok(video_buffer)
}


pub fn convert_images_to_video2(
    images: Vec<String>,
    fps: Option<u32>,
    resolution: Option<(u32, u32)>,
    output_path: Option<String>, // Add optional output path parameter
) -> Result<Vec<u8>, String> {
    // Load and validate images
    let frames = images
        .iter()
        .map(|path| open(path).map_err(|e| format!("Failed to open image: {}", e)))
        .collect::<Result<Vec<_>, _>>()?;

    // Configure encoder
    let mut encoder = Encoder::new(/* ... configuration based on FPS, resolution ... */)
        .map_err(|e| format!("Failed to configure encoder: {}", e))?;

    // Encode frames into video buffer
    let mut video_buffer = Vec::new();
    for frame in frames {
        encoder
            .encode_frame(&frame.to_rgba8(), &mut video_buffer)
            .map_err(|e| format!("Failed to encode frame: {}", e))?;
    }

    // Finalize encoding
    encoder
        .finish(&mut video_buffer)
        .map_err(|e| format!("Failed to finalize encoding: {}", e))?;

    // Save the buffer to a file if output_path is provided
    if let Some(path) = output_path {
        let mut file = File::create(&path).map_err(|e| format!("Failed to create file: {}", e))?;
        file.write_all(&video_buffer)
            .map_err(|e| format!("Failed to write video to file: {}", e))?;
        println!("Video saved to {}", path);
    }

    Ok(video_buffer)
}
