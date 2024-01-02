//import modules
mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;
// use std::io;



use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use image::{Rgba, ImageBuffer, RgbaImage};
use material::{scatter, Material};
use ray::Ray;
use raylib::prelude::*;
use sphere::Sphere;
use vec3::Vec3;



use rand::prelude::*;
use rayon::prelude::*;
use std::time;




fn color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.001, std::f32::MAX) {
        let mut scattered = Ray::ray(Vec3::default(), Vec3::default());
        let mut attentuation = Vec3::default();

        if depth < 50 && scatter(&rec.material, r, &rec, &mut attentuation, &mut scattered) {
            return attentuation * color(&scattered, world, depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}



fn render(image_height: &u32, image_width: &u32, num_sample: &i32 )-> ImageBuffer<Rgba<u8>, Vec<u8>>{
    let mut buffer: RgbaImage = ImageBuffer::new(*image_width, *image_height);

    // this is so helpful, https://stackoverflow.com/questions/46965867/rust-borrowed-value-must-be-valid-for-the-static-lifetime
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let mut rng = rand::thread_rng();

    //Setup the scene with objects

    scene.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, 100.0, 0.0),
        100.0,
        Material::Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        },
    )));

    scene.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, -1.0, 0.0),
        1.0,
        Material::Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    )));

    //init world and add the scene into it.
    let world = HittableList::new(scene);

    //setup the camera parameters and add camera.
    let aspect_ratio = *image_width as f32 / *image_height as f32;
    let look_from = Vec3::new(0.0, 0.0, 40.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = 10.0;
    let apeture = 0.1;

    let cam = Camera::camera(
        look_from,
        look_at,
        vup,
        30.0,
        aspect_ratio,
        apeture,
        dist_to_focus,
    );

    

    // the "screen" is just a vector of rgb tuples for each pixel (width * height)
    // let mut screen = vec![(0u32, 0u32, 0u32); *image_width as usize * *image_height as usize];
    // let start = time::Instant::now();

    // println!("{} ,{}", image_height ,image_width);



    for (x,  y, pixel) in buffer.enumerate_pixels_mut(){

        let mut col = Vec3::default();

        for _ in 0..*num_sample{

            let u = (x as f32 + rng.gen::<f32>()) / (image_width-1) as f32;
            let v = (y as f32 + rng.gen::<f32>()) / (image_height-1) as f32;

            //let r = Ray::ray(*camera.origin(), camera.lower_left_corner() + camera.horizontal() * u +  camera.vertical() * v);
            let r = &cam.get_ray(u, v);
            col = col + color(&r, &world, 0)
        }

        col = col / *num_sample as f32;
        col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

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
    const IMAGE_WIDTH:u32 =  1280; //480;
    const IMAGE_HEIGHT:u32 = 720; //480; 
    const NUM_SAMPLE:i32 = 2;

    

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
    let frame  = render(&IMAGE_HEIGHT, &IMAGE_WIDTH , &NUM_SAMPLE);
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
  

    

    // let vec1:Vec3 = Vec3::new(6.3 ,8.3 ,6.4);
    // let vec2:Vec3 = Vec3::new(2.3 ,4.3 ,6.4);

    // let vec3 = vec1+vec2;

    // println!("vector sum: {:?}", vec3);
    // println!("  {:?}", frame);
    

    
}



