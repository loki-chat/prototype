use eframe::egui;

use crate::ui::views::MainView;

pub struct App {
	view: View,
}

impl App {
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		Self {
			view: View::Main(MainView::default()),
		}
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			match &mut self.view {
				View::Main(view) => view.ui(ui),
			};
		});
	}
}

enum View {
	Main(MainView),
}
