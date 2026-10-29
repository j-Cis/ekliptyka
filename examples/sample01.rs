// ./ekliptyka/examples/sample01.rs
use ekliptyka::core::astro::astro_plural::Cycle;

fn main() {
	let my_cycle = Cycle::new(420, 2026);
	my_cycle.plot_table();
}
