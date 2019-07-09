extern crate image;
use image::{GenericImage, GenericImageView, Rgba, DynamicImage, ImageBuffer, ImageRgba8};
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::prelude::*;
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use imageproc::distance_transform::Norm;
use rusttype::{FontCollection, Scale};
use crate::{PhotonImage, helpers, Rgb};

/// Add text to an image.
#[wasm_bindgen]
pub fn create_image(width: u32, height: u32, background_color: Rgb) -> PhotonImage {
    // create a pixel 
    let pixel =  image::Rgba([background_color.r, background_color.g, background_color.b, 100]);
    let image_buffer = ImageBuffer::from_pixel(width, height, pixel);
    let rgb_img = image::ImageRgba8(image_buffer);

    let raw_pixels = rgb_img.raw_pixels();
    return PhotonImage { raw_pixels: raw_pixels, width: width, height: height};
}

/// Draw a PhotonImage onto a container image.
#[wasm_bindgen]
pub fn draw_photonimage(mut container_img: &mut PhotonImage, photon_img: &PhotonImage, x_pos: u32, y_pos: u32) {
    let mut dyn_container_img = helpers::dyn_image_from_raw(&container_img);
    let dyn_photonimg = helpers::dyn_image_from_raw(&photon_img);

    image::imageops::overlay(&mut dyn_container_img, &dyn_photonimg, x_pos, y_pos);
    let raw_pixels = dyn_container_img.raw_pixels();
    container_img.raw_pixels = raw_pixels;
}
