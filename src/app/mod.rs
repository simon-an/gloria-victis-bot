use crate::{desktop::window_size, shared::BotMode};
use crate::{game_interactions, statics::*, Resolution};
use eframe::{egui, epi};
use egui::{ComboBox, ScrollArea};
use inputbot::MouseCursor;
use tokio::sync::mpsc::Receiver;
use std::sync::Arc;
use image::*;

#[derive(Default)]
pub struct MyApp {
    // texture: Option<egui::TextureId>,
    texture: Option<(egui::Vec2, egui::TextureId)>,
    mode: BotMode,
    animate_progress_bar: bool,
    text: String,
    error: String,
    channel: Option<Receiver<DynamicImage>>
}

impl MyApp {
    fn new (channel: Receiver<DynamicImage>) -> Self {
        Self {
            channel: Some(channel),
            ..Default::default()
        }
    }
}

pub fn create(channel: Receiver<DynamicImage>) -> !{
    let options = eframe::NativeOptions {
        // Let's show off that we support transparent windows
        transparent: true,
        ..Default::default()
    };
    let app = Box::new(MyApp::new(channel));
    eframe::run_native(app, options);
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "✨ GV BOT"
    }

    // fn ui(&mut self, ctx: &CtxRef, integration_context: &mut IntegrationContext) {
    //     let repaint_signal = integration_context.repaint_signal.clone();
    //     let thread = std::thread::spawn(move || {
    //         repaint_signal.request_repaint();
    //     });
    //     thread.join();
    // }

    // pub fn set_texture(&mut self, image_buffer: ImageBuffer<Rgba<u8>, Vec<_>>, size: (usize, usize)){
    //     let pixels = image_buffer.into_vec();
    //     assert_eq!(size.0 * size.1 * 4, pixels.len());
    //     let pixels: Vec<_> = pixels
    //         .chunks_exact(4)
    //         .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
    //         .collect();

    //     // Allocate a texture:
    //     let texture = frame
    //         .tex_allocator()
    //         .alloc_srgba_premultiplied(size, &pixels);
    //     let size = egui::Vec2::new(size.0 as f32, size.1 as f32);
    //     self.texture = Some((size, texture));
    // }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        
        if let Some(receiver) = &mut self.channel {
            // Are we there yet?
            match receiver.try_recv() {
                Ok(image) => {
                    // let image = image::load_from_memory(&res).expect("Failed to load image");
                    let image_buffer = image.to_rgba8();
                    let size = (image.width() as usize, image.height() as usize);
                    let pixels = image_buffer.into_vec();
                    assert_eq!(size.0 * size.1 * 4, pixels.len());
                    let pixels: Vec<_> = pixels
                        .chunks_exact(4)
                        .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                        .collect();
        
        
                    // Allocate a texture:q
                    let texture = frame
                        .tex_allocator()
                        .alloc_srgba_premultiplied(size, &pixels);
                    let size = egui::Vec2::new(size.0 as f32, size.1 as f32);
                    self.texture = Some((size, texture));
                },
                Err(_) => {

                }
            }
        }

        if self.texture.is_none() {
            let image_data = include_bytes!("../../images/blox.jpg");

            let image = image::load_from_memory(image_data).expect("Failed to load image");
            let image_buffer = image.to_rgba8();
            let size = (image.width() as usize, image.height() as usize);
            let pixels = image_buffer.into_vec();
            assert_eq!(size.0 * size.1 * 4, pixels.len());
            let pixels: Vec<_> = pixels
                .chunks_exact(4)
                .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                .collect();


            // Allocate a texture:q
            let texture = frame
                .tex_allocator()
                .alloc_srgba_premultiplied(size, &pixels);
            let size = egui::Vec2::new(size.0 as f32, size.1 as f32);
            self.texture = Some((size, texture));
            // self.texture = Some(texture);
        }

        let Self {
            animate_progress_bar,
            mode,
            text,
            error,
            texture,
            ..
        } = self;

        let (rect, width, height) = window_size();
        let window_pos = (rect.left, rect.top);
        let resolution: Resolution = (width, height).into();

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            // show_menu_bar(ui);
            ui.horizontal(|ui| {
                ui.label(format!("Current mode: {}", &mode.to_string()));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Here is an image for you:");
                if let Some((size, texture)) = texture {
                    ui.image(*texture, *size);

                    // ui.heading("This is an image you can click:");
                    // ui.add(egui::ImageButton::new(texture, size));
                }
            });
        });

        egui::SidePanel::right("egui_demo_panel")
            .min_width(200.0)
            .default_width(250.0)
            .show(ctx, |ui| {
                egui::trace!(ui);
                ui.vertical_centered(|ui| {
                    ui.heading("✒ modi");
                    ui.separator();

                    ScrollArea::vertical().show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ComboBox::from_label("Select Mode")
                                .selected_text(mode.to_string())
                                .show_ui(ui, |ui| {
                                    BotMode::values().iter().for_each(|style| {
                                        ui.selectable_value(mode, *style, style.to_string());
                                    });
                                });

                            ui.separator();
                            // if ui.button("Organize windows").clicked() {
                            //     ui.ctx().memory().reset_areas();
                            // }

                            let response = ui.text_edit_singleline(text);
                            // if response.changed() {

                            // }
                            let counter_target = Arc::clone(&COUNTER_TARGET);
                            ui.label(format!("Target Value: {}", *counter_target.lock().unwrap()));
                            if response.lost_focus()
                            /*&& ui.input().key_pressed(egui::Key::Enter) */
                            {
                                if let Ok(number) = text.parse::<u32>() {
                                    let counter_target = Arc::clone(&COUNTER_TARGET);
                                    *counter_target.lock().unwrap() = number;
                                } else {
                                    error.push_str("Only Numbers allowed");
                                }
                            }

                            if !error.is_empty() {
                                ui.label(format!("Errors: {}", error));
                            }

                            ui.separator();

                            let running = Arc::clone(&RUNNING);
                            let counter = Arc::clone(&COUNTER);

                            let counter_target = Arc::clone(&COUNTER_TARGET);
                            let progress = *counter.lock().unwrap() as f32
                                / *counter_target.lock().unwrap() as f32;
                            *animate_progress_bar = *running.lock().unwrap();
                            let progress_bar = egui::ProgressBar::new(progress)
                                .show_percentage()
                                .animate(*animate_progress_bar);
                            /**animate_progress_bar = */
                            ui.add(progress_bar);
                            // .on_hover_text("The progress bar can be animated!")
                            // .hovered();
                        });
                        ui.separator();

                        egui::Grid::new("my_grid")
                            .num_columns(2)
                            .spacing([20.0, 4.0])
                            .striped(true)
                            .show(ui, |ui| {
                                let (x, y) = MouseCursor::pos();
                                ui.label(format!("Mouse Position Absolute"));
                                ui.label(format!("({:},{:})", x, y));
                                ui.end_row();

                                ui.label(format!("Mouse Position in Window",));
                                ui.label(format!("({:},{:})", x - window_pos.0, y - window_pos.1));
                                ui.end_row();

                                ui.label(format!("Window Size"));
                                ui.label(format!("({:},{:})", width, height));
                                ui.end_row();

                                ui.label(format!("Resolution detected"));
                                ui.label(format!("({:?})", resolution));
                                ui.end_row();
                            });
                    });
                });
            });
        game_interactions::bind_keys_for(*mode, window_pos, resolution);
    }
}
