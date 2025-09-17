use eframe::egui::{self, Align, Direction, Layout, RichText};
use eframe::EventLoopBuilderHook;
use winit::platform::windows::EventLoopBuilderExtWindows;

pub fn about() -> eframe::Result {
    // stupid hack but we are on windows only sooooo
    let event_loop_builder: Option<EventLoopBuilderHook> = Some(Box::new(|event_loop_builder| {
		event_loop_builder.with_any_thread(true);
	}));
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 200.0]).with_resizable(false),
        event_loop_builder,
        ..Default::default()
    };
    eframe::run_native(
        "discordloops - about",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<AboutApp>::default())
        }),
    )
}

struct AboutApp {
    _meower: bool
}

impl Default for AboutApp {
    fn default() -> Self {
        Self {
            // yep we are meowing
            _meower: true
        }
    }
}

impl eframe::App for AboutApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                ui.add_sized([100.0, 100.0], egui::Image::new(egui::include_image!("../creature.png")));
                ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                    ui.label(RichText::new("DiscordLoops").size(22.0).italics());
                    ui.label("a simple discord rich presence app for fl studio written in rust by the creature on the right");
                });
            });
            ui.separator();
            ui.label(RichText::new("credits to").size(11.0));
            ui.label("dtolnay · EmbarkStudios · tjardoo · VoidStarKat · nabijaczleweli");
            ui.with_layout(Layout::right_to_left(Align::BOTTOM), |ui| {
                ui.hyperlink_to("source code", "https://github.com/miaaaa0a/discordloops");
                ui.label("·");
                ui.hyperlink_to("twitter", "https://x.com/miaaaa0a");
            });
        });
    }
}