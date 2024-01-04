
use eframe::epi;
use egui::CtxRef;
pub struct App{

}

impl epi::App for App{

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{

        });
        
    }

    fn name(&self) -> &str {
        "Simple Ray Tracer"
    }
}