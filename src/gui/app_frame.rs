use eframe::egui;
use crate::{app::AppMode, gui::theme};

// ----------------------------------------------------------------------------
// SUBMODUŁ: Abstrakt geometryczny (Vas / Fovea)
// ----------------------------------------------------------------------------
pub mod vas {
    use eframe::egui;

    #[derive(Debug, Clone, Default)]
    pub enum Spatium {
        Pixel(f32),
        Percent(f32),
        #[default]
        Auto,
    }

    // impl Default for Spatium {
    //     fn default() -> Self {
    //         Self::Auto
    //     }
    // }

    #[derive(Debug, Clone)]
    pub struct Fovea {
        pub spatium: Spatium,
        pub numerus: usize,
        pub latens: bool,
        pub contractio: f32,
        pub amplificatio: f32,
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
        Compressio,
        Volutatio,
        Truncatio,
        Flexio,
        Paginatio,
    }

    pub struct Vas {
        pub axis_maior_horizontalis: bool,
        pub axis_maior_reversus: bool,
        pub redundantia_maior: Redundantia,
        pub padding_am_cm_at_ct_gap: (f32, f32, f32, f32, f32),
        pub foveae: Box<[Fovea]>,
    }

    impl Vas {
        pub fn resolve(&self, rect: egui::Rect) -> Vec<(usize, egui::Rect)> {
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
                    Spatium::Pixel(px) => { sizes[i] = px; allocated_length += px; }
                    Spatium::Percent(pct) => { let s = available_length * (pct / 100.0); sizes[i] = s; allocated_length += s; }
                    Spatium::Auto => { auto_count += 1; }
                }
            }

            let remaining_length = (available_length - allocated_length).max(0.0);
            if auto_count > 0 {
                let auto_size = remaining_length / auto_count as f32;
                for (i, fovea) in visible_foveae.iter().enumerate() {
                    if matches!(fovea.spatium, Spatium::Auto) {
                        sizes[i] = auto_size;
                        allocated_length += auto_size;
                    }
                }
            }

            if allocated_length > available_length && self.redundantia_maior == Redundantia::Compressio {
                let scale = available_length / allocated_length;
                for size in &mut sizes { *size *= scale; }
            }

            let mut results = Vec::new();
            
            let mut current_pos = if self.axis_maior_reversus {
                if self.axis_maior_horizontalis { inner_rect.max.x } else { inner_rect.max.y }
            } else {
                if self.axis_maior_horizontalis { inner_rect.min.x } else { inner_rect.min.y }
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
    pub panel_slots: Option<Vec<egui::Ui>>, // 3 lub 4 gniazda zależnie od trybu
    pub nav_slots: Vec<egui::Ui>,           // 7 gniazd belki
}

impl LayoutFrame {
    pub fn toggle_button(ui: &mut egui::Ui, active: bool, text: &str) -> egui::Response {
        let bg = if active { theme::BTN_ACTIVE_BG } else { theme::BTN_INACTIVE_BG };
        let button = egui::Button::new(
            egui::RichText::new(text).font(egui::FontId::proportional(theme::FONT_SIZE_BTN)).color(theme::TEXT_DEFAULT),
        )
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
            style.text_styles.insert(egui::TextStyle::Heading, egui::FontId::proportional(theme::FONT_SIZE_SECTION_TITLE));
            
            style.spacing.interact_size.y = theme::INPUT_HEIGHT;
            
            ctx.set_style_of(theme, style);
        }
    }

    pub fn begin(&self, ui: &mut egui::Ui) -> LayoutFrame {
        self.apply_theme(ui.ctx());
        let full_rect = ui.available_rect_before_wrap();

        // =========================================================
        // A) "VAS" główny root
        // =========================================================
        let vas_root = Vas {
            axis_maior_horizontalis: false,
            axis_maior_reversus: true, // Od dołu do góry
            redundantia_maior: Redundantia::Compressio,
            padding_am_cm_at_ct_gap: (0.0, 0.0, 0.0, 0.0, 0.0),
            foveae: Box::new([
                // Gniazdo 1: Belka dolna (najpierw, bo reversus = true buduje od dołu)
                Fovea { numerus: 1, spatium: Spatium::Pixel(theme::BOTTOM_BAR_HEIGHT), contractio: 0.0, amplificatio: 0.0, repletio: false, latens: false },
                // Gniazdo 2: Panel Opcji
                Fovea { numerus: 2, spatium: Spatium::Pixel(theme::PANEL_HEIGHT), contractio: 1.0, amplificatio: 0.0, repletio: false, latens: !self.show_panel },
                // Gniazdo 3: Kanwa
                Fovea { numerus: 3, spatium: Spatium::Auto, contractio: 1.0, amplificatio: 1.0, repletio: true, latens: false },
            ]),
        };

        let root_rects = vas_root.resolve(full_rect);
        let rect_nav    = root_rects.iter().find(|(n, _)| *n == 1).unwrap().1;
        let rect_panel  = root_rects.iter().find(|(n, _)| *n == 2).map(|(_, r)| *r);
        let rect_canvas = root_rects.iter().find(|(n, _)| *n == 3).unwrap().1;

        // =========================================================
        // B) "VAS" Belka pozioma (w Gnieździe 1)
        // =========================================================
        ui.painter().rect_filled(rect_nav, 0.0, theme::BOTTOM_BAR_BG);
        ui.painter().line_segment([rect_nav.left_top(), rect_nav.right_top()], egui::Stroke::new(1.0, theme::SEPARATOR_COLOR));

        let vas_nav = Vas {
            axis_maior_horizontalis: true,
            axis_maior_reversus: false,
            redundantia_maior: Redundantia::Volutatio,
            padding_am_cm_at_ct_gap: (10.0, 10.0, 4.0, 4.0, 8.0),
            foveae: Box::new([
                Fovea { numerus: 0, spatium: Spatium::Auto, ..Default::default() }, // Kalkulator
                Fovea { numerus: 1, spatium: Spatium::Auto, ..Default::default() }, // Animacja
                Fovea { numerus: 2, spatium: Spatium::Auto, ..Default::default() }, // Ukryj
                Fovea { numerus: 3, spatium: Spatium::Auto, ..Default::default() }, // Debug
                Fovea { numerus: 4, spatium: Spatium::Pixel(30.0), ..Default::default() }, // github web
                Fovea { numerus: 5, spatium: Spatium::Pixel(30.0), ..Default::default() }, // github repo
                Fovea { numerus: 6, spatium: Spatium::Pixel(20.0), ..Default::default() }, // localhost
            ]),
        };

        let mut nav_slots = Vec::new();
        for (_, r) in vas_nav.resolve(rect_nav) {
            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r));
            child.set_clip_rect(r);
            nav_slots.push(child);
        }

        // =========================================================
        // C i D) "VAS" Panel opcji (w Gnieździe 2)
        // =========================================================
        let panel_slots = if let Some(r_panel) = rect_panel {
            ui.painter().rect_filled(r_panel, 0.0, theme::PANEL_BG);
            ui.painter().line_segment([r_panel.left_top(), r_panel.right_top()], egui::Stroke::new(1.0, theme::SEPARATOR_COLOR));

            let mut slots = Vec::new();

            if self.mode == AppMode::BasicCalculations {
                // C) VAS dla Kalkulatora
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
                // D) VAS dla Animacji
                let vas_anim = Vas {
                    axis_maior_horizontalis: true,
                    axis_maior_reversus: true, // Od prawej do lewej
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

        LayoutFrame {
            canvas_ui,
            panel_slots,
            nav_slots,
        }
    }
}