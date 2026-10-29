use eframe::egui;

use crate::gui::{canvas, controls};

#[derive(PartialEq)]
pub enum AppMode {
	BasicCalculations,
	Animation,
	PointTables,
}

pub struct EclipticaApp {
	pub mode: AppMode,
	pub visual_precession_angle: f32,
	pub earth_angle: f32,

	pub input_year: String,
	pub input_month: String,
	pub input_day: String,
	pub input_hour: String,
	pub input_minute: String,
	pub input_zone: String,

	// NOWE: Pole na przechowywanie wyniku obliczeń
	pub calc1_result: String,

	pub is_playing: bool,
}

impl Default for EclipticaApp {
	fn default() -> Self {
		Self {
			mode: AppMode::BasicCalculations,
			visual_precession_angle: 0.0,
			earth_angle: 0.0,

			input_year: "1990".to_string(),
			input_month: "04".to_string(),
			input_day: "07".to_string(),
			input_hour: "00".to_string(),
			input_minute: "02".to_string(),
			input_zone: "+02:00".to_string(),

			calc1_result: String::new(),

			is_playing: false,
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

		egui::Panel::left("left_panel").resizable(true).default_size(300.0).show(ui, |ui| {
			controls::render_controls(self, ui);
		});

		egui::CentralPanel::default().show(ui, |ui| {
			canvas::render_orbit(self, ui);
		});

		if self.mode == AppMode::Animation {
			ui.ctx().request_repaint();
			self.earth_angle += 0.01;
			self.visual_precession_angle += 0.001;
		}
	}
}
