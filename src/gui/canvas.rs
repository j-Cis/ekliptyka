use eframe::egui;

use crate::app::EclipticaApp;
use crate::gui::theme;

pub fn render_orbit(app: &mut EclipticaApp, ui: &mut egui::Ui) {
	let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());
	let rect = response.rect;

	let a = rect.width().min(rect.height()) * 0.25;
	let e = 0.6_f32;
	let p = a * (1.0 - e * e);
	let c = a * e;

	let mut sun_pos = rect.center();
	sun_pos.x += c;

	let precession = app.visual_precession_angle;

	// 1. Słońce
	painter.circle_filled(sun_pos, theme::SUN_SIZE, theme::SUN_COLOR);

	// 2. Orbita
	let points = 120;
	let mut orbit_points = Vec::with_capacity(points + 1);
	for i in 0..=points {
		let theta = (i as f32 / points as f32) * std::f32::consts::TAU;
		let r = p / (1.0 + e * (theta - precession).cos());

		let x = r * theta.cos();
		let y = -r * theta.sin();

		orbit_points.push(sun_pos + egui::vec2(x, y));
	}

	painter.add(egui::Shape::line(
		orbit_points,
		egui::Stroke::new(theme::ORBIT_STROKE_WIDTH, theme::ORBIT_COLOR),
	));

	// --- ZNACZNIKI NA ORBICIE ---

	// A. Aphelium i Peryhelium (Linie poprzeczne + tekst)
	let font_id = egui::FontId::proportional(theme::ORBIT_LABELS_FONT_SIZE);

	// Peryhelium (kąt zgodny z precesją)
	let peri_r = p / (1.0 + e);
	let peri_pos = sun_pos + egui::vec2(peri_r * precession.cos(), -peri_r * precession.sin());
	let peri_perp = egui::vec2(precession.sin(), precession.cos()) * 8.0;
	let peri_out = egui::vec2(precession.cos(), -precession.sin());
	painter.line_segment(
		[peri_pos - peri_perp, peri_pos + peri_perp],
		egui::Stroke::new(2.0, theme::PERYHELIUM_COLOR),
	);
	painter.text(
		peri_pos + peri_out * theme::ORBIT_LABELS_OFFSET,
		egui::Align2::CENTER_CENTER,
		"Peryhelium",
		font_id.clone(),
		theme::PERYHELIUM_COLOR,
	);

	// Aphelium (kąt przeciwny do precesji)
	let aph_theta = precession + std::f32::consts::PI;
	let aph_r = p / (1.0 - e);
	let aph_pos = sun_pos + egui::vec2(aph_r * aph_theta.cos(), -aph_r * aph_theta.sin());
	let aph_out = egui::vec2(aph_theta.cos(), -aph_theta.sin());
	painter.line_segment(
		[aph_pos - peri_perp, aph_pos + peri_perp],
		egui::Stroke::new(2.0, theme::APHELIUM_COLOR),
	);
	painter.text(
		aph_pos + aph_out * theme::ORBIT_LABELS_OFFSET,
		egui::Align2::CENTER_CENTER,
		"Aphelium",
		font_id.clone(),
		theme::APHELIUM_COLOR,
	);

	// B. Równonoce (Zielone Romby na osi poziomej - kąty 0 i PI)
	let equinoxes = [
		(0.0, "Northward\nEquinox", egui::Align2::CENTER_CENTER, egui::vec2(theme::ORBIT_LABELS_OFFSET, 0.0)),
		(std::f32::consts::PI, "Southward\nEquinox", egui::Align2::CENTER_CENTER, egui::vec2(-theme::ORBIT_LABELS_OFFSET, 0.0)),
	];
	let mut eq_positions = Vec::new();
	for &(eq_theta, label, align, text_offset) in &equinoxes {
		let r = p / (1.0 + e * (eq_theta - precession).cos());
		let pos = sun_pos + egui::vec2(r * eq_theta.cos(), -r * eq_theta.sin());
		eq_positions.push(pos);

		let size = theme::SEASONS_POINT_SIZE;
		painter.add(egui::Shape::convex_polygon(
			vec![
				pos + egui::vec2(0.0, -size),
				pos + egui::vec2(size, 0.0),
				pos + egui::vec2(0.0, size),
				pos + egui::vec2(-size, 0.0),
			],
			theme::EQUINOX_COLOR,
			egui::Stroke::new(1.0, theme::TEXT_DEFAULT),
		));

		// Etykieta dwuwierszowa po bokach elipsy
		painter.text(pos + text_offset, align, label, font_id.clone(), theme::EQUINOX_COLOR);
	}

	// C. Przesilenia (Pomarańczowe Kwadraty na osi pionowej - kąty PI/2 i
	// 3PI/2)
	let solstices = [
		(
			std::f32::consts::FRAC_PI_2,
			"Northern\nSolstice",
			egui::Align2::CENTER_CENTER,
			egui::vec2(0.0, -theme::ORBIT_LABELS_OFFSET),
		),
		(
			3.0 * std::f32::consts::FRAC_PI_2,
			"Southern\nSolstice",
			egui::Align2::CENTER_CENTER,
			egui::vec2(0.0, theme::ORBIT_LABELS_OFFSET),
		),
	];
	let mut sol_positions = Vec::new();
	for &(sol_theta, label, align, text_offset) in &solstices {
		let r = p / (1.0 + e * (sol_theta - precession).cos());
		let pos = sun_pos + egui::vec2(r * sol_theta.cos(), -r * sol_theta.sin());
		sol_positions.push(pos);

		let size = theme::SEASONS_POINT_SIZE * 0.75; // Skalowanie dla kwadratów (oryginalnie 4.5 przy 6.0 dla rombów)
		painter.rect_filled(
			egui::Rect::from_center_size(pos, egui::vec2(size * 2.0, size * 2.0)),
			0.0,
			theme::SOLSTICE_COLOR,
		);

		// Etykieta dwuwierszowa nad/pod elipsą
		painter.text(pos + text_offset, align, label, font_id.clone(), theme::SOLSTICE_COLOR);
	}

	// --- LINIE DEBUGOWANIA OŚWIETLAJĄCE GEOMETRIĘ ---
	// --- LINIE I PUNKTY DEBUGOWANIA ---
	if app.debug_angle {
		let debug_stroke = egui::Stroke::new(theme::DEBUG_LINE_STROKE_WIDTH, theme::DEBUG_EXTRA_COLOR);
		let dot_radius = theme::DEBUG_POINT_SIZE;

		// 1. Linia apsyd (Peryhelium <-> Aphelium)
		painter.line_segment([peri_pos, aph_pos], debug_stroke);

		// 2. Punkty prostopadłe przechodzące przez Słońce (Semi-latus rectum w
		//    ognisku)
		// Kąty prostopadłe do osi apsyd: precession + PI/2 oraz precession -
		// PI/2
		let theta_f1 = precession + std::f32::consts::FRAC_PI_2;
		let theta_f2 = precession - std::f32::consts::FRAC_PI_2;
		// Ponieważ cos(PI/2) = 0, promień r z równania biegunowego wynosi
		// dokładnie p
		let pos_f1 = sun_pos + egui::vec2(p * theta_f1.cos(), -p * theta_f1.sin());
		let pos_f2 = sun_pos + egui::vec2(p * theta_f2.cos(), -p * theta_f2.sin());

		painter.line_segment([pos_f1, pos_f2], debug_stroke);
		painter.circle_filled(pos_f1, dot_radius, theme::DEBUG_EXTRA_COLOR);
		painter.circle_filled(pos_f2, dot_radius, theme::DEBUG_EXTRA_COLOR);

		// 3. Punkty na małej osi elipsy (przechodzące przez geometryczny środek
		//    elipsy)
		// Środek elipsy jest odsunięty od ogniska o wektor -a * e wzdłuż osi
		// apsyd
		let ellipse_center = sun_pos - egui::vec2(a * e * precession.cos(), -a * e * precession.sin());
		// Półoś mała: b = a * sqrt(1 - e^2)
		let b = a * (1.0 - e * e).sqrt();
		// Wektor jednostkowy prostopadły do osi apsyd
		let perp_unit = egui::vec2(-precession.sin(), -precession.cos());
		let pos_c1 = ellipse_center + perp_unit * b;
		let pos_c2 = ellipse_center - perp_unit * b;

		painter.line_segment([pos_c1, pos_c2], debug_stroke);
		painter.circle_filled(pos_c1, dot_radius, theme::DEBUG_EXTRA_COLOR);
		painter.circle_filled(pos_c2, dot_radius, theme::DEBUG_EXTRA_COLOR);

		// Dotychczasowe linie celownicze pór roku
		painter.line_segment(
			[eq_positions[0], eq_positions[1]],
			egui::Stroke::new(theme::SEASONS_LINE_STROKE_WIDTH, theme::EQUINOX_COLOR),
		);
		painter.line_segment(
			[sol_positions[0], sol_positions[1]],
			egui::Stroke::new(theme::SEASONS_LINE_STROKE_WIDTH, theme::SOLSTICE_COLOR),
		);
	}

	// --- RYSOWANIE ZIEMI ---

	// 3. Rysowanie pojedynczej Ziemi (dla Scenariusza 1, 2 lub Animacji)
	if app.show_single_earth {
		let earth_theta = app.earth_angle + std::f32::consts::PI;
		let earth_r = p / (1.0 + e * (earth_theta - precession).cos());
		let ex = earth_r * earth_theta.cos();
		let ey = -earth_r * earth_theta.sin();
		let earth_pos = sun_pos + egui::vec2(ex, ey);
		painter.circle_filled(earth_pos, theme::EARTH_SIZE, theme::EARTH_COLOR);
	}

	// 4. Rysowanie wygenerowanego cyklu (Scenariusz 3)
	if app.show_multiple_earths {
		let font_id_earth = egui::FontId::proportional(theme::SCENARIO_3_LABEL_FONT_SIZE);
		for pt in &app.calc3_points {
			let theta = pt.angle_rad + std::f32::consts::PI;
			let earth_r = p / (1.0 + e * (theta - precession).cos());
			let ex = earth_r * theta.cos();
			let ey = -earth_r * theta.sin();
			let pos = sun_pos + egui::vec2(ex, ey);

			painter.circle_filled(pos, theme::EARTH_SIZE, theme::EARTH_COLOR);

			let text_offset = egui::vec2(
				theme::SCENARIO_3_LABEL_OFFSET * theta.cos(),
				-theme::SCENARIO_3_LABEL_OFFSET * theta.sin(),
			);
			painter.text(
				pos + text_offset,
				egui::Align2::CENTER_CENTER,
				&pt.label,
				font_id_earth.clone(),
				theme::EARTH_COLOR, // Zakładamy ten sam kolor etykiety dla Ziemi zgodnie z uwagą w theme.rs
			);
		}
	}
}