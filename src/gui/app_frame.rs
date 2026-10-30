use eframe::egui;

use crate::{app::AppMode, gui::theme};

// ----------------------------------------------------------------------------
// SUBMODUŁ: Abstrakt geometryczny (Vas / Fovea)
// ----------------------------------------------------------------------------
pub mod vas {
	use eframe::egui;

	/// Pomocniczy enum do precyzyjnego opisu rozpiętości (piksele lub procenty)
	#[derive(Debug, Clone, Default)]
	pub enum Spatium {
		Pixel(f32),
		Percent(f32),
		#[default]
		Auto,
	}

	#[derive(Debug, Clone)]
	pub struct Fovea {
		/// Deklarowana rozpiętość (np. w procentach lub pikselach)
		pub spatium: Spatium,

		/// Numer w kolejności (indeks gniazda na osi)
		pub numerus: usize,

		/// Może się ukrywać (flaga widoczności / toggle)
		pub latens: bool,

		/// Może się kurczyć (współczynnik kurczenia / shrink)
		pub contractio: f32,

		/// Może się rozszerzać (współczynnik rośnięcia / grow)
		pub amplificatio: f32,

		/// Ma wypełnić pozostałą przestrzeń (flaga typu flex-fill /
		/// auto-stretch)
		pub repletio: bool,
	}

	impl Default for Fovea {
		fn default() -> Self {
			Self {
				spatium: Spatium::Auto,
				numerus: 0,
				latens: false,
				contractio: 0.0,
				amplificatio: 0.0,
				repletio: false,
			}
		}
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum Redundantia {
		/// Elementy wewnątrz ramy zmniejszają swój wymiar poprzeczny,
		/// aby cała zawartość idealnie upchnęła się w dostępnej przestrzeni
		/// (Ściskanie / Shrink).
		Compressio,

		/// Elementy zachowują oryginalny rozmiar i wykraczają poza krawędź
		/// ramy, umożliwiając użytkownikowi płynne przesuwanie widoku
		/// wzdłuż osi (Przewijanie / Scroll).
		Volutatio,

		/// Zawartość, która nie mieści się w granicach kontenera,
		/// zostaje twardo obcięta dokładnie na linii jego krawędzi (Ucięcie /
		/// Clip).
		Truncatio,

		/// Strumień elementów po napotkaniu krawędzi kontenera załamuje się,
		/// przenosząc nadmiarowe gniazda do nowo utworzonego rzędu lub linii
		/// (Zawijanie / Wrap).
		Flexio,

		/// Nadmiarowa zawartość zostaje pogrupowana w osobne, pełnowymiarowe
		/// ekrany, umożliwiając przełączanie się między nimi jak między
		/// kartami w przeglądarce (Stronicowanie / Tabs).
		Paginatio,
	}

	/// Wspólny rdzeń geometryczny i strukturalny dla każdego kontenera.
	pub struct Vas {
		/// Orientacja osi głównej: true = pozioma, false = pionowa.
		pub axis_maior_horizontalis: bool,

		/// Wektor ruchu na osi głównej: false = Directus, true = Reversus.
		pub axis_maior_reversus: bool,

		/// Strategia obsługi przepełnienia (overflow) na osi głównej.
		pub redundantia_maior: Redundantia,

		/// Pakiet wartości geometrycznych layoutu (.0: p_am, .1: p_cm, .2:
		/// p_at, .3: p_ct, .4: gap)
		pub padding_am_cm_at_ct_gap: (f32, f32, f32, f32, f32),

		/// Zablokowana sekwencja gniazd ułożonych wzdłuż osi głównej.
		pub foveae: Box<[Fovea]>,
	}

