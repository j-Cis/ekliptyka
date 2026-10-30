use std::{env, process};

use x::core::astro::astro_plural::Cycle;

fn main() {
	// Pobieramy argumenty z terminala
	let args: Vec<String> = env::args().collect();

	// Sprawdzamy, czy podano dokładnie 2 argumenty (args[0] to nazwa programu)
	if args.len() != 3 {
		eprintln!("Użycie: cargo run --example sample01cli -- <ilość_kawałków> <rok>");
		eprintln!("Przykład: cargo run --example sample01cli -- 420 2026");
		process::exit(1);
	}

	// Parsujemy pierwszy argument jako u64
	let pieces: u64 = match args[1].parse() {
		Ok(num) => num,
		Err(_) => {
			eprintln!("Błąd: Pierwszy argument (kawałki) musi być liczbą całkowitą dodatnią.");
			process::exit(1);
		}
	};

	// Parsujemy drugi argument jako i16
	let year: i16 = match args[2].parse() {
		Ok(num) => num,
		Err(_) => {
			eprintln!("Błąd: Drugi argument (rok) musi być liczbą całkowitą.");
			process::exit(1);
		}
	};

	// Generujemy cykl i rysujemy tabelę
	let my_cycle = Cycle::new(pieces, year);
	my_cycle.plot_table();
}

// UŻYCIE
// `cargo run --example sample01cli -- 420 2026`
