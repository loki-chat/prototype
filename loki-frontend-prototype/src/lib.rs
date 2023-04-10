mod app;
mod id;
mod model;
mod ui;

pub fn run() {
	let native_options = eframe::NativeOptions::default();
	eframe::run_native(
		"Loki (Prototype)",
		native_options,
		Box::new(|cc| Box::new(app::App::new(cc))),
	)
	.expect("Failed to create window");
}
