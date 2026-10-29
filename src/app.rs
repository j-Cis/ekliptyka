use eframe::egui;

use crate::gui::{canvas, controls};

pub struct InputTimer {
	pub year: String,
	pub month: String,
	pub day: String,
	pub hour: String,
	pub minute: String,
	pub zone: String,
}

pub struct InputPoint {
	pub year: String,
	pub angle: String,
	pub zone: String,
}

pub struct InputParts {
	pub year: String,
	pub parts: String,
	pub zone: String,
}
// ---------------------------------

#[derive(PartialEq)]
pub enum AppMode {
	BasicCalculations,
	Animation,
}

// Struktura przechowująca pojedynczy punkt do narysowania
#[derive(Clone)]
pub struct PlottedPoint {
	pub label: String,  // etykieta UI (np. "1/4")
	pub info: String,   // szczegóły w panelu
	pub angle_rad: f32, // kąt na płótnie
}

pub struct EclipticaApp {
	pub mode: AppMode,
	pub is_playing: bool,

	pub visual_precession_angle: f32,
	// Główna Ziemia (Scenariusz 1, 2 i Animacja)
	pub earth_angle: f32,
	pub show_single_earth: bool,
	pub show_multiple_earths: bool,
	pub cycle_33_phase: f32,

	pub input_from_timer: InputTimer,
	pub input_from_point: InputPoint,
	pub input_from_parts: InputParts,

	pub calc1_result: String,
	pub calc2_result: String,
	// Lista wygenerowanych Ziemi z etykietami
	pub calc3_points: Vec<PlottedPoint>,
}

impl Default for EclipticaApp {
	fn default() -> Self {
		Self {
			mode: AppMode::BasicCalculations,
			is_playing: false,
			visual_precession_angle: 0.0,
			earth_angle: 0.0,
			show_single_earth: true,
			show_multiple_earths: false,
			cycle_33_phase: 0.0,

			input_from_timer: InputTimer {
				year: "1990".to_string(),
				month: "04".to_string(),
				day: "07".to_string(),
				hour: "00".to_string(),
				minute: "02".to_string(),
				zone: "+02:00".to_string(),
			},
			input_from_point: InputPoint {
				year: "2026".to_string(),
				angle: "0.0".to_string(),
				zone: "+02:00".to_string(),
			},
			input_from_parts: InputParts {
				year: "2026".to_string(),
				parts: "4".to_string(),
				zone: "+00:00".to_string(),
			},

			calc1_result: String::new(),
			calc2_result: String::new(),
			calc3_points: Vec::new(),
		}
	}
}

impl eframe::App for EclipticaApp {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
		egui::Panel::top("top_panel").show(ui, |ui| {
			ui.horizontal(|ui| {
				ui.selectable_value(&mut self.mode, AppMode::BasicCalculations, "Kalkulator");
				ui.selectable_value(&mut self.mode, AppMode::Animation, "Animacja");
				if self.mode == AppMode::Animation && self.is_playing {
					ui.ctx().request_repaint();
					self.earth_angle += 0.05; // Prędkość Ziemi
					self.visual_precession_angle += 0.005; // Prędkość precesji
				}
			});
		});

		egui::Panel::left("left_panel").resizable(true).default_size(350.0).show(ui, |ui| {
			controls::render_controls(self, ui);
		});

		egui::CentralPanel::default().show(ui, |ui| {
			canvas::render_orbit(self, ui);
		});

		// Bezpieczna pętla animacji
		if self.mode == AppMode::Animation && self.is_playing {
			// Zamiast rysować bez limitu, wymuszamy max ~60 klatek na sekundę
			// (16ms)
			ui.ctx().request_repaint_after(std::time::Duration::from_millis(16));

			// Bezpieczne zapętlanie wartości (zapobiega przepełnieniu floatów)
			let tau = std::f32::consts::TAU;
			self.earth_angle = (self.earth_angle + 0.01).rem_euclid(tau);
			self.visual_precession_angle = (self.visual_precession_angle + 0.001).rem_euclid(tau);
			self.cycle_33_phase = (self.cycle_33_phase + 0.003).rem_euclid(tau);
		}
	}
}
