use std::num::NonZeroU32;



use eframe::{
    egui,
    // epaint::{image, pos2, Color32, Rect},
    wgpu::{self},
};
use ray_tracing_rust::core::{camera::Camera, render::render_scene, vec3::Vec3, hittable::Hittable, hittable_list::HittableList};

const IMAGE_WIDTH: u32 = 1280; //480;
const IMAGE_HEIGHT: u32 = 720; //480;
const NUM_SAMPLE: i32 = 10;

pub struct App{
    pub image_buffer: Vec<u8>,
    previous_time: std::time::Instant,
    pub samples: i32,
    // texture: Option<(egui::Vec2, egui::TextureId)>,
    lookfrom_x: f32,
    lookfrom_y: f32,
    lookfrom_z: f32,
    lookat_x: f32,
    lookat_y: f32,
    lookat_z: f32,
    camera_lookfrom: Vec3,
    camera_lookat: Vec3,
    camera_vup: Vec3,
    camera_vfov: f32,
    camera_aspect: f32,
    camera_aperture: f32,
    camera_focus_dist: f32,

    texture_width: usize,
    texture_height: usize,
    texture_id: egui::TextureId,
    // texture_bind_group_layout: wgpu::BindGroupLayout,
    // texture_bind_group: wgpu::BindGroup,
    scene:  Vec<Box<dyn Hittable>>,
    pub render_button_pressed: bool,
    save_button_pressed: bool,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let eframe::egui_wgpu::RenderState {
            device, renderer, ..
        } = cc.wgpu_render_state.as_ref().unwrap();

        let texture_width = 1280;
        let texture_height = 720;
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size: wgpu::Extent3d {
                width: IMAGE_WIDTH,
                height: IMAGE_HEIGHT,
                // width: texture_width as _,
                // height: texture_height as _,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let texture_id = renderer.write().register_native_texture(
            device,
            &texture.create_view(&wgpu::TextureViewDescriptor::default()),
            wgpu::FilterMode::Linear,
        );

