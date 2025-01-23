use image::{ImageBuffer, Pixel, Rgb};
use std::{
  fs::{create_dir_all, File},
  io::{Cursor, Write},
  path::Path,
};

pub fn create_frame(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
  img.pixels().flat_map(|p| p.channels().to_vec()).collect()
}

pub fn save_video(
  data: Cursor<Vec<u8>>,
  output_path: String,
  width: i32,
  height: i32,
) -> std::io::Result<()> {
  let path = Path::new(&output_path);

  // Create parent directory if it doesn't exist
  if let Some(parent) = path.parent() {
    if !parent.exists() {
      create_dir_all(parent)?;
    }
  }

  match path.extension() {
    Some(ext) => {
      match ext.to_str() {
        Some("mp4") => {
          let mut mp4muxer = minimp4::Mp4Muxer::new(File::create(path).unwrap());
          mp4muxer.init_video(width, height, false, "title");
          mp4muxer.write_video(&data.into_inner());
          mp4muxer.close();
          println!("MP4 file saved at `output.mp4`.");
          Ok(())
        }
        _ => {
          // Create and write to the file
          let mut file = File::create(&path)?;
          file.write_all(&data.clone().into_inner())?;
          Ok(())
        }
      }
    }
    None => {
      // Create and write to the file
      let mut file = File::create(&path)?;
      file.write_all(&data.into_inner())?;
      Ok(())
    }
  }
}
