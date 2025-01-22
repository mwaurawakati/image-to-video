use image::{ImageBuffer, Pixel, Rgb};
use mp4::Mp4Config;
use std::{
  fs::{create_dir_all, File},
  io::{Cursor, Write},
  path::Path,
};

pub fn create_frame(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
  img.pixels().flat_map(|p| p.channels().to_vec()).collect()
}

pub fn save_video(data: Cursor<Vec<u8>>, output_path: String) -> std::io::Result<()> {
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
          let mut file = File::create(path).unwrap();
          let config = Mp4Config {
            major_brand: str::parse("isom").unwrap(),
            minor_version: 512,
            compatible_brands: vec![
              str::parse("isom").unwrap(),
              str::parse("iso2").unwrap(),
              str::parse("avc1").unwrap(),
              str::parse("mp41").unwrap(),
            ],
            timescale: 1000,
          };

          //let writer = Mp4Writer::write_start(&mut mp4_file, &config).unwrap();

          let mut writer = mp4::Mp4Writer::write_start(data, &config).map_err(|e| {
            std::io::Error::new(
              std::io::ErrorKind::Other,
              format!("failed to start writing mp4 with error: {:?}", e),
            )
          })?;
          /*let track_config = mp4::TrackConfig{
            track_type: mp4::TrackType::Video,
            timescale: 60, // FPS
            language: "und".to_string(),
            media_conf:mp4::AvcConfig{
                Width: DEFAULT_WIDTH,
                height: DEFAULT_HEIGHT
            }
          }
          writer.add_track(&track_config);*/
          writer.write_end().map_err(|e| {
            std::io::Error::new(
              std::io::ErrorKind::Other,
              format!("failed to finish writing mp4 with error: {:?}", e),
            )
          })?;
          let data: Vec<u8> = writer.into_writer().into_inner();
          file.write_all(&data)?;
          println!("MP4 file saved at `output.mp4`.");
          Ok(())
        }
        _ => {
          // Create and write to the file
          let mut file = File::create(&path)?;
          file.write_all(&data.into_inner())?;
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
