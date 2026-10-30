use eframe::egui;

use crate::gui::{
	app_frame::{AppLayout, LayoutFrame},
	canvas,
	controls,
};

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

#[derive(PartialEq)]
pub enum AppMode {
	BasicCalculations,
	Animation,
}

#[derive(Clone)]
pub struct PlottedPoint {
	pub label: String,
	pub info: String,
	pub angle_rad: f32,
}

pub struct EclipticaApp {
	pub mode: AppMode,
	pub is_playing: bool,
	pub show_bottom_panel: bool,
	pub debug_angle: bool,

	pub visual_precession_angle: f32,
	pub earth_angle: f32,
	pub show_single_earth: bool,
	pub show_multiple_earths: bool,
	pub cycle_33_phase: f32,

	pub input_from_timer: InputTimer,
	pub input_from_point: InputPoint,
	pub input_from_parts: InputParts,

	pub calc1_result: String,
	pub calc2_result: String,
	pub calc3_points: Vec<PlottedPoint>,
}

impl Default for EclipticaApp {
	fn default() -> Self {
		Self {
			mode: AppMode::Animation,
			is_playing: false,
			show_bottom_panel: true,
			debug_angle: false,
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
		let layout = AppLayout { show_panel: self.show_bottom_panel, ..Default::default() };

		let mut frame = layout.begin(ui);

		// 1. Zapełniamy Kanwę (góra)
		canvas::render_orbit(self, &mut frame.canvas_ui);

		// 2. Zapełniamy Chowany Panel z kontrolkami (środek)
		if let Some(mut drawer_ui) = frame.drawer_ui {
			egui::ScrollArea::vertical().auto_shrink([false; 2]).show(&mut drawer_ui, |scroll_ui| {
				controls::render_controls(self, scroll_ui);
			});
		}

		// 3. Zapełniamy Dolną Belkę używając NASZYCH, stylizowanych przycisków
		frame.nav_ui.horizontal_centered(|nav_ui| {
			nav_ui.add_space(8.0);

			let is_calc = self.mode == AppMode::BasicCalculations;
			if LayoutFrame::toggle_button(nav_ui, is_calc, "🧮 Kalkulator").clicked() {
				self.mode = AppMode::BasicCalculations;
			}

			let is_anim = self.mode == AppMode::Animation;
			if LayoutFrame::toggle_button(nav_ui, is_anim, "🌀 Animacja").clicked() {
				self.mode = AppMode::Animation;
			}

			nav_ui.separator();

			let panel_label = if self.show_bottom_panel { "🔽 Ukryj" } else { "🔼 Panel" };
			if LayoutFrame::toggle_button(nav_ui, self.show_bottom_panel, panel_label).clicked() {
				self.show_bottom_panel = !self.show_bottom_panel;
			}

			let debug_label = if self.debug_angle { "🛠 Debug: ON" } else { "🛠 Debug: OFF" };
			if LayoutFrame::toggle_button(nav_ui, self.debug_angle, debug_label).clicked() {
				self.debug_angle = !self.debug_angle;
			}
		});

		// Pętla animacji
		if self.mode == AppMode::Animation && self.is_playing {
			ui.ctx().request_repaint_after(std::time::Duration::from_millis(16));
			let tau = std::f32::consts::TAU;
			self.earth_angle = (self.earth_angle + 0.01).rem_euclid(tau);
			self.visual_precession_angle = (self.visual_precession_angle + 0.001).rem_euclid(tau);
			self.cycle_33_phase = (self.cycle_33_phase + 0.003).rem_euclid(tau);
		}
	}
}
