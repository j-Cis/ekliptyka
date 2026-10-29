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
// ---------------------------------

#[derive(PartialEq)]
pub enum AppMode {
	BasicCalculations,
	Animation,
	PointTables,
}

pub struct EclipticaApp {
	pub mode: AppMode,
	pub is_playing: bool,

	pub visual_precession_angle: f32,
	pub earth_angle: f32,
	pub cycle_33_phase: f32,

	pub input_from_timer: InputTimer,
	pub input_from_point: InputPoint,

	pub calc1_result: String,
	pub calc2_result: String,
}

impl Default for EclipticaApp {
	fn default() -> Self {
		Self {
			mode: AppMode::BasicCalculations,
			is_playing: false,
			visual_precession_angle: 0.0,
			earth_angle: 0.0,
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

			calc1_result: String::new(),
			calc2_result: String::new(),
		}
	}
}

impl eframe::App for EclipticaApp {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
		egui::Panel::top("top_panel").show(ui, |ui| {
			ui.horizontal(|ui| {
				ui.selectable_value(&mut self.mode, AppMode::BasicCalculations, "Podstawowe (Kalkulator)");
				ui.selectable_value(&mut self.mode, AppMode::Animation, "Animacja");
				ui.selectable_value(&mut self.mode, AppMode::PointTables, "Wykaz Punktów (Tabela)");
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

		// Pętla animacji uwzględniająca flagę is_playing
		if self.mode == AppMode::Animation && self.is_playing {
			ui.ctx().request_repaint();
			self.earth_angle += 0.01;
			self.visual_precession_angle += 0.001;
			self.cycle_33_phase += 0.003;
		}
	}
}
