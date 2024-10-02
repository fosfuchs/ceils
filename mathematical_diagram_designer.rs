//just run it with the command "cargo r -- "sin(x*0.01)*100""
//for work need image, imageproc, meval

use std::env;
use image::Rgb;

const COUNT_DOTS: usize = 1000;
const IMAGE_SIZE: (u32, u32) = (500, 500);

fn main() {
  // let br = Vec::new();
  let mut image = image::RgbImage::new(IMAGE_SIZE.0+1, IMAGE_SIZE.1+1);

  for x in 1..IMAGE_SIZE.0/10 {
    imageproc::drawing::draw_line_segment_mut(&mut image, ((x*10) as f32, IMAGE_SIZE.1 as f32), ((x*10) as f32, 0.), Rgb([55u8, 55u8, 55u8]));
  }
  for y in 1..IMAGE_SIZE.1/10 {
    imageproc::drawing::draw_line_segment_mut(&mut image, (IMAGE_SIZE.0 as f32, (y*10) as f32), (0., (y*10) as f32), Rgb([55u8, 55u8, 55u8]));
  }
  for x in 1..IMAGE_SIZE.0/50 {
    imageproc::drawing::draw_line_segment_mut(&mut image, ((x*50) as f32, IMAGE_SIZE.1 as f32), ((x*50) as f32, 0.), Rgb([255u8, 255u8, 255u8]));
  }
  for y in 1..IMAGE_SIZE.1/50 {
    imageproc::drawing::draw_line_segment_mut(&mut image, (IMAGE_SIZE.0 as f32, (y*50) as f32), (0., (y*50) as f32), Rgb([255u8, 255u8, 255u8]));
  }
  
  for argument in env::args().skip(1) {
    let fun = argument.parse::<meval::Expr>().unwrap().bind("x").unwrap();
    let mut last_point_y = 0.;

    for i in 2..COUNT_DOTS {
      let point_y = fun((IMAGE_SIZE.0 as f64/COUNT_DOTS as f64)*i as f64-(IMAGE_SIZE.0/2) as f64) as f32;
      println!("{:^5}:{:^5}", ((IMAGE_SIZE.0) as f32/COUNT_DOTS as f32)*(i-1) as f32, ((IMAGE_SIZE.0) as f32/COUNT_DOTS as f32)*i as f32);
      imageproc::drawing::draw_line_segment_mut(&mut image, (((IMAGE_SIZE.0) as f32/COUNT_DOTS as f32)*(i-1) as f32, last_point_y+(IMAGE_SIZE.0/2) as f32), (((IMAGE_SIZE.0) as f32/COUNT_DOTS as f32)*i as f32, point_y+(IMAGE_SIZE.0/2) as f32), Rgb([0u8, 0u8, 255u8]));
      last_point_y=point_y;
    }
  }
  println!("done");
  image::imageops::flip_vertical_in_place(&mut image);
  image.save_with_format("out.png", image::ImageFormat::Png).unwrap();
}
