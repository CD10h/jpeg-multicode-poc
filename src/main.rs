use rand::prelude::*;
use std::fs;
use std::path::Path;

use indicatif::{ParallelProgressIterator, ProgressIterator};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSlice;
use turbojpeg::Compressor;
use turbojpeg::{Decompressor, Image, PixelFormat};

fn jpegize_data(
    jpeg_file: Vec<u8>,
    iterations: usize,
    quality: i32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // get the JPEG data
    let mut jpeg_data = jpeg_file;

    // initialize a Decompressor
    let mut decompressor = Decompressor::new()?;
    let mut compressor = Compressor::new()?;
    compressor.set_quality(quality);
    // read the JPEG header with image size
    let header = decompressor.read_header(&jpeg_data)?;
    let (width, height) = (header.width, header.height);

    let mut image = Image {
        pixels: vec![0; 3 * width * height],
        width: width,
        pitch: 3 * width, // we use no padding between rows
        height: height,
        format: PixelFormat::RGB,
    };

    for _ in 0..iterations {
        // decompress the JPEG data
        decompressor.decompress(&jpeg_data, image.as_deref_mut())?;

        // compress the Image to a Vec<u8> of JPEG data
        jpeg_data = compressor.compress_to_vec(image.as_deref())?;
    }
    Ok(jpeg_data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let in_dir = "./wizout/";
    let out_dir = "./wizz/";

    let files = fs::read_dir(in_dir)?
        .map(|path| path.unwrap())
        .collect_vec();
    files
        .par_chunks(32)
        .progress_count(files.len() as u64 / 32)
        .for_each(|chunk| {
            chunk.iter().for_each(|path| {
                // let mut rng = rand::thread_rng();
                // let iters = rng.gen_range(300..2000);
                // let quality = rng.gen_range(20..25);
                let iters = 5000;
                let quality = 10;

                // if iters > 1000 {
                //     quality = (quality as f64 * 1.5 ) as i32;
                //     quality = i32::clamp(quality, 1, 100)
                // }

                let filename = path.file_name().into_string().unwrap();

                let out_path = format!("{}{}", out_dir, filename);

                let jpeg_file = std::fs::read(path.path()).unwrap();
                let jpegized_file = jpegize_data(jpeg_file, iters, quality).unwrap();
                std::fs::write(out_path, jpegized_file).unwrap();
            });
        });

    Ok(())
}