	impl Vas {
		/// Oblicza całkowitą wymaganą rozpiętość zawartości na osi głównej
		pub fn total_content_length(&self, rect: egui::Rect) -> f32 {
			let (am, cm, _at, _ct, gap) = self.padding_am_cm_at_ct_gap;
			let visible_foveae: Vec<&Fovea> = self.foveae.iter().filter(|f| !f.latens).collect();
			if visible_foveae.is_empty() {
				return 0.0;
			}
			let total_gap = gap * (visible_foveae.len() - 1) as f32;
			let main_length = if self.axis_maior_horizontalis {
				(rect.width() - am - cm).max(0.0)
			} else {
				(rect.height() - am - cm).max(0.0)
			};
			let available_length = (main_length - total_gap).max(0.0);

			let mut allocated = 0.0;
			let mut auto_count = 0;
			for fovea in &visible_foveae {
				match fovea.spatium {
					Spatium::Pixel(px) => allocated += px,
					Spatium::Percent(pct) => allocated += available_length * (pct / 100.0),
					Spatium::Auto => auto_count += 1,
				}
			}

			if auto_count > 0 {
				// W Volutatio minimalny rozmiar gniazda Auto to 115 px (miejsce
				// na ikonę i tekst bez łamania wiersza)
				let min_auto = if self.redundantia_maior == Redundantia::Volutatio { 115.0 } else { 0.0 };
				let remaining = (available_length - allocated).max(0.0);
				let auto_size = (remaining / auto_count as f32).max(min_auto);
				allocated += auto_size * auto_count as f32;
			}

			allocated + total_gap + am + cm
		}

		pub fn resolve(&self, rect: egui::Rect) -> Vec<(usize, egui::Rect)> { self.resolve_scroll(rect, 0.0) }

