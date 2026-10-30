use eframe::egui;
use crate::gui::theme;

/// Czysty, generyczny layout trójdzielny (Canvas + Chowany Drawer + Pasek dolny)
/// odpowiadający za spójne reguły stylizacji i podział przestrzeni.
pub struct AppLayout {
    pub show_panel: bool,
    pub panel_height: f32,
    pub bottom_bar_height: f32,
    pub canvas_bg: egui::Color32,
    pub panel_bg: egui::Color32,
    pub bottom_bar_bg: egui::Color32,
    pub border_color: egui::Color32,
}

impl Default for AppLayout {
    fn default() -> Self {
        Self {
            show_panel: true,
            panel_height: theme::PANEL_HEIGHT,
            bottom_bar_height: theme::BOTTOM_BAR_HEIGHT,
            canvas_bg: theme::CANVAS_BG,
            panel_bg: theme::PANEL_BG,
            bottom_bar_bg: theme::BOTTOM_BAR_BG,
            border_color: theme::SEPARATOR_COLOR,
        }
    }
}

/// Uchwyt do slotów layoutu z gotowymi metodami pomocniczymi do wstawiania kontrolek
pub struct LayoutFrame {
    pub canvas_ui: egui::Ui,
    pub drawer_ui: Option<egui::Ui>,
    pub nav_ui: egui::Ui,
}

impl LayoutFrame {
    /// Pomocnik wstawiający wystylizowany przycisk typu toggle zgodny ze stałymi z theme.rs
    pub fn toggle_button(ui: &mut egui::Ui, active: bool, text: &str) -> egui::Response {
        let bg = if active {
            theme::BTN_ACTIVE_BG
        } else {
            theme::BTN_INACTIVE_BG
        };

        let button = egui::Button::new(
            egui::RichText::new(text)
                .font(egui::FontId::proportional(theme::FONT_SIZE_BTN))
                .color(theme::TEXT_DEFAULT),
        )
        .min_size(egui::vec2(0.0, theme::BTN_HEIGHT))
        .fill(bg);

        ui.add(button)
    }
}

impl AppLayout {
    /// Aplikuje globalne reguły stylistyczne z theme.rs do kontekstu egui (0.37.0 API)
    pub fn apply_theme(&self, ctx: &egui::Context) {
        // W egui 0.37 style są przypisane do konkretnych motywów (Theme).
        // Aplikujemy nasze style do obu wariantów, by zignorować ustawienia OS użytkownika.
        let themes = [egui::Theme::Dark, egui::Theme::Light];
        
        for theme in themes {
            // Pobieramy obecny styl dla danego motywu i klonujemy go do edycji
            let mut style = (*ctx.style_of(theme)).clone();


            // 1. Zmiany właściwości wizualnych

						// Zaznaczenia globalne (np. tekst, kursor) w kolorze aktywnego guzika
style.visuals.selection.bg_fill = theme::BTN_ACTIVE_BG;
style.visuals.selection.stroke = egui::Stroke::new(1.0, theme::BTN_ACTIVE_BG);


            // Tekst i separatory
            style.visuals.override_text_color = Some(theme::TEXT_DEFAULT);
            style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, theme::SEPARATOR_COLOR);

            // Pola tekstowe (Inputy)
            style.visuals.extreme_bg_color = theme::INPUT_BG;

						// Tła dla wartości liczbowych suwaków (DragValue) i pomniejszych kontrolek
            style.visuals.widgets.noninteractive.weak_bg_fill = theme::SLIDER_INPUT_BG;
            style.visuals.widgets.inactive.weak_bg_fill = theme::SLIDER_INPUT_BG;
            style.visuals.widgets.hovered.weak_bg_fill = theme::SLIDER_INPUT_BG;
            style.visuals.widgets.active.weak_bg_fill = theme::SLIDER_INPUT_BG;

            // Suwaki (Sliders)
            style.visuals.widgets.inactive.bg_fill = theme::SLIDER_RAIL_COLOR;
            style.visuals.widgets.hovered.bg_fill = theme::SLIDER_RAIL_COLOR;
            style.visuals.widgets.active.bg_fill = theme::SLIDER_HANDLE_COLOR;
            style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.5, theme::SLIDER_HANDLE_BORDER);

            // 2. Zaokrąglenia widgetów - Egui 0.37.0 używa `CornerRadius` z modułu epaint
            style.visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(4);
            style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(4);
            style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(4);
            style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(4);
            
            // 3. Typografia
            style.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::proportional(theme::FONT_SIZE_LABEL),
            );
            style.text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::proportional(theme::FONT_SIZE_BTN),
            );
            style.text_styles.insert(
                egui::TextStyle::Heading,
                egui::FontId::proportional(theme::FONT_SIZE_SECTION_TITLE),
            );
            
            // 4. Interakcja
            style.spacing.interact_size.y = theme::INPUT_HEIGHT;

            // 5. Zapisanie stylu z powrotem do kontekstu egui dla danego motywu
            ctx.set_style_of(theme, style);
        }
    }

    /// Inicjalizuje podział ekranu, maluje tła slotów i przygotowuje odizolowane sub-interfejsy
    pub fn begin(&self, ui: &mut egui::Ui) -> LayoutFrame {
        // 1. Zastosowanie stylów motywu
        self.apply_theme(ui.ctx());

        let full_rect = ui.max_rect();

        let effective_drawer_h = if self.show_panel {
            self.panel_height.min(full_rect.height() * 0.5)
        } else {
            0.0
        };

        let nav_top = full_rect.max.y - self.bottom_bar_height;
        let drawer_top = nav_top - effective_drawer_h;

        // 2. Slot Górny: Kanwa
        let canvas_rect = egui::Rect::from_min_max(
            full_rect.min,
            egui::pos2(full_rect.max.x, drawer_top),
        );
        ui.painter().rect_filled(canvas_rect, 0.0, self.canvas_bg);
        let canvas_ui = ui.new_child(egui::UiBuilder::new().max_rect(canvas_rect));

        // 3. Slot Środkowy: Ukrywany panel
        let drawer_ui = if self.show_panel {
            let drawer_rect = egui::Rect::from_min_max(
                egui::pos2(full_rect.min.x, drawer_top),
                egui::pos2(full_rect.max.x, nav_top),
            );

            ui.painter().rect_filled(drawer_rect, 0.0, self.panel_bg);
            ui.painter().line_segment(
                [drawer_rect.left_top(), drawer_rect.right_top()],
                egui::Stroke::new(1.0, self.border_color),
            );

            Some(ui.new_child(egui::UiBuilder::new().max_rect(drawer_rect)))
        } else {
            None
        };

        // 4. Slot Dolny: Zakotwiczona belka nawigacji
        let nav_rect = egui::Rect::from_min_max(
            egui::pos2(full_rect.min.x, nav_top),
            full_rect.max,
        );

        ui.painter().rect_filled(nav_rect, 0.0, self.bottom_bar_bg);
        ui.painter().line_segment(
            [nav_rect.left_top(), nav_rect.right_top()],
            egui::Stroke::new(1.0, self.border_color),
        );

        let nav_ui = ui.new_child(egui::UiBuilder::new().max_rect(nav_rect));

        LayoutFrame {
            canvas_ui,
            drawer_ui,
            nav_ui,
        }
    }
}