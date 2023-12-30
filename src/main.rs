//import modules
mod vec3;


// use std::io;
use image::{Rgb, RgbImage , ImageBuffer};
use vec3::Vec3;






fn main() {
    //Image dims
    const IMAGE_HEIGHT:u32 = 256;
    const IMAGE_WIDTH:u32 = 256;

    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

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

    for (x,  y, pixel) in buffer.enumerate_pixels_mut(){
        let r = x as f64 / (IMAGE_WIDTH-1) as f64;
        let g = y as f64 / (IMAGE_HEIGHT-1) as f64;
        let b = 0.0;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }

   match buffer.save("image.png") {
    Err(e) => eprintln!("Error writing file: {}", e),
    Ok(()) => println!("Done")

    }

    let vec1:Vec3 = Vec3::new(6.3 ,8.3 ,6.4);
    let vec2:Vec3 = Vec3::new(2.3 ,4.3 ,6.4);

    let vec3 = vec1+vec2;

    println!("vector sum: {:?}", vec3);

    
}