		pub fn resolve_scroll(&self, rect: egui::Rect, scroll_offset: f32) -> Vec<(usize, egui::Rect)> {
			let (am, cm, at, ct, gap) = self.padding_am_cm_at_ct_gap;

			let mut inner_rect = rect;
			if self.axis_maior_horizontalis {
				inner_rect.min.x += am;
				inner_rect.max.x -= cm;
				inner_rect.min.y += at;
				inner_rect.max.y -= ct;
			} else {
				inner_rect.min.y += am;
				inner_rect.max.y -= cm;
				inner_rect.min.x += at;
				inner_rect.max.x -= ct;
			}

			let visible_foveae: Vec<&Fovea> = self.foveae.iter().filter(|f| !f.latens).collect();
			if visible_foveae.is_empty() {
				return vec![];
			}

			let total_gap = gap * (visible_foveae.len() - 1) as f32;
			let main_length = if self.axis_maior_horizontalis { inner_rect.width() } else { inner_rect.height() };
			let cross_length = if self.axis_maior_horizontalis { inner_rect.height() } else { inner_rect.width() };

			let available_length = (main_length - total_gap).max(0.0);

			let mut sizes = vec![0.0; visible_foveae.len()];
			let mut allocated_length = 0.0;
			let mut auto_count = 0;

			for (i, fovea) in visible_foveae.iter().enumerate() {
				match fovea.spatium {
					Spatium::Pixel(px) => {
						sizes[i] = px;
						allocated_length += px;
					}
					Spatium::Percent(pct) => {
						let s = available_length * (pct / 100.0);
						sizes[i] = s;
						allocated_length += s;
					}
					Spatium::Auto => {
						auto_count += 1;
					}
				}
			}

			let remaining_length = (available_length - allocated_length).max(0.0);
			if auto_count > 0 {
				// W Volutatio gniazda zachowują oryginalny rozmiar i NIE MOGĄ
				// skurczyć się poniżej wymaganych 115 px
				let min_auto = if self.redundantia_maior == Redundantia::Volutatio { 115.0 } else { 0.0 };
				let auto_size = (remaining_length / auto_count as f32).max(min_auto);
				for (i, fovea) in visible_foveae.iter().enumerate() {
					if matches!(fovea.spatium, Spatium::Auto) {
						sizes[i] = auto_size;
						allocated_length += auto_size;
					}
				}
			}

			// WYŁĄCZNIE Compressio ma prawo zmniejszać wymiary, aby upchnąć
			// zawartość
			if allocated_length > available_length && self.redundantia_maior == Redundantia::Compressio {
				let scale = available_length / allocated_length;
				for size in &mut sizes {
					*size *= scale;
				}
			}

			let mut results = Vec::new();

			// W Volutatio punkt startowy jest przesuwany o scroll_offset
			let mut current_pos = if self.axis_maior_reversus {
				if self.axis_maior_horizontalis {
					inner_rect.max.x + scroll_offset
				} else {
					inner_rect.max.y + scroll_offset
				}
			} else {
				if self.axis_maior_horizontalis {
					inner_rect.min.x - scroll_offset
				} else {
					inner_rect.min.y - scroll_offset
				}
			};

			for (i, &size) in sizes.iter().enumerate() {
				let fovea = visible_foveae[i];

				let slot_rect = if self.axis_maior_horizontalis {
					let (min_x, max_x) = if self.axis_maior_reversus {
						(current_pos - size, current_pos)
					} else {
						(current_pos, current_pos + size)
					};
					egui::Rect::from_min_max(
						egui::pos2(min_x, inner_rect.min.y),
						egui::pos2(max_x, inner_rect.min.y + cross_length),
					)
				} else {
					let (min_y, max_y) = if self.axis_maior_reversus {
						(current_pos - size, current_pos)
					} else {
						(current_pos, current_pos + size)
					};
					egui::Rect::from_min_max(
						egui::pos2(inner_rect.min.x, min_y),
						egui::pos2(inner_rect.min.x + cross_length, max_y),
					)
				};

				results.push((fovea.numerus, slot_rect));

				if self.axis_maior_reversus {
					current_pos -= size + gap;
				} else {
					current_pos += size + gap;
				}
			}

			results
		}
	}
}

// ----------------------------------------------------------------------------
// WYKORZYSTANIE SILNIKA W RAMCE APLIKACJI
// ----------------------------------------------------------------------------
use vas::{Fovea, Redundantia, Spatium, Vas};

pub struct LayoutFrame {
	pub canvas_ui: egui::Ui,
	pub panel_slots: Option<Vec<egui::Ui>>,
	pub nav_slots: Vec<egui::Ui>,
}

impl LayoutFrame {
	pub fn toggle_button(ui: &mut egui::Ui, active: bool, text: &str) -> egui::Response {
		let bg = if active { theme::BTN_ACTIVE_BG } else { theme::BTN_INACTIVE_BG };
		let button = egui::Button::new(
			egui::RichText::new(text).font(egui::FontId::proportional(theme::FONT_SIZE_BTN)).color(theme::TEXT_DEFAULT),
		)
		.wrap_mode(egui::TextWrapMode::Extend) // ZAKAZ ŁAMANIA WIERSZA W GUZIKACH
		.min_size(egui::vec2(0.0, theme::BTN_HEIGHT))
		.fill(bg);

		ui.add_sized(ui.available_size(), button)
	}
}

pub struct AppLayout {
	pub show_panel: bool,
	pub mode: AppMode,
}

impl AppLayout {
	pub fn apply_theme(&self, ctx: &egui::Context) {
		let themes = [egui::Theme::Dark, egui::Theme::Light];
		for theme in themes {
			let mut style = (*ctx.style_of(theme)).clone();

			style.visuals.selection.bg_fill = theme::BTN_ACTIVE_BG;
			style.visuals.selection.stroke = egui::Stroke::new(1.0, theme::BTN_ACTIVE_BG);
			style.visuals.override_text_color = Some(theme::TEXT_DEFAULT);
			style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, theme::SEPARATOR_COLOR);
			style.visuals.extreme_bg_color = theme::INPUT_BG;

			style.visuals.widgets.noninteractive.weak_bg_fill = theme::SLIDER_INPUT_BG;
			style.visuals.widgets.inactive.weak_bg_fill = theme::SLIDER_INPUT_BG;
			style.visuals.widgets.hovered.weak_bg_fill = theme::SLIDER_INPUT_BG;
			style.visuals.widgets.active.weak_bg_fill = theme::SLIDER_INPUT_BG;

			style.visuals.widgets.inactive.bg_fill = theme::SLIDER_RAIL_COLOR;
			style.visuals.widgets.hovered.bg_fill = theme::SLIDER_RAIL_COLOR;
			style.visuals.widgets.active.bg_fill = theme::SLIDER_HANDLE_COLOR;
			style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.5, theme::SLIDER_HANDLE_BORDER);

