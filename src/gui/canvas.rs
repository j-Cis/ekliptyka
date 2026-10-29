use eframe::egui;

use crate::app::EclipticaApp;

pub fn render_orbit(app: &mut EclipticaApp, ui: &mut egui::Ui) {
	let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());
	let rect = response.rect;

	// Mniejszy mnożnik (np. 0.25) by orbita zmieściła się w całości
	let a = rect.width().min(rect.height()) * 0.25;
	let e = 0.6_f32;
	let p = a * (1.0 - e * e);
	let c = a * e;

	// Przesuwamy punkt Słońca nieco od geometrycznego środka panelu,
	// aby elipsa nie uderzała w krawędź
	let mut screen_center = rect.center();
	screen_center.x += c; // Przesunięcie wizualne

	let precession = app.visual_precession_angle;

	// 1. Rysowanie Słońca w stałym miejscu (ognisku)
	painter.circle_filled(screen_center, 18.0, egui::Color32::from_rgb(255, 160, 50));

	// 2. Rysowanie orbity z równania biegunowego
	let points = 120;
	let mut orbit_points = Vec::with_capacity(points + 1);
	for i in 0..=points {
		// Kąt theta obrotu wokół Słońca (0 do 2 PI)
		let theta = (i as f32 / points as f32) * std::f32::consts::TAU;

		// Równanie elipsy we współrzędnych biegunowych z precesją
		// r = p / (1 + e * cos(theta - omega))
		let r = p / (1.0 + e * (theta - precession).cos());

		let x = r * theta.cos();
		let y = r * theta.sin();

		orbit_points.push(screen_center + egui::vec2(x, y));
	}

	painter.add(egui::Shape::line(
		orbit_points,
		egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 255, 200)), // Jasna linia
	));

	// 3. Rysowanie Ziemi na orbicie
	let earth_theta = app.earth_angle;
	let earth_r = p / (1.0 + e * (earth_theta - precession).cos());

	let ex = earth_r * earth_theta.cos();
	let ey = earth_r * earth_theta.sin();

	let earth_pos = screen_center + egui::vec2(ex, ey);
	painter.circle_filled(earth_pos, 10.0, egui::Color32::from_rgb(60, 120, 255));
}
