use crate::{desktop::window_size, shared::BotMode};
use crate::{game_interactions, statics::*, Resolution};
use eframe::egui::load::SizedTexture;
use eframe::egui::{self, ImageSource};
use egui::{ComboBox, ScrollArea};
use image::*;
use inputbot::MouseCursor;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

impl<'a> Default for MyApp<'a> {
    fn default() -> Self {
        Self {
            texture: None,
            mode: BotMode::default(),
            animate_progress_bar: true,
            text: "".to_string(),
            error: "".to_string(),
            channel: None,
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MyApp<'a> {
    // texture: Option<egui::TextureId>,
    // texture: Option<(egui::Vec2, egui::TextureId)>,
    #[serde(skip)]
    texture: Option<egui::ImageSource<'a>>,
    mode: BotMode,
    animate_progress_bar: bool,
    text: String,
    error: String,
    #[serde(skip)]
    channel: Option<Receiver<DynamicImage>>,
}

impl<'a> MyApp<'a> {
    fn new(channel: Receiver<DynamicImage>, cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Self {
            channel: Some(channel),
            ..Default::default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

pub fn create(channel: Receiver<DynamicImage>) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        // Let's show off that we support transparent windows
        transparent: true,
        initial_window_size: Some([400.0, 300.0].into()),
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "✨ GV BOT",
        options,
        Box::new(|cc| Box::new(MyApp::new(channel, cc))),
    )
}

impl<'a> eframe::App for MyApp<'a> {
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

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        if let Some(receiver) = &mut self.channel {
            // Are we there yet?
            match receiver.try_recv() {
                Ok(image) => {
                    // // let image = image::load_from_memory(&res).expect("Failed to load image");
                    let image_buffer = image.to_rgba8();
                    let size = (image.width() as usize, image.height() as usize);
                    let pixels = image_buffer.into_vec();
                    assert_eq!(size.0 * size.1 * 4, pixels.len());
                    // let pixels: Vec<_> = pixels
                    //     .chunks_exact(4)
                    //     .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                    //     .collect();
                    let img =
                        egui::ColorImage::from_rgba_unmultiplied(size.into(), pixels.as_slice());
                    let texture = ctx.load_texture("my-image", img, Default::default());
                    let src = ImageSource::Texture(SizedTexture::from_handle(&texture));
                    self.texture = Some(src);
                }
                Err(_) => {}
            }
        }

        if self.texture.is_none() {
            let img: ImageSource = egui::include_image!("../../images/blox.jpg");
            // let texture: &egui::TextureHandle = self.texture.insert(
            //     // Load the texture only once.
            //     ctx.load_texture("default-image", img, Default::default()),
            // );
            // let img = egui::Image::new(img).rounding(5.0);
            // // let image_data = include_bytes!("../../images/blox.jpg");
            // // let img: eframe::epaint::FontImage = egui::FontImage::new(
            // //     [256, 256],
            // // );
            self.texture = Some(img);
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
                if let Some(texture) = texture {
                    ui.image(texture.clone());
                    // ui.heading("This is an image you can click:");
                }
            });
        });

        egui::SidePanel::right("egui_demo_panel")
            .min_width(200.0)
            .default_width(250.0)
            .show(ctx, |ui| {
                // egui::trace!(ui);
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
                            /* animate_progress_bar = */
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
