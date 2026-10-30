use std::str::FromStr;

use eframe::egui;

use crate::{
	app::{AppMode, EclipticaApp},
	core::astro::{
		astro_plural::Cycle,
		specs::{find_time_and_distance_for_longitude, sun_position},
	},
	gui::theme,
};

pub fn render_controls(app: &mut EclipticaApp, slots: &mut [egui::Ui]) {
	match app.mode {
		AppMode::BasicCalculations => {
			let mut iter = slots.iter_mut();
			let ui0 = iter.next().unwrap();
			let ui1 = iter.next().unwrap();
			let ui2 = iter.next().unwrap();

			// --- KOLUMNA 1: SCENARIUSZ 1 ---
			ui0.label("Scenariusz 1: 🗓️ -> 🌀");
			ui0.horizontal(|ui| {
				ui.add(egui::TextEdit::singleline(&mut app.input_from_timer.year).desired_width(50.0));
				ui.label("-");
				ui.add(egui::TextEdit::singleline(&mut app.input_from_timer.month).desired_width(30.0));
				ui.label("-");
				ui.add(egui::TextEdit::singleline(&mut app.input_from_timer.day).desired_width(30.0));
			});

			ui0.horizontal(|ui| {
				ui.add(egui::TextEdit::singleline(&mut app.input_from_timer.hour).desired_width(30.0));
				ui.label(":");
				ui.add(egui::TextEdit::singleline(&mut app.input_from_timer.minute).desired_width(30.0));
			});

			ui0.horizontal(|ui| {
				ui.label(" Strefa:");
				ui.add(egui::TextEdit::singleline(&mut app.input_from_timer.zone).desired_width(65.0));
			});

			ui0.add_space(5.0);

			ui0.horizontal(|ui| {
				let calc1_btn = egui::Button::new(
					egui::RichText::new("Oblicz 🌀")
						.font(egui::FontId::proportional(theme::FONT_SIZE_BTN))
						.color(theme::TEXT_DEFAULT),
				)
				.min_size(egui::vec2(100.0, theme::BTN_HEIGHT))
				.fill(theme::BTN_ACTION_BG);

				if ui.add(calc1_btn).clicked() {
					let dt_str = format!(
						"{}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:00{}",
						app.input_from_timer.year.trim(),
						app.input_from_timer.month.trim(),
						app.input_from_timer.day.trim(),
						app.input_from_timer.hour.trim(),
						app.input_from_timer.minute.trim(),
						app.input_from_timer.zone.trim()
					);

					match FromStr::from_str(&dt_str) as Result<jiff::Timestamp, _> {
						Ok(timestamp) => {
							let unix_seconds = timestamp.as_second() as f64;
							let jd = (unix_seconds / 86400.0) + 2440587.5;

							let (lon, distance_au) = sun_position(jd);
							let odleglosc_km = distance_au * 149.5978707;

							app.calc1_result = format!(
								"Kąt ekliptyczny: {:.6}°\nOdległość: {:.3} mln km\nJulian Day: {:.4}",
								lon, odleglosc_km, jd
							);

							app.earth_angle = lon.to_radians() as f32;
							app.show_single_earth = true;
							app.show_multiple_earths = false;

							if let Ok(year) = app.input_from_timer.year.trim().parse::<f32>() {
								let years_from_2000 = year - 2000.0;
								app.visual_precession_angle = (years_from_2000 * 1.0).to_radians();
							}
						}
						Err(e) => {
							app.calc1_result = format!("Błąd formatu daty:\n{}", e);
						}
					}
				}
			});

			if !app.calc1_result.is_empty() {
				ui0.add_space(5.0);
				ui0.label(egui::RichText::new(&app.calc1_result).color(theme::SCENARIO_1_RESULT_COLOR));
			}

			// --- KOLUMNA 2: SCENARIUSZ 2 ---
			ui1.label("Scenariusz 2: 🌀 -> 🗓️");
			ui1.horizontal(|ui| {
				ui.add(egui::TextEdit::singleline(&mut app.input_from_point.year).desired_width(50.0));
				ui.label("Rok");
				ui.add(egui::TextEdit::singleline(&mut app.input_from_point.angle).desired_width(70.0));
				ui.label("Kąt (°)");
			});
			ui1.horizontal(|ui| {
				ui.label("Strefa:");
				ui.add(egui::TextEdit::singleline(&mut app.input_from_point.zone).desired_width(65.0));
			});

			ui1.add_space(5.0);

			ui1.horizontal(|ui| {
				let calc2_btn = egui::Button::new(
					egui::RichText::new("Oblicz 🗓️")
						.font(egui::FontId::proportional(theme::FONT_SIZE_BTN))
						.color(theme::TEXT_DEFAULT),
				)
				.min_size(egui::vec2(100.0, theme::BTN_HEIGHT))
				.fill(theme::BTN_ACTION_BG);

				if ui.add(calc2_btn).clicked() {
					let parsed_year = app.input_from_point.year.trim().parse::<i16>();
					let parsed_angle = app.input_from_point.angle.trim().parse::<f64>();

					match (parsed_year, parsed_angle) {
						(Ok(year), Ok(angle)) => {
							let (jd, distance_au) = find_time_and_distance_for_longitude(year, angle);
							let odleglosc_km = distance_au * 149.5978707;

							let unix_seconds = (jd - 2440587.5) * 86400.0;
							if let Ok(timestamp) = jiff::Timestamp::from_second(unix_seconds.round() as i64) {
								let tz: jiff::tz::TimeZone =
									jiff::tz::TimeZone::get(app.input_from_point.zone.trim())
										.unwrap_or(jiff::tz::TimeZone::UTC);
								let point_time = timestamp.to_zoned(tz);

								app.calc2_result = format!(
									"Znaleziona Data:\n{}\nOdległość: {:.3} mln km\nJulian Day: {:.4}",
									point_time.strftime("%Y-%m-%d %H:%M:%S"),
									odleglosc_km,
									jd
								);

								app.earth_angle = angle.to_radians() as f32;
								app.show_single_earth = true;
								app.show_multiple_earths = false;
								let years_from_2000 = year as f32 - 2000.0;
								app.visual_precession_angle = (years_from_2000 * 1.0).to_radians();
							}
						}
						_ => {
							app.calc2_result =
								"Błąd: Upewnij się, że Rok to liczba całkowita, a Kąt zawiera kropkę dziesiętną."
									.to_string();
						}
					}
				}
			});

			if !app.calc2_result.is_empty() {
				ui1.add_space(5.0);
				ui1.label(egui::RichText::new(&app.calc2_result).color(theme::SCENARIO_2_RESULT_COLOR));
			}

			// --- KOLUMNA 3: SCENARIUSZ 3 ---
			ui2.label("Scenariusz 3: Rok + Ilość Części -> Daty");
			ui2.horizontal(|ui| {
				ui.add(egui::TextEdit::singleline(&mut app.input_from_parts.year).desired_width(50.0));
				ui.label("Rok");
				ui.add(egui::TextEdit::singleline(&mut app.input_from_parts.parts).desired_width(50.0));
				ui.label("Części");
			});

			ui2.horizontal(|ui| {
				ui.label("Strefa:");
				ui.add(egui::TextEdit::singleline(&mut app.input_from_parts.zone).desired_width(65.0));
			});

			ui2.add_space(5.0);

			ui2.horizontal(|ui| {
				let calc3_btn = egui::Button::new(
					egui::RichText::new("Oblicz ⚙️")
						.font(egui::FontId::proportional(theme::FONT_SIZE_BTN))
						.color(theme::TEXT_DEFAULT),
				)
				.min_size(egui::vec2(100.0, theme::BTN_HEIGHT))
				.fill(theme::BTN_ACTION_BG);

				if ui.add(calc3_btn).clicked() {
					let parsed_year = app.input_from_parts.year.trim().parse::<i16>();
					let parsed_parts = app.input_from_parts.parts.trim().parse::<u64>();

					match (parsed_year, parsed_parts) {
						(Ok(year), Ok(parts)) => {
							let cycle = Cycle::new(parts, year);
							app.calc3_points.clear();

							for point in cycle.points.iter() {
								let label = format!("{}/{}", point.etykieta.0, point.etykieta.1);
								let info = format!(
									"[{}] {:.1}° -> {} {}",
									label, point.kat, point.data_miesieczna, point.godzina_utc
								);
								app.calc3_points.push(crate::app::PlottedPoint {
									label,
									info,
									angle_rad: point.kat.to_radians() as f32,
								});
							}

							app.show_single_earth = false;
							app.show_multiple_earths = true;

							if let Ok(y) = app.input_from_parts.year.trim().parse::<f32>() {
								let years_from_2000 = y - 2000.0;
								let prec_deg = 102.9 + years_from_2000 * (360.0 / 21000.0);
								app.visual_precession_angle = prec_deg.to_radians();
							}
						}
						_ => { /* Błąd */ }
					}
				}
			});

			if !app.calc3_points.is_empty() {
				ui2.add_space(5.0);
				for pt in &app.calc3_points {
					ui2.label(egui::RichText::new(&pt.info).color(theme::SCENARIO_3_RESULT_COLOR));
				}
			}
		}

		AppMode::Animation => {
			let mut iter = slots.iter_mut();
			let ui0 = iter.next().unwrap(); // Guzik
			let ui1 = iter.next().unwrap(); // Precesja
			let ui2 = iter.next().unwrap(); // Cykl 33
			let ui3 = iter.next().unwrap(); // Orbita

			ui0.heading("Panel Sterowania");
			ui0.label("Sterowanie Animacją");
			ui0.add_space(4.0);

			let (btn_text, btn_color) = if app.is_playing {
				("⏸ Pause", theme::BTN_INACTIVE_BG)
			} else {
				("▶ Play", theme::BTN_ACTIVE_BG)
			};

			let play_btn = egui::Button::new(
				egui::RichText::new(btn_text)
					.font(egui::FontId::proportional(theme::FONT_SIZE_BTN))
					.color(theme::TEXT_DEFAULT)
					.strong(),
			)
			.min_size(egui::vec2(140.0, theme::BTN_HEIGHT))
			.fill(btn_color);

			if ui0.add(play_btn).clicked() {
				app.is_playing = !app.is_playing;
			}

			ui1.heading("Precesja (Obrót Linii Apsyd)");
			ui1.label("• ~21 000 lat: Cykl wzgl. równonocy");
			ui1.label("• ~112 000 lat: Cykl wzgl. odległych gwiazd");
			ui1.label("• 19-23 tys. lat: Precesja klimatyczna");

			let mut prec_degrees = app.visual_precession_angle.to_degrees().rem_euclid(360.0);
			if ui1.add(egui::Slider::new(&mut prec_degrees, 0.0..=360.0).text("Kąt Precesji (°)")).changed() {
				app.visual_precession_angle = prec_degrees.to_radians();
			}
			ui1.label(format!("Cykl ułamkowy: {:.4}", prec_degrees / 360.0));

			ui2.heading("Cykl 33-letni");
			ui2.label("Zbieżność kalendarzowa / słoneczna");

			let mut c33_degrees = app.cycle_33_phase.to_degrees().rem_euclid(360.0);
			if ui2.add(egui::Slider::new(&mut c33_degrees, 0.0..=360.0).text("Faza cyklu (°)")).changed() {
				app.cycle_33_phase = c33_degrees.to_radians();
			}
			ui2.label(format!("Cykl ułamkowy: {:.4}", c33_degrees / 360.0));

			ui3.heading("Ruch orbitalny Ziemi");
			ui3.label("Pełny obrót wokół Słońca (1 rok)");

			let mut earth_degrees = app.earth_angle.to_degrees().rem_euclid(360.0);
			if ui3.add(egui::Slider::new(&mut earth_degrees, 0.0..=360.0).text("Pozycja Ziemi (°)")).changed() {
				app.earth_angle = earth_degrees.to_radians();
			}
			ui3.label(format!("Pozycja ułamkowa: {:.4}", earth_degrees / 360.0));
		}
	}
}
