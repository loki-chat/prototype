use eframe::egui;

fn main() {
	let native_options = eframe::NativeOptions::default();
	eframe::run_native(
		"Loki (Prototype)",
		native_options,
		Box::new(|cc| Box::new(App::new(cc))),
	)
	.expect("Failed to create window");
}

struct App;

impl App {
	fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		Self {}
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Hello, World!");
		});
	}
}
