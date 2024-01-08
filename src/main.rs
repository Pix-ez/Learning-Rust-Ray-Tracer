// use eframe::epi;
// use egui::{CtxRef, TextureId, Image};
use eframe::wgpu;

mod gui;
use gui::App;
// use wgpu;
// cross build --target x86_64-pc-windows-gnu --release
fn main() {

    eframe::run_native(
        "simple Ray Tracer",
        eframe::NativeOptions {
            renderer: eframe::Renderer::Wgpu,
            wgpu_options: eframe::egui_wgpu::WgpuConfiguration {
                device_descriptor: wgpu::DeviceDescriptor {
                    ..Default::default()
                },
                present_mode: wgpu::PresentMode::AutoNoVsync,
                power_preference: wgpu::PowerPreference::HighPerformance,
                ..Default::default()
            },
            ..Default::default()
        },
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .unwrap()
}