        Self {
            image_buffer: vec![],
            previous_time: std::time::Instant::now(),
            texture_width,
            texture_height,
            texture_id,
            // texture_bind_group_layout,
            // texture_bind_group,
            samples: 1,
            render_button_pressed: false,
            save_button_pressed: false,
            lookfrom_x: 0.0,
            lookfrom_y: 0.0,
            lookfrom_z: 20.0,
            lookat_x: 0.0,
            lookat_y: 0.0,
            lookat_z: 3.0,
            camera_lookfrom: Vec3::new(0.0, 1.0, 10.0),
            camera_lookat: Vec3::new(0.0, 0.0, 0.0),
            camera_vup: Vec3::new(0.0, 1.0, 0.0),
            camera_vfov: 30.0,
            camera_aspect: 40.0,
            camera_aperture: 2.0,
            camera_focus_dist: 10.0,

            scene:  Vec::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let time = std::time::Instant::now();
        let dt = time.duration_since(self.previous_time);

        let ts = dt.as_secs_f32();
        //  self.scene = Vec::new();

        // let world = HittableList::new(&self.scene);

        

        

        egui::SidePanel::left("Left Panel").default_width(100.0).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(format!("FPS: {}", 1.0 / ts));
                ui.label(format!("Frame Time: {}ms", 1000.0 * ts));
                // UI elements for controlling camera parameters
                ui.label("Camera Parameters");
                //  ui.add(egui::Vec2::new(&mut self.camera_lookfrom).prefix("Look From:"));
                //  ui.add(egui::Vec3::new(&mut self.camera_lookat).prefix("Look At:"));
                //  ui.add(egui::Vec3::new(&mut self.camera_vup).prefix("Up Vector:"));
                ui.add(egui::Slider::new(&mut self.lookfrom_x, -40.0..=40.0).text("Look from x"));
                ui.add(egui::Slider::new(&mut self.lookfrom_y, -40.0..=40.0).text("Look from y"));
                ui.add(egui::Slider::new(&mut self.lookfrom_z, -40.0..=40.0).text("Look from z"));

                ui.add(egui::Slider::new(&mut self.lookat_x, -40.0..=40.0).text("Look at x"));
                ui.add(egui::Slider::new(&mut self.lookat_y, -40.0..=40.0).text("Look at y"));
                ui.add(egui::Slider::new(&mut self.lookat_z, -40.0..=40.0).text("Look at z"));

                ui.add(egui::Slider::new(&mut self.camera_vfov, 0.60..=120.0).text("FOV"));
                ui.add(egui::Slider::new(&mut self.camera_aperture, 0.1..=10.0).text("Aperture"));
                ui.add(
                    egui::Slider::new(&mut self.camera_focus_dist, 1.0..=100.0)
                        .text("Focus Distance"),
                );
                ui.add(egui::Slider::new(&mut self.samples, 4..=200).drag_value_speed(2.0).text("samples"));

                

                ui.collapsing("Object list", |ui: &mut egui::Ui|{
                    ui.heading("objects")

                } 
                  
                )

            })
        });

       

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(ctx.style().visuals.panel_fill))
            .show(ctx, |ui| {
                let eframe::egui_wgpu::RenderState {
                    device,
                    queue,
                    renderer,
                    ..
                } = frame.wgpu_render_state().unwrap();

                let size = ui.available_size();
                let size = (size.x.max(100.0) as usize, size.y.max(100.0) as usize);

                // recreate the texture if it is the wrong size
                // if size != (self.texture_width, self.texture_height) {
                (self.texture_width, self.texture_height) = size;

                let texture = device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("Texture"),
                    size: wgpu::Extent3d {
                        width: IMAGE_WIDTH,
                        height: IMAGE_HEIGHT,
                        // width:1280,
                        // height:720,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING, //wgpu::TextureUsages::TEXTURE_BINDING
                    view_formats: &[],
                });

                renderer.write().update_egui_texture_from_wgpu_texture(
                    device,
                    &texture.create_view(&wgpu::TextureViewDescriptor::default()),
                    wgpu::FilterMode::Linear,
                    self.texture_id,
                );

                let look_from = Vec3::new(self.lookfrom_x, self.lookfrom_y, self.lookfrom_z);
                let look_at = Vec3::new(self.lookat_x, self.lookat_y, self.lookat_z);
                let vup = Vec3::new(0.0, 1.0, 0.0);
                let aspect_ratio = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;
                let camera = Camera::camera(
                    look_from,
                    look_at,
                    vup,
                    self.camera_vfov,
                    aspect_ratio,
                    self.camera_aperture,
                    self.camera_focus_dist,
                );

               //Check if render is enabled 
                ui.checkbox(&mut self.render_button_pressed, "Render")
                    .changed();

                if ui.button("Save Img").clicked() {
                    self.save_button_pressed = true;
                };

                // ui.checkbox(&mut self.save_button_pressed, "Save")
                //     .changed();
                // print!("{}", self.render_button_pressed);

                

                if self.render_button_pressed {
                    let image_buffer = render_scene(
                        IMAGE_HEIGHT,
                        IMAGE_WIDTH,
                        self.samples,
                        
                        camera,
                        // final_scene,
                        // world,
                        
                        self.save_button_pressed,
                    );
                    self.save_button_pressed = false;

                    queue.write_texture(
                        texture.as_image_copy(),
                        &image_buffer.as_slice(),
                        wgpu::ImageDataLayout {
                            offset: 0,
                            bytes_per_row: Some(NonZeroU32::new(texture.width() * 4).unwrap()),
                            rows_per_image: None,
                        },
                        texture.size(),
                    );

                    //draw render_buffer image on screen
                    ui.image(self.texture_id, [1280.0, 720.0]);
                }

                // println!("{:?}", texture.width());
                // }

                // println!("debug print {:?}" ,texture.dimension() ) ;

                // println!("debug print {:?}" ,image_buffer ) ;

                // ui.painter().image(self.texture_id, rect, uv, Color32::from_white_alpha(255));

                // }
            });

        // ctx.request_repaint();
        self.previous_time = time;
    }

  
   
}
