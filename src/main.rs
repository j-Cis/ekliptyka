#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Ukrywa czarną konsolę na Windows w trybie Release

// ==========================================
// 1. NATYWNY PUNKT WEJŚCIA (Windows, Linux, Mac)
// ==========================================
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Ecliptica - Symulator Orbity Ziemi"),
        ..Default::default()
    };

    // Zmieniona nazwa importu na "x", tak jak ustawiłeś w Cargo.toml
    eframe::run_native(
        "Ecliptica",
        options,
        Box::new(|_cc| Ok(Box::new(x::app::EclipticaApp::default()))),
    )
}

// ==========================================
// 2. PUNKT WEJŚCIA DLA WEBASSEMBLY (Przeglądarka)
// ==========================================
#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::JsCast; // Wymagane, by bezpiecznie rzutować obiekty DOM

    // Przekierowanie logów systemowych Rusta bezpośrednio do F12 (konsola przeglądarki)
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        // Znalezienie kanwy na podstawie jej sztywnego ID wpisanego w index.html
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id("the_canvas_id")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let runner = eframe::WebRunner::new();
        runner
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Ok(Box::new(x::app::EclipticaApp::default()))),
            )
            .await
            .expect("Błąd: Nie udało się wystartować eframe");
    });
}