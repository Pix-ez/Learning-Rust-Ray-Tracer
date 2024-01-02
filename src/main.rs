//import modules
mod vec3;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod material;
// use std::io;

use raylib::prelude::*;
use image::{Rgba, ImageBuffer, RgbaImage};
use vec3::Vec3;
use ray::Ray;
use hittable::Hittable;
use hittable_list::HittableList;
use sphere::Sphere;



use rand::prelude::*;
use rayon::prelude::*;
use std::time;


fn color(r:&Ray) -> Vec3{
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.8*(unit_direction.y() + 1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0)*t
}

fn render(image_height: &u32,image_width: &u32 )-> ImageBuffer<Rgba<u8>, Vec<u8>>{
    let mut buffer: RgbaImage = ImageBuffer::new(*image_width, *image_height);

    println!("{} ,{}", image_height ,image_width);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for (x,  y, pixel) in buffer.enumerate_pixels_mut(){
        
        let u = x as f32 / (image_width-1) as f32;
        let v = y as f32 / (image_height-1) as f32;

        let r = Ray::ray(origin, lower_left_corner + horizontal * u +  vertical * v);

        let col = color(&r);
        

        let ir = (255.999 * col.r()) as u8;
        let ig = (255.999 * col.g()) as u8;
        let ib = (255.999 * col.b()) as u8;

        *pixel = Rgba([ir, ig, ib, 255]);
    }

    match buffer.save("image.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(()) => println!("Done")
    }

    return  buffer;



}

fn main() {
    //Image dims
    const IMAGE_HEIGHT:u32 = 720;
    const IMAGE_WIDTH:u32 = 1280;

    

    // println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    // for j in (0..IMAGE_HEIGHT).rev(){
    //     eprintln!("Scanlines remaining: {}", j);
    //     for i in 0..IMAGE_WIDTH{
    //         let r = i as f64 / (IMAGE_WIDTH-1) as f64;
    //         let g = j as f64 / (IMAGE_HEIGHT-1) as f64;
    //         let b = 0.25;

    //         let ir = (255.99 * r) as u8;
    //         let ig = (255.99 * g) as u8;
    //         let ib = (255.99 * b) as u8;

    //         println!("{} {} {}", ir, ig, ib);
    //     }
    // }


    
        let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Ray Tracer in Rust")
        .vsync()
        .build();



        // Convert RGBA image buffer to Raylib Image
    let frame  = render(&IMAGE_HEIGHT, &IMAGE_WIDTH);
    let image = rl.load_texture(&thread, "image.png").unwrap();
    // let buf:Vec<u8> = frame.iter().flat_map(|rgb| rgb.data.iter()).cloned().collect();

    // let mut image = Image::load_image_from_mem(filetype, &buf, size);
    // let texture = rl.load_texture_from_image(&thread, &frame)

        while !rl.window_should_close() {
                    let mut d = rl.begin_drawing(&thread);
            
                    
                    d.clear_background(Color::BLACK);
                    // d.draw_text("Ray Tracer in Rust", 12, 12, 20, Color::WHITE);
                    d.draw_texture(&image, 40, 40, Color::WHITE);
                   
                    d.draw_fps(2, 3)
                }
  

    

    let vec1:Vec3 = Vec3::new(6.3 ,8.3 ,6.4);
    let vec2:Vec3 = Vec3::new(2.3 ,4.3 ,6.4);

    // let vec3 = vec1+vec2;

    // println!("vector sum: {:?}", vec3);
    // println!("  {:?}", frame);
    

    
}



// fn main() {


//    
// }