			style.visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(4);
			style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(4);
			style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(4);
			style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(4);

			style.text_styles.insert(egui::TextStyle::Body, egui::FontId::proportional(theme::FONT_SIZE_LABEL));
			style.text_styles.insert(egui::TextStyle::Button, egui::FontId::proportional(theme::FONT_SIZE_BTN));
			style
				.text_styles
				.insert(egui::TextStyle::Heading, egui::FontId::proportional(theme::FONT_SIZE_SECTION_TITLE));

			style.spacing.interact_size.y = theme::INPUT_HEIGHT;

			ctx.set_style_of(theme, style);
		}
	}

	pub fn begin(&self, ui: &mut egui::Ui) -> LayoutFrame {
		self.apply_theme(ui.ctx());
		let full_rect = ui.available_rect_before_wrap();

		// =========================================================
		// A) "VAS" główny root (Compressio - ściskanie pionowe)
		// =========================================================
		let vas_root = Vas {
			axis_maior_horizontalis: false,
			axis_maior_reversus: true,
			redundantia_maior: Redundantia::Compressio,
			padding_am_cm_at_ct_gap: (0.0, 0.0, 0.0, 0.0, 0.0),
			foveae: Box::new([
				Fovea {
					numerus: 1,
					spatium: Spatium::Pixel(theme::BOTTOM_BAR_HEIGHT),
					contractio: 0.0,
					amplificatio: 0.0,
					repletio: false,
					latens: false,
				},
				Fovea {
					numerus: 2,
					spatium: Spatium::Pixel(theme::PANEL_HEIGHT),
					contractio: 1.0,
					amplificatio: 0.0,
					repletio: false,
					latens: !self.show_panel,
				},
				Fovea {
					numerus: 3,
					spatium: Spatium::Auto,
					contractio: 1.0,
					amplificatio: 1.0,
					repletio: true,
					latens: false,
				},
			]),
		};

		let root_rects = vas_root.resolve(full_rect);
		let rect_nav = root_rects.iter().find(|(n, _)| *n == 1).unwrap().1;
		let rect_panel = root_rects.iter().find(|(n, _)| *n == 2).map(|(_, r)| *r);
		let rect_canvas = root_rects.iter().find(|(n, _)| *n == 3).unwrap().1;

		// =========================================================
		// B) "VAS" Belka pozioma (Volutatio - zachowanie rozmiarów i scroll)
		// =========================================================
		ui.painter().rect_filled(rect_nav, 0.0, theme::BOTTOM_BAR_BG);
		ui.painter()
			.line_segment([rect_nav.left_top(), rect_nav.right_top()], egui::Stroke::new(1.0, theme::SEPARATOR_COLOR));

		let vas_nav = Vas {
			axis_maior_horizontalis: true,
			axis_maior_reversus: false,
			redundantia_maior: Redundantia::Volutatio,
			padding_am_cm_at_ct_gap: (10.0, 10.0, 4.0, 4.0, 8.0),
			foveae: Box::new([
				Fovea { numerus: 0, spatium: Spatium::Auto, ..Default::default() },
				Fovea { numerus: 1, spatium: Spatium::Auto, ..Default::default() },
				Fovea { numerus: 2, spatium: Spatium::Auto, ..Default::default() },
				Fovea { numerus: 3, spatium: Spatium::Auto, ..Default::default() },
				Fovea { numerus: 4, spatium: Spatium::Pixel(30.0), ..Default::default() },
				Fovea { numerus: 5, spatium: Spatium::Pixel(30.0), ..Default::default() },
				Fovea { numerus: 6, spatium: Spatium::Pixel(20.0), ..Default::default() },
			]),
		};

		// Obsługa Volutatio: Obliczanie przewijania i interakcja gestem/myszą
		// bez blokowania kliknięć
		let total_nav_width = vas_nav.total_content_length(rect_nav);
		let max_scroll = (total_nav_width - rect_nav.width()).max(0.0);

		let nav_scroll_id = ui.scope_id().with("vas_nav_volutatio_offset");
		let mut scroll_x = ui.ctx().data_mut(|d| *d.get_temp_mut_or_default::<f32>(nav_scroll_id));

		let is_pointer_down = ui.input(|i| i.pointer.button_down(egui::PointerButton::Primary));
		let pointer_pos = ui.input(|i| i.pointer.latest_pos());
		if is_pointer_down && pointer_pos.is_some_and(|pos| rect_nav.contains(pos)) {
			let delta_x = ui.input(|i| i.pointer.delta().x);
			scroll_x -= delta_x;
		}
		if pointer_pos.is_some_and(|pos| rect_nav.contains(pos)) {
			let wheel = ui.input(|i| i.smooth_scroll_delta.x + i.smooth_scroll_delta.y);
			scroll_x -= wheel;
		}
		scroll_x = scroll_x.clamp(0.0, max_scroll);
		ui.ctx().data_mut(|d| *d.get_temp_mut_or_default::<f32>(nav_scroll_id) = scroll_x);

		let mut nav_slots = Vec::new();
		for (_, r) in vas_nav.resolve_scroll(rect_nav, scroll_x) {
			let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r));
			child.set_clip_rect(rect_nav);
			nav_slots.push(child);
		}

		// =========================================================
		// C i D) "VAS" Panel opcji (w Gnieździe 2)
		// =========================================================
		let panel_slots = if let Some(r_panel) = rect_panel {
			ui.painter().rect_filled(r_panel, 0.0, theme::PANEL_BG);
			ui.painter().line_segment(
				[r_panel.left_top(), r_panel.right_top()],
				egui::Stroke::new(1.0, theme::SEPARATOR_COLOR),
			);

			let mut slots = Vec::new();

			if self.mode == AppMode::BasicCalculations {
				let vas_calc = Vas {
					axis_maior_horizontalis: true,
					axis_maior_reversus: false,
					redundantia_maior: Redundantia::Paginatio,
					padding_am_cm_at_ct_gap: (10.0, 10.0, 10.0, 10.0, 20.0),
					foveae: Box::new([
						Fovea { numerus: 0, spatium: Spatium::Auto, ..Default::default() },
						Fovea { numerus: 1, spatium: Spatium::Auto, ..Default::default() },
						Fovea { numerus: 2, spatium: Spatium::Auto, ..Default::default() },
					]),
				};
				for (_, r) in vas_calc.resolve(r_panel) {
					let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r));
					child.set_clip_rect(r);
					slots.push(child);
				}
			} else {
				let vas_anim = Vas {
					axis_maior_horizontalis: true,
					axis_maior_reversus: true,
					redundantia_maior: Redundantia::Flexio,
					padding_am_cm_at_ct_gap: (10.0, 10.0, 10.0, 10.0, 20.0),
					foveae: Box::new([
						Fovea { numerus: 0, spatium: Spatium::Auto, ..Default::default() },
						Fovea { numerus: 1, spatium: Spatium::Auto, ..Default::default() },
						Fovea { numerus: 2, spatium: Spatium::Auto, ..Default::default() },
						Fovea { numerus: 3, spatium: Spatium::Auto, ..Default::default() },
					]),
				};
				for (_, r) in vas_anim.resolve(r_panel) {
					let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r));
					child.set_clip_rect(r);
					slots.push(child);
				}
			}
			Some(slots)
		} else {
			None
		};

		// =========================================================
		// KANWA (w Gnieździe 3)
		// =========================================================
		ui.painter().rect_filled(rect_canvas, 0.0, theme::CANVAS_BG);
		let mut canvas_ui = ui.new_child(egui::UiBuilder::new().max_rect(rect_canvas));
		canvas_ui.set_clip_rect(rect_canvas);

		LayoutFrame { canvas_ui, panel_slots, nav_slots }
	}
}
