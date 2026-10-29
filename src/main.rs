use eframe::egui;
use ekliptyka::app::EclipticaApp;

fn main() -> eframe::Result<()> {
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_inner_size([1200.0, 800.0])
			.with_title("Ecliptica - Symulator Orbity Ziemi"),
		..Default::default()
	};

	eframe::run_native("Ecliptica", options, Box::new(|_cc| Ok(Box::new(EclipticaApp::default()))))
}
