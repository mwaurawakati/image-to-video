import { Config, convertImagesToVideo, VideoEncoding } from './index.js'

let conf = Config.defaultConfigWithImages([
  '/Users/mwaurawakati/Downloads/bolt.png',
  '/Users/mwaurawakati/Downloads/google.png',
  '/Users/mwaurawakati/Downloads/apple.png',
  '/Users/mwaurawakati/Downloads/bolt.png',
  '/Users/mwaurawakati/Downloads/google.png',
  '/Users/mwaurawakati/Downloads/apple.png',
  '/Users/mwaurawakati/Downloads/bolt.png',
  '/Users/mwaurawakati/Downloads/google.png',
  '/Users/mwaurawakati/Downloads/apple.png',
  '/Users/mwaurawakati/Downloads/bolt.png',
  '/Users/mwaurawakati/Downloads/google.png',
  '/Users/mwaurawakati/Downloads/apple.png',
  '/Users/mwaurawakati/Downloads/bolt.png',
  '/Users/mwaurawakati/Downloads/google.png',
  '/Users/mwaurawakati/Downloads/apple.png',
  '/Users/mwaurawakati/Downloads/bolt.png',
  '/Users/mwaurawakati/Downloads/google.png',
  '/Users/mwaurawakati/Downloads/apple.png',
  '/Users/mwaurawakati/Downloads/bolt.png',
  '/Users/mwaurawakati/Downloads/google.png',
  '/Users/mwaurawakati/Downloads/apple.png',
])
conf.outputPath = '/Users/mwaurawakati/Desktop/t.mp4'
//conf.videoEncoding = VideoEncoding.AV1
try {
  let data = convertImagesToVideo(conf)
  console.log(data)
} catch (e) {
  console.log(e)
}
//console.log(conf)
