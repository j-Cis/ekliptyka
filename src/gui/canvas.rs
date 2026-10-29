use eframe::egui;

use crate::app::EclipticaApp;

pub fn render_orbit(app: &mut EclipticaApp, ui: &mut egui::Ui) {
	// --- FLAGA DEBUGOWANIA ---
	const DEBUG_ANGLE: bool = true;

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
	painter.circle_filled(sun_pos, 18.0, egui::Color32::from_rgb(255, 160, 50));

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

	painter.add(egui::Shape::line(orbit_points, egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 255, 200))));

	// --- ZNACZNIKI NA ORBICIE ---
    
    // A. Aphelium i Peryhelium (Linie poprzeczne + tekst)
    let font_id = egui::FontId::proportional(12.0);
    
    // Peryhelium (kąt zgodny z precesją)
    let peri_r = p / (1.0 + e); 
    let peri_pos = sun_pos + egui::vec2(peri_r * precession.cos(), -peri_r * precession.sin());
    let peri_perp = egui::vec2(precession.sin(), precession.cos()) * 8.0; 
    let peri_out = egui::vec2(precession.cos(), -precession.sin()); 
    painter.line_segment([peri_pos - peri_perp, peri_pos + peri_perp], egui::Stroke::new(2.0, egui::Color32::RED));
    painter.text(peri_pos + peri_out * 26.0, egui::Align2::CENTER_CENTER, "Peryhelium", font_id.clone(), egui::Color32::RED);
	
    // Aphelium (kąt przeciwny do precesji)
    let aph_theta = precession + std::f32::consts::PI;
    let aph_r = p / (1.0 - e); 
    let aph_pos = sun_pos + egui::vec2(aph_r * aph_theta.cos(), -aph_r * aph_theta.sin());
    let aph_out = egui::vec2(aph_theta.cos(), -aph_theta.sin());
    painter.line_segment([aph_pos - peri_perp, aph_pos + peri_perp], egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 200, 255)));
    painter.text(aph_pos + aph_out * 26.0, egui::Align2::CENTER_CENTER, "Aphelium", font_id.clone(), egui::Color32::from_rgb(100, 200, 255));
	
    // B. Równonoce (Zielone Romby na osi poziomej - kąty 0 i PI)
    let equinoxes = [
        (0.0, "Northward\nEquinox", egui::Align2::LEFT_CENTER, egui::vec2(18.0, 0.0)),
        (std::f32::consts::PI, "Southward\nEquinox", egui::Align2::RIGHT_CENTER, egui::vec2(-18.0, 0.0)),
    ];
    let mut eq_positions = Vec::new();
    for &(eq_theta, label, align, text_offset) in &equinoxes {
        let r = p / (1.0 + e * (eq_theta - precession).cos());
        let pos = sun_pos + egui::vec2(r * eq_theta.cos(), -r * eq_theta.sin());
        eq_positions.push(pos);
        
        let size = 6.0;
        painter.add(egui::Shape::convex_polygon(
            vec![
                pos + egui::vec2(0.0, -size),
                pos + egui::vec2(size, 0.0),
                pos + egui::vec2(0.0, size),
                pos + egui::vec2(-size, 0.0),
            ],
            egui::Color32::GREEN,
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        ));
        
        // Etykieta dwuwierszowa po bokach elipsy
        painter.text(pos + text_offset, align, label, font_id.clone(), egui::Color32::GREEN);
    }
	
    // C. Przesilenia (Pomarańczowe Kwadraty na osi pionowej - kąty PI/2 i 3PI/2)
    let solstices = [
        (std::f32::consts::FRAC_PI_2, "Northern\nSolstice", egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -16.0)),
        (3.0 * std::f32::consts::FRAC_PI_2, "Southern\nSolstice", egui::Align2::CENTER_TOP, egui::vec2(0.0, 16.0)),
    ];
    let mut sol_positions = Vec::new();
    for &(sol_theta, label, align, text_offset) in &solstices {
        let r = p / (1.0 + e * (sol_theta - precession).cos());
        let pos = sun_pos + egui::vec2(r * sol_theta.cos(), -r * sol_theta.sin());
        sol_positions.push(pos);
        
        let size = 4.5;
        painter.rect_filled(
            egui::Rect::from_center_size(pos, egui::vec2(size * 2.0, size * 2.0)),
            0.0,
            egui::Color32::from_rgb(255, 120, 0),
        );
        
        // Etykieta dwuwierszowa nad/pod elipsą
        painter.text(pos + text_offset, align, label, font_id.clone(), egui::Color32::from_rgb(255, 120, 0));
    }

	// --- LINIE DEBUGOWANIA OŚWIETLAJĄCE GEOMETRIĘ ---
	// --- LINIE I PUNKTY DEBUGOWANIA ---
    if DEBUG_ANGLE {
        let black_stroke = egui::Stroke::new(1.2, egui::Color32::BLACK);
        let dot_radius = 4.0; // Mniejsze od Ziemi (Ziemia ma zazwyczaj promień 6.0-8.0)
	
        // 1. Linia apsyd (Peryhelium <-> Aphelium)
        painter.line_segment([peri_pos, aph_pos], black_stroke);
	
        // 2. Punkty prostopadłe przechodzące przez Słońce (Semi-latus rectum w ognisku)
        // Kąty prostopadłe do osi apsyd: precession + PI/2 oraz precession - PI/2
        let theta_f1 = precession + std::f32::consts::FRAC_PI_2;
        let theta_f2 = precession - std::f32::consts::FRAC_PI_2;
        // Ponieważ cos(PI/2) = 0, promień r z równania biegunowego wynosi dokładnie p
        let pos_f1 = sun_pos + egui::vec2(p * theta_f1.cos(), -p * theta_f1.sin());
        let pos_f2 = sun_pos + egui::vec2(p * theta_f2.cos(), -p * theta_f2.sin());
	
        painter.line_segment([pos_f1, pos_f2], black_stroke);
        painter.circle_filled(pos_f1, dot_radius, egui::Color32::BLACK);
        painter.circle_filled(pos_f2, dot_radius, egui::Color32::BLACK);
	
        // 3. Punkty na małej osi elipsy (przechodzące przez geometryczny środek elipsy)
        // Środek elipsy jest odsunięty od ogniska o wektor -a * e wzdłuż osi apsyd
        let ellipse_center = sun_pos - egui::vec2(a * e * precession.cos(), -a * e * precession.sin());
        // Półoś mała: b = a * sqrt(1 - e^2)
        let b = a * (1.0 - e * e).sqrt();
        // Wektor jednostkowy prostopadły do osi apsyd
        let perp_unit = egui::vec2(-precession.sin(), -precession.cos());
        let pos_c1 = ellipse_center + perp_unit * b;
        let pos_c2 = ellipse_center - perp_unit * b;
	
        painter.line_segment([pos_c1, pos_c2], black_stroke);
        painter.circle_filled(pos_c1, dot_radius, egui::Color32::BLACK);
        painter.circle_filled(pos_c2, dot_radius, egui::Color32::BLACK);
	
        // Dotychczasowe linie celownicze pór roku
        painter.line_segment(
            [eq_positions[0], eq_positions[1]], 
            egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 255, 0))
        );
        painter.line_segment(
            [sol_positions[0], sol_positions[1]], 
            egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 120, 0))
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
		painter.circle_filled(earth_pos, 10.0, egui::Color32::from_rgb(60, 120, 255));
	}

	// 4. Rysowanie wygenerowanego cyklu (Scenariusz 3)
	if app.show_multiple_earths {
		// <--- NOWY WARUNEK
		let font_id_earth = egui::FontId::proportional(14.0);
		for pt in &app.calc3_points {
			let theta = pt.angle_rad + std::f32::consts::PI;
			let earth_r = p / (1.0 + e * (theta - precession).cos());
			let ex = earth_r * theta.cos();
			let ey = -earth_r * theta.sin();
			let pos = sun_pos + egui::vec2(ex, ey);

			painter.circle_filled(pos, 10.0, egui::Color32::from_rgb(60, 120, 255));

			let text_offset = egui::vec2(28.0 * theta.cos(), -28.0 * theta.sin());
			painter.text(
				pos + text_offset,
				egui::Align2::CENTER_CENTER,
				&pt.label,
				font_id_earth.clone(),
				egui::Color32::from_rgb(100, 150, 255),
			);
		}
	}
}
