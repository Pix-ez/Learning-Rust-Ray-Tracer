use std::clone;

use image::{ImageBuffer, Rgba};
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator, IndexedParallelIterator};

use super::{
    camera::Camera,
    hittable::Hittable,
    hittable_list::HittableList,
    material::{scatter, Material},
    ray::Ray,
    sphere::Sphere,
    vec3::Vec3,
};

pub fn color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
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
        //Vec3::new(0.5, 0.7, 1.0)
    }
}

pub fn render_scene(
    image_height: u32,
    image_width: u32,
    num_sample: i32,
    cam: Camera,
    // scene: &Vec<Box<dyn Hittable>>,
    // world : HittableList,
    save: bool,
) -> Vec<u8> {
    // println!("render call");
    

    //  helpful information - https://stackoverflow.com/questions/46965867/rust-borrowed-value-must-be-valid-for-the-static-lifetime
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    // // let mut rng = rand::thread_rng();

    //Setup the scene with objects
    

    // scene.push(Box::new(Sphere::sphere(
    //     Vec3::new(0.0, 100.0, 0.0),
    //     100.0,
    //     Material::Lambertian {
    //         albedo: Vec3::new(0.5, 0.5, 0.5),
    //     },
    // )));

    // scene.push(Box::new(Sphere::sphere(
    //     Vec3::new(0.0, -3.0, 0.0),
    //     3.0,
    //     Material::Lambertian {
    //         albedo: Vec3::new(0.8, 0.2, 0.2),
    //     },
    // )));

    // scene.push(Box::new(Sphere::sphere(
    //     Vec3::new(9.0, -5.0, 0.0),
    //     6.0,
    //     Material::Metal {
    //         albedo: Vec3::new(0.6, 0.6, 0.6),
    //         roughness: 0.0,
    //     },
    // )));

    // scene.push(Box::new(Sphere::sphere(
    //     Vec3::new(-7.0, -4.0, 0.0),
    //     4.0,
    //     // Material::Emmisive { color: Vec3::new(0.3, 0.2, 0.8), strenght: 10.0 }
    //     Material::Dielectric { ref_idx: 1.45 },
    // )));

    scene.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        },
    )));

    let mut rng = rand::thread_rng();
    for a in -8..8 {
        for b in -5..5 {
            let choose_mat = rng.gen::<f32>();
            let centre = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (centre - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    scene.push(Box::new(Sphere::sphere(
                        centre,
                        0.2,
                        Material::Lambertian { albedo },
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_init(0.5, 1.0);
                    let roughness = rng.gen_range(0.0, 0.5);
                    scene.push(Box::new(Sphere::sphere(
                        centre,
                        0.2,
                        Material::Metal { albedo, roughness },
                    )));
                } else {
                    scene.push(Box::new(Sphere::sphere(
                        centre,
                        0.2,
                        Material::Dielectric { ref_idx: 1.5 },
                    )));
                }
            }
        }
    }
    scene.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric { ref_idx: 1.5 },
    )));
    scene.push(Box::new(Sphere::sphere(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    )));
    scene.push(Box::new(Sphere::sphere(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            roughness: 0.0,
        },
    )));

    // let vec = scene.to_owned();
    //init world and add the scene into it.
    // let world = HittableList::new(*scene.clone());
    let world = HittableList::new(scene);

    //setup the camera parameters and add camera.
    // let aspect_ratio = *image_width as f32 / *image_height as f32;
    // let look_from = Vec3::new(0.0, 1.0, 10.0);
    // let look_at = Vec3::new(0.0, 0.0, 0.0);
    // let vup = Vec3::new(0.0, 1.0, 0.0);

    // let dist_to_focus = 10.0;
    // let apeture = 4.0;

    let buffer = (0..image_height)
        .into_par_iter().rev()
        .flat_map(|y| {
            (0..image_width)
                .flat_map(|x| {
                    let color: Vec3 = (0..num_sample)
                        .map(|_| {
                            let mut rng = rand::thread_rng();
                            let u: f32 =
                                ((x as f32) + rng.gen::<f32>()) / ((image_width - 1) as f32);
                            let v: f32 =
                                ((y as f32) + rng.gen::<f32>()) / ((image_height - 1) as f32);
                            let ray = cam.get_ray(u, v);
                            return color(&ray, &world, 30);
                        })
                        .fold(Vec3::new(0.0, 0.0, 0.0), |acc, c| acc + c);

                    let col = color / (num_sample as f32);
                    let col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

                    let ir = (255.999 * col.r()) as u8;
                    let ig = (255.999 * col.g()) as u8;
                    let ib = (255.999 * col.b()) as u8;

                    vec![Rgba([ir, ig, ib, 255])]
                })
                .collect::<Vec<Rgba<u8>>>()
        })
        .collect::<Vec<Rgba<u8>>>();

    let flat_buffer: Vec<u8> = buffer
        .into_iter()
        .flat_map(|rgba| rgba.0.iter().cloned().collect::<Vec<_>>())
        .collect();

    let cloned_flat_buffer = flat_buffer.clone();

    if save {
        let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_vec(image_width as u32, image_height as u32, flat_buffer)
                .expect("Failed to create ImageBuffer from flat_buffer");

        match image_buffer.save("image.png") {
            Err(e) => eprintln!("Error writing file: {}", e),
            Ok(()) => println!("Done"),
        }
    }

    return cloned_flat_buffer;
}
