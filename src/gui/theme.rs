use eframe::egui::Color32;

// ============================================================================
// SEKCJA 1: MOTYW INTERFEJSU UŻYTKOWNIKA (FRAME, WIDGETY, PANELE)
// ============================================================================

// Kolory tła głównych obszarów
pub const CANVAS_BG: Color32 = Color32::from_rgb(15, 23, 42);
pub const PANEL_BG: Color32 = Color32::from_rgb(30, 41, 59);
pub const BOTTOM_BAR_BG: Color32 = Color32::from_rgb(2, 15, 28);

// Guziki (stany aktywne i nieaktywne typu toggle)
pub const BTN_ACTIVE_BG: Color32 = Color32::from_rgb(12, 88, 164);
pub const BTN_INACTIVE_BG: Color32 = Color32::from_rgb(2, 15, 28);
pub const BTN_ACTION_BG: Color32 = Color32::from_rgb(15, 23, 42);

// Typografia i separatory
pub const TEXT_DEFAULT: Color32 = Color32::WHITE;
pub const SEPARATOR_COLOR: Color32 = Color32::from_rgb(7, 43, 71);

// Pola tekstowe (Inputy)
pub const INPUT_BG: Color32 = Color32::from_rgb(15, 23, 42);

// Wymiary wysokości (px)
pub const PANEL_HEIGHT: f32 = 240.0;
pub const BOTTOM_BAR_HEIGHT: f32 = 38.0;
pub const BTN_HEIGHT: f32 = 34.0;
pub const INPUT_HEIGHT: f32 = 22.0;

// Rozmiary czcionek (pt / px)
pub const FONT_SIZE_BTN: f32 = 16.0;
pub const FONT_SIZE_INPUT: f32 = 15.0;
pub const FONT_SIZE_LABEL: f32 = 15.0;
pub const FONT_SIZE_SECTION_TITLE: f32 = 20.0;

// Suwaki (Sliders)
pub const SLIDER_RAIL_COLOR: Color32 = Color32::from_rgb(15, 23, 42);
pub const SLIDER_INPUT_BG: Color32 = Color32::from_rgb(15, 23, 42);
pub const SLIDER_HANDLE_COLOR: Color32 = Color32::from_rgb(12, 88, 164);
pub const SLIDER_HANDLE_BORDER: Color32 = Color32::from_rgb(180, 210, 240);

// ============================================================================
// SEKCJA 2: ASTRONOMIA, PŁÓTNO (KANWA) I WYNIKI SCENARIUSZY
// ============================================================================

// Orbita bazowa
pub const ORBIT_COLOR: Color32 = Color32::from_rgb(30, 41, 59);
pub const ORBIT_STROKE_WIDTH: f32 = 1.5;

// Słońce
pub const SUN_COLOR: Color32 = Color32::from_rgb(255, 160, 50);
pub const SUN_SIZE: f32 = 20.0;

// Ziemia oraz punkty/etykiety cyklu w Scenariuszu 3
pub const EARTH_COLOR: Color32 = Color32::from_rgb(0, 65, 62);
pub const EARTH_SIZE: f32 = 5.0;
pub const SCENARIO_3_LABEL_FONT_SIZE: f32 = 12.0;
pub const SCENARIO_3_LABEL_OFFSET: f32 = 28.0;

// Punkty charakterystyczne orbity (Geometria pór roku)
pub const SOLSTICE_COLOR: Color32 = Color32::from_rgb(255, 120, 0); // Kwadraty, etykiety i linia łącząca
pub const EQUINOX_COLOR: Color32 = Color32::GREEN; // Romby, etykiety i linia łącząca
pub const APHELIUM_COLOR: Color32 = Color32::from_rgb(100, 200, 255); // Odcinek i etykieta
pub const PERYHELIUM_COLOR: Color32 = Color32::RED; // Odcinek i etykieta

// Linie i punkty trybu Debug
pub const DEBUG_EXTRA_COLOR: Color32 = Color32::BLACK;
pub const SEASONS_LINE_STROKE_WIDTH: f32 = 1.5;
pub const DEBUG_LINE_STROKE_WIDTH: f32 = 1.2;

pub const SEASONS_POINT_SIZE: f32 = 4.0; // Rozmiar rombów i kwadratów
pub const DEBUG_POINT_SIZE: f32 = 3.0; // Rozmiar małych czarnych punktów

// Etykiety punktów orbitalnych (Aphelium, Peryhelium, Equinoxes, Solstices)
pub const ORBIT_LABELS_FONT_SIZE: f32 = 12.0;
pub const ORBIT_LABELS_OFFSET: f32 = 80.0; // Dystans zewnętrzny etykiet od orbity

// Kolory wyników scenariuszy obliczeniowych
pub const SCENARIO_1_RESULT_COLOR: Color32 = Color32::GREEN;
pub const SCENARIO_2_RESULT_COLOR: Color32 = Color32::LIGHT_BLUE;
pub const SCENARIO_3_RESULT_COLOR: Color32 = Color32::GOLD;
