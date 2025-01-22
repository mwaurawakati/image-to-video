import { Config, convertImagesToVideo } from './index.js'

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
conf.outputPath = '/Users/mwaurawakati/Desktop/t.h264'
try {
  convertImagesToVideo(conf)
} catch (e) {
  console.log(e)
}
//console.log(conf)
