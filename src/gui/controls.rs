use std::str::FromStr;

use eframe::egui;

use crate::{
	app::{AppMode, EclipticaApp},
	core::astro::specs::sun_position,
};

pub fn render_controls(app: &mut EclipticaApp, ui: &mut egui::Ui) {
	ui.heading("Panel Sterowania");
	ui.add_space(10.0);

	match app.mode {
		AppMode::BasicCalculations => {
			ui.label("Scenariusz 1: Data -> Kąt");

			// Poprawione szerokości inputów
			ui.horizontal(|ui| {
				ui.add(egui::TextEdit::singleline(&mut app.input_year).desired_width(50.0));
				ui.label("-");
				ui.add(egui::TextEdit::singleline(&mut app.input_month).desired_width(30.0));
				ui.label("-");
				ui.add(egui::TextEdit::singleline(&mut app.input_day).desired_width(30.0));
			});

			ui.horizontal(|ui| {
				ui.add(egui::TextEdit::singleline(&mut app.input_hour).desired_width(30.0));
				ui.label(":");
				ui.add(egui::TextEdit::singleline(&mut app.input_minute).desired_width(30.0));
				ui.label(" Strefa:");
				ui.add(egui::TextEdit::singleline(&mut app.input_zone).desired_width(65.0));
			});

			ui.add_space(5.0);

			ui.horizontal(|ui| {
				if ui.button("Oblicz Kąt").clicked() {
					// Budujemy ciąg znaków w formacie ISO 8601 (np.
					// 1990-04-07T00:02:00+02:00)
					let dt_str = format!(
						"{}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:00{}",
						app.input_year.trim(),
						app.input_month.trim(),
						app.input_day.trim(),
						app.input_hour.trim(),
						app.input_minute.trim(),
						app.input_zone.trim()
					);

					// ZMIANA: Zamiast Zoned używamy Timestamp, który idealnie
					// radzi sobie z formatem ISO 8601 zawierającym samo
					// przesunięcie (np. +02:00)
					match FromStr::from_str(&dt_str) as Result<jiff::Timestamp, _> {
						Ok(timestamp) => {
							// Konwersja czasu na sekundy Uniksa, a potem na
							// Julian Day
							let unix_seconds = timestamp.as_second() as f64;
							let jd = (unix_seconds / 86400.0) + 2440587.5;

							// Wywołanie Twojego dokładnego silnika VSOP87
							let (lon, distance_au) = sun_position(jd);
							let odleglosc_km = distance_au * 149.5978707;

							// Aktualizacja tekstu w panelu
							app.calc1_result = format!(
								"Kąt ekliptyczny: {:.6}°\nOdległość: {:.3} mln km\nJulian Day: {:.4}",
								lon, odleglosc_km, jd
							);

							// Aktualizacja pozycji Ziemi na głównym rysunku
							// (Canvas)
							app.earth_angle = lon.to_radians() as f32;

							// Parsujemy rok (zakładamy bazowy 2000 dla
							// wyliczeń)
							if let Ok(year) = app.input_year.trim().parse::<f32>() {
								let years_from_2000 = year - 2000.0;
								// Rzeczywista precesja jest zbyt wolna do
								// zauważenia.
								// Sztucznie ją przyspieszamy: np. 1 stopień na
								// każdy rok.
								app.visual_precession_angle = (years_from_2000 * 1.0).to_radians();
							}
						}
						Err(e) => {
							app.calc1_result = format!("Błąd formatu daty:\n{}", e);
						}
					}
				}
			});

			// Wyświetlanie wyniku pod przyciskiem
			if !app.calc1_result.is_empty() {
				ui.add_space(10.0);
				ui.label(egui::RichText::new(&app.calc1_result).color(egui::Color32::GREEN));
			}

			ui.separator();

			ui.label("Scenariusz 2: Kąt -> Data");
			ui.horizontal(|ui| {
				let _ = ui.button("Oblicz Datę");
			});
		}
		AppMode::Animation => {
			ui.label("Sterowanie Animacją");
			ui.horizontal(|ui| {
				if ui.button("▶ Play").clicked() {
					app.is_playing = true;
				}
				if ui.button("⏸ Pause").clicked() {
					app.is_playing = false;
				}
			});

			// Konwertujemy obecny kąt Ziemi na stopnie dla suwaka (mod 360, aby
			// zapętlał się)
			let mut angle_degrees = (app.earth_angle.to_degrees() % 360.0).max(0.0);

			// Rysujemy suwak operujący na stopniach
			if ui.add(egui::Slider::new(&mut angle_degrees, 0.0..=360.0).text("Pozycja Ziemi (°)")).changed() {
				// Jeśli użytkownik przesunął suwak, aktualizujemy faktyczny kąt
				// w radianach
				app.earth_angle = angle_degrees.to_radians();
			}

			// Obliczamy pozycję jako ułamek (0.0000 do 1.0000)
			let position_fraction = angle_degrees / 360.0;
			ui.label(format!("Pozycja ułamkowa: {:.4}", position_fraction));
		}
		AppMode::PointTables => {
			ui.label("Generowanie Tabeli");
			let _ = ui.button("Generuj Cykl");
		}
	}
}
