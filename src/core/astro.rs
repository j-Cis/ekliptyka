// or astro_single rename to astro_unit
pub mod astro_single {

	use std::cmp::Ordering;

	use super::super::cycle::Fraction;

	// Reprezentacja pojedynczego punktu
	#[derive(Debug, Clone)]
	pub struct CyclePoint {
		pub etykieta: Fraction,
		pub pozycja: f64,
		pub kat: f64,
		pub data_miesieczna: String,
		pub data_weekendowa: String,
		pub godzina_utc: String,
		pub odleglosc_km: f64,
		pub timestamp_sec: i64,
	}

	impl PartialEq for CyclePoint {
		fn eq(&self, other: &Self) -> bool {
			self.timestamp_sec == other.timestamp_sec
		}
	}
	impl Eq for CyclePoint {}

	impl PartialOrd for CyclePoint {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			Some(self.cmp(other))
		}
	}
	impl Ord for CyclePoint {
		fn cmp(&self, other: &Self) -> Ordering {
			self.timestamp_sec.cmp(&other.timestamp_sec)
		}
	}
}

// or astro_plural rename to astro_sets
pub mod astro_plural {
	use std::collections::BTreeSet;

	use jiff::{Span, Timestamp, tz::TimeZone};

	use super::{
		super::cycle::{Fraction, gcd},
		astro_single::CyclePoint,
		specs::find_time_and_distance_for_longitude,
	};
	pub struct Cycle {
		pub points: BTreeSet<CyclePoint>,
	}

	impl Cycle {
		pub fn new(pieces: u64, target_year: i16) -> Self {
			let mut points = BTreeSet::new();

			for i in 0..pieces {
				let divisor = gcd(i, pieces);
				let pozycja = i as f64 / pieces as f64;
				let target_kat = pozycja * 360.0;

				// Szukamy momentu startując od wiosny zadanego roku
				let (mut jd, mut distance_au) = find_time_and_distance_for_longitude(target_year, target_kat);

				let mut unix_seconds = (jd - 2440587.5) * 86400.0;
				let mut timestamp = Timestamp::from_second(unix_seconds.round() as i64).unwrap();
				let mut point_time = timestamp.to_zoned(TimeZone::UTC);

				// KOREKTA KALENDARZOWA:
				// Zmuszamy datę do powrotu w granice wybranego roku (np. wyższe
				// kąty od marca przeskoczą na styczeń-luty)
				if point_time.year() > target_year {
					let (jd_new, dist_new) = find_time_and_distance_for_longitude(target_year - 1, target_kat);
					jd = jd_new;
					distance_au = dist_new;
					unix_seconds = (jd - 2440587.5) * 86400.0;
					timestamp = Timestamp::from_second(unix_seconds.round() as i64).unwrap();
					point_time = timestamp.to_zoned(TimeZone::UTC);
				} else if point_time.year() < target_year {
					let (jd_new, dist_new) = find_time_and_distance_for_longitude(target_year + 1, target_kat);
					jd = jd_new;
					distance_au = dist_new;
					unix_seconds = (jd - 2440587.5) * 86400.0;
					timestamp = Timestamp::from_second(unix_seconds.round() as i64).unwrap();
					point_time = timestamp.to_zoned(TimeZone::UTC);
				}

				// Zaokrąglenie o 30 sekund w górę, by format %H:%M był poprawny
				// matematycznie
				point_time = point_time.checked_add(Span::new().seconds(30)).unwrap();
				let final_timestamp_sec = point_time.timestamp().as_second();

				let odleglosc_km = distance_au * 149.5978707;

				points.insert(CyclePoint {
					etykieta: Fraction(i / divisor, pieces / divisor),
					pozycja,
					kat: target_kat,
					data_miesieczna: point_time.strftime("%Y-%m-%d").to_string(),
					data_weekendowa: point_time.strftime("%G-W%V-%u").to_string(),
					godzina_utc: point_time.strftime("%H:%M").to_string(),
					odleglosc_km,
					timestamp_sec: final_timestamp_sec,
				});
			}

			Self { points }
		}

		pub fn plot_table(&self) {
			println!(
				"{:<4} | {:<8} | {:<9} | {:<6} | {:<10} | {:<10} | {:<8} | {:<10}",
				"Lp.", "Obrót", "Pozycja", "Kąt", "YYYY-MM-DD", "GGGG-WWW-D", "Czas UTC", "Odl. Słońca (mln km)"
			);
			println!("{:-<105}", "");

			for (index, point) in self.points.iter().enumerate() {
				println!(
					"{:<4} | {:>3}/{:<4} | {:.7} | {:>5.1}° | {} | {} | {:>8} | {:.3} mln km",
					index,
					point.etykieta.0,
					point.etykieta.1,
					point.pozycja,
					point.kat,
					point.data_miesieczna,
					point.data_weekendowa,
					point.godzina_utc,
					point.odleglosc_km
				);
			}
		}
	}
}

pub mod specs {
	#![allow(clippy::approx_constant)]

	// =====================================================================
	// SILNIK ASTRONOMICZNY
	// Pozorna długość ekliptyczna Słońca: VSOP87D (Ziemia, wersja skrócona wg
	// Meeusa, "Astronomical Algorithms", dod. III) + nutacja IAU 1980 +
	// aberracja
	// + poprawka FK5. Zastępuje formułę niskiej dokładności z rozdz. 25
	// (jej deklarowana dokładność to ok. 0,01° ≈ 14 min czasu).
	// =====================================================================

	/// ΔT = TT − UT w dobach. 0,0008 d ≈ 69 s – wartość z oryginalnego kodu
	/// (rok 2026). Dla innych lat trzeba ją zmienić.
	const DELTA_T_DAYS: f64 = 0.0008;

	/// Człon szeregu (A, B, C) → A·cos(B + C·τ), τ w tysiącleciach juliańskich
	/// od J2000.0.
	type Term = (f64, f64, f64);

	/// Długość heliocentryczna Ziemi L, człony L0 (jednostka 1e-8 rad).
	const L0: &[Term] = &[
		(175347046.0, 0.0, 0.0),
		(3341656.0, 4.6692568, 6283.07585),
		(34894.0, 4.6261, 12566.1517),
		(3497.0, 2.7441, 5753.3849),
		(3418.0, 2.8289, 3.5231),
		(3136.0, 3.6277, 77713.7715),
		(2676.0, 4.4181, 7860.4194),
		(2343.0, 6.1352, 3930.2097),
		(1324.0, 0.7425, 11506.7698),
		(1273.0, 2.0371, 529.691),
		(1199.0, 1.1096, 1577.3435),
		(990.0, 5.233, 5884.927),
		(902.0, 2.045, 26.298),
		(857.0, 3.508, 398.149),
		(780.0, 1.179, 5223.694),
		(753.0, 2.533, 5507.553),
		(505.0, 4.583, 18849.228),
		(492.0, 4.205, 775.523),
		(357.0, 2.92, 0.067),
		(317.0, 5.849, 11790.629),
		(284.0, 1.899, 796.298),
		(271.0, 0.315, 10977.079),
		(243.0, 0.345, 5486.778),
		(206.0, 4.806, 2544.314),
		(205.0, 1.869, 5573.143),
		(202.0, 2.458, 6069.777),
		(156.0, 0.833, 213.299),
		(132.0, 3.411, 2942.463),
		(126.0, 1.083, 20.775),
		(115.0, 0.645, 0.98),
		(103.0, 0.636, 4694.003),
		(102.0, 0.976, 15720.839),
		(102.0, 4.267, 7.114),
		(99.0, 6.21, 2146.17),
		(98.0, 0.68, 155.42),
		(86.0, 5.98, 161000.69),
		(85.0, 1.3, 6275.96),
		(85.0, 3.67, 71430.7),
		(80.0, 1.81, 17260.15),
		(79.0, 3.04, 12036.46),
		(75.0, 1.76, 5088.63),
		(74.0, 3.5, 3154.69),
		(74.0, 4.68, 801.82),
		(70.0, 0.83, 9437.76),
		(62.0, 3.98, 8827.39),
		(61.0, 1.82, 7084.9),
		(57.0, 2.78, 6286.6),
		(56.0, 4.39, 14143.5),
		(56.0, 3.47, 6279.55),
		(52.0, 0.19, 12139.55),
		(52.0, 1.33, 1748.02),
		(51.0, 0.28, 5856.48),
		(49.0, 0.49, 1194.45),
		(41.0, 5.37, 8429.24),
		(41.0, 2.4, 19651.05),
		(39.0, 6.17, 10447.39),
		(37.0, 6.04, 10213.29),
		(37.0, 2.57, 1059.38),
		(36.0, 1.71, 2352.87),
		(36.0, 1.78, 6812.77),
		(33.0, 0.59, 17789.85),
		(30.0, 0.44, 83996.85),
		(30.0, 2.74, 1349.87),
		(25.0, 3.16, 4690.48),
	];

	/// Człony L1 (mnożone przez τ).
	const L1: &[Term] = &[
		(628331966747.0, 0.0, 0.0),
		(206059.0, 2.678235, 6283.07585),
		(4303.0, 2.6351, 12566.1517),
		(425.0, 1.59, 3.523),
		(119.0, 5.796, 26.298),
		(109.0, 2.966, 1577.344),
		(93.0, 2.59, 18849.23),
		(72.0, 1.14, 529.69),
		(68.0, 1.87, 398.15),
		(67.0, 4.41, 5507.55),
		(59.0, 2.89, 5223.69),
		(56.0, 2.17, 155.42),
		(45.0, 0.4, 796.3),
		(36.0, 0.47, 775.52),
		(29.0, 2.65, 7.11),
		(21.0, 5.34, 0.98),
		(19.0, 1.85, 5486.78),
		(19.0, 4.97, 213.3),
		(17.0, 2.99, 6275.96),
		(16.0, 0.03, 2544.31),
		(16.0, 1.43, 2146.17),
		(15.0, 1.21, 10977.08),
		(12.0, 2.83, 1748.02),
		(12.0, 3.26, 5088.63),
		(12.0, 5.27, 1194.45),
		(12.0, 2.08, 4694.0),
		(11.0, 0.77, 553.57),
		(10.0, 1.3, 6286.6),
		(10.0, 4.24, 1349.87),
		(9.0, 2.7, 242.73),
		(9.0, 5.64, 951.72),
		(8.0, 5.3, 2352.87),
		(6.0, 2.65, 9437.76),
		(6.0, 4.67, 4690.48),
	];

	/// Człony L2 (τ²).
	const L2: &[Term] = &[
		(52919.0, 0.0, 0.0),
		(8720.0, 1.0721, 6283.0758),
		(309.0, 0.867, 12566.152),
		(27.0, 0.05, 3.52),
		(16.0, 5.19, 26.3),
		(16.0, 3.68, 155.42),
		(10.0, 0.76, 18849.23),
		(9.0, 2.06, 77713.77),
		(7.0, 0.83, 775.52),
		(5.0, 4.66, 1577.34),
		(4.0, 1.03, 7.11),
		(4.0, 3.44, 5573.14),
		(3.0, 5.14, 796.3),
		(3.0, 6.05, 5507.55),
		(3.0, 1.19, 242.73),
		(3.0, 6.12, 529.69),
		(3.0, 0.31, 398.15),
		(3.0, 2.28, 553.57),
		(2.0, 4.38, 5223.69),
		(2.0, 3.75, 0.98),
	];

	/// Człony L3 (τ³).
	const L3: &[Term] = &[
		(289.0, 5.844, 6283.076),
		(35.0, 0.0, 0.0),
		(17.0, 5.49, 12566.15),
		(3.0, 5.2, 155.42),
		(1.0, 4.72, 3.52),
		(1.0, 5.3, 18849.23),
		(1.0, 5.97, 242.73),
	];

	/// Człony L4 (τ⁴).
	const L4: &[Term] = &[(114.0, 3.142, 0.0), (8.0, 4.13, 6283.08), (1.0, 3.84, 12566.15)];

	/// Człony L5 (τ⁵).
	const L5: &[Term] = &[(1.0, 3.14, 0.0)];

	/// Promień wodzący Ziemi R, człony R0 (jednostka 1e-8 AU).
	const R0: &[Term] = &[
		(100013989.0, 0.0, 0.0),
		(1670700.0, 3.0984635, 6283.07585),
		(13956.0, 3.05525, 12566.1517),
		(3084.0, 5.1985, 77713.7715),
		(1628.0, 1.1739, 5753.3849),
		(1576.0, 2.8469, 7860.4194),
		(925.0, 5.453, 11506.77),
		(542.0, 4.564, 3930.21),
		(472.0, 3.661, 5884.927),
		(346.0, 0.964, 5507.553),
		(329.0, 5.9, 5223.694),
		(307.0, 0.299, 5573.143),
		(243.0, 4.273, 11790.629),
		(212.0, 5.847, 1577.344),
		(186.0, 5.022, 10977.079),
		(175.0, 3.012, 18849.228),
		(110.0, 5.055, 5486.778),
		(98.0, 0.89, 6069.78),
		(86.0, 5.69, 15720.84),
		(86.0, 1.27, 161000.69),
		(65.0, 0.27, 17260.15),
		(63.0, 0.92, 529.69),
		(57.0, 2.01, 83996.85),
		(56.0, 5.24, 71430.7),
		(49.0, 3.25, 2544.31),
		(47.0, 2.58, 775.52),
		(45.0, 5.54, 9437.76),
		(43.0, 6.01, 6275.96),
		(39.0, 5.36, 4694.0),
		(38.0, 2.39, 8827.39),
		(37.0, 0.83, 19651.05),
		(37.0, 4.9, 12139.55),
		(36.0, 1.67, 12036.46),
		(35.0, 1.84, 2942.46),
		(33.0, 0.24, 7084.9),
		(32.0, 0.18, 5088.63),
		(32.0, 1.78, 398.15),
		(28.0, 1.21, 6286.6),
		(28.0, 1.9, 6279.55),
		(26.0, 4.59, 10447.39),
	];

	/// Człony R1 (τ).
	const R1: &[Term] = &[
		(103019.0, 1.10749, 6283.07585),
		(1721.0, 1.0644, 12566.1517),
		(702.0, 3.142, 0.0),
		(32.0, 1.02, 18849.23),
		(31.0, 2.84, 5507.55),
		(25.0, 1.32, 5223.69),
		(18.0, 1.42, 1577.34),
		(10.0, 5.91, 10977.08),
		(9.0, 1.42, 6275.96),
		(9.0, 0.27, 5486.78),
	];

	/// Człony R2 (τ²).
	const R2: &[Term] = &[
		(4359.0, 5.7846, 6283.0758),
		(124.0, 5.579, 12566.152),
		(12.0, 3.14, 0.0),
		(9.0, 3.63, 77713.77),
		(6.0, 1.87, 5573.14),
		(3.0, 5.47, 18849.23),
	];

	/// Człony R3 (τ³).
	const R3: &[Term] = &[(145.0, 4.273, 6283.076), (7.0, 3.92, 12566.15)];

	/// Człony R4 (τ⁴).
	const R4: &[Term] = &[(4.0, 2.56, 6283.08)];

	/// Nutacja w długości, IAU 1980 (Meeus, rozdz. 22), 31 największych
	/// wyrazów. (D, M, M', F, Ω, s0, s1) – amplituda = s0 + s1·T w
	/// jednostkach 0,0001″; T w stuleciach juliańskich.
	const NUTATION: &[(i8, i8, i8, i8, i8, f64, f64)] = &[
		(0, 0, 0, 0, 1, -171996.0, -174.2),
		(-2, 0, 0, 2, 2, -13187.0, -1.6),
		(0, 0, 0, 2, 2, -2274.0, -0.2),
		(0, 0, 0, 0, 2, 2062.0, 0.2),
		(0, 1, 0, 0, 0, 1426.0, -3.4),
		(0, 0, 1, 0, 0, 712.0, 0.1),
		(-2, 1, 0, 2, 2, -517.0, 1.2),
		(0, 0, 0, 2, 1, -386.0, -0.4),
		(0, 0, 1, 2, 2, -301.0, 0.0),
		(-2, -1, 0, 2, 2, 217.0, -0.5),
		(-2, 0, 1, 0, 0, -158.0, 0.0),
		(-2, 0, 0, 2, 1, 129.0, 0.1),
		(0, 0, -1, 2, 2, 123.0, 0.0),
		(2, 0, 0, 0, 0, 63.0, 0.0),
		(0, 0, 1, 0, 1, 63.0, 0.1),
		(2, 0, -1, 2, 2, -59.0, 0.0),
		(0, 0, -1, 0, 1, -58.0, -0.1),
		(0, 0, 1, 2, 1, -51.0, 0.0),
		(-2, 0, 2, 0, 0, 48.0, 0.0),
		(0, 0, -2, 2, 1, 46.0, 0.0),
		(2, 0, 0, 2, 2, -38.0, 0.0),
		(0, 0, 2, 2, 2, -31.0, 0.0),
		(0, 0, 2, 0, 0, 29.0, 0.0),
		(-2, 0, 1, 2, 2, 29.0, 0.0),
		(0, 0, 0, 2, 0, 26.0, 0.0),
		(-2, 0, 0, 2, 0, -22.0, 0.0),
		(0, 0, -1, 2, 1, 21.0, 0.0),
		(0, 2, 0, 0, 0, 17.0, -0.1),
		(2, 0, -1, 0, 1, 16.0, 0.0),
		(-2, 2, 0, 2, 2, -16.0, 0.1),
		(0, 1, 0, 0, 1, -15.0, 0.0),
	];

	fn series(terms: &[Term], tau: f64) -> f64 {
		terms.iter().map(|&(a, b, c)| a * (b + c * tau).cos()).sum::<f64>()
	}

	/// Nutacja w długości Δψ w stopniach; t – stulecia juliańskie od J2000.0.
	fn nutation_in_longitude_deg(t: f64) -> f64 {
		let t2 = t * t;
		let t3 = t2 * t;
		let d = 297.85036 + 445267.111480 * t - 0.0019142 * t2 + t3 / 189474.0;
		let m = 357.52772 + 35999.050340 * t - 0.0001603 * t2 - t3 / 300000.0;
		let mp = 134.96298 + 477198.867398 * t + 0.0086972 * t2 + t3 / 56250.0;
		let f = 93.27191 + 483202.017538 * t - 0.0036825 * t2 + t3 / 327270.0;
		let om = 125.04452 - 1934.136261 * t + 0.0020708 * t2 + t3 / 450000.0;

		let mut sum = 0.0; // w jednostkach 0,0001″
		for &(c_d, c_m, c_mp, c_f, c_om, s0, s1) in NUTATION {
			let arg =
				((c_d as f64) * d + (c_m as f64) * m + (c_mp as f64) * mp + (c_f as f64) * f + (c_om as f64) * om)
					.to_radians();
			sum += (s0 + s1 * t) * arg.sin();
		}
		sum * 1e-4 / 3600.0
	}

	pub fn find_time_and_distance_for_longitude(year: i16, target_lon: f64) -> (f64, f64) {
		let days_from_equinox = target_lon * 365.2425 / 360.0;
		let base_jd = 2451623.5 + (year as f64 - 2000.0) * 365.2425;
		let jd_guess = base_jd + days_from_equinox;

		// Rzeczywisty moment odbiega od ruchu jednostajnego nawet o ok. 4 doby,
		// więc okno ±10 dób zostawia zapas (przy ±5 zapas był ok. 1 doby).
		const HALF_WINDOW_DAYS: f64 = 10.0;
		let mut jd_min = jd_guess - HALF_WINDOW_DAYS;
		let mut jd_max = jd_guess + HALF_WINDOW_DAYS;

		let mut final_jd = jd_guess;

		for _ in 0..60 {
			let jd_mid = (jd_min + jd_max) / 2.0;
			let (lon, _) = sun_position(jd_mid);

			let mut diff = lon - target_lon;
			if diff > 180.0 {
				diff -= 360.0;
			}
			if diff < -180.0 {
				diff += 360.0;
			}

			if diff > 0.0 {
				jd_max = jd_mid;
			} else {
				jd_min = jd_mid;
			}
			final_jd = jd_mid;
		}

		// Jeśli wynik wylądował przy brzegu okna, to rozwiązania w nim nie
		// było.
		debug_assert!(
			(final_jd - jd_guess).abs() < HALF_WINDOW_DAYS - 0.5,
			"brak rozwiązania w oknie wyszukiwania dla kąta {}",
			target_lon
		);

		let (_, distance_au) = sun_position(final_jd);
		(final_jd, distance_au)
	}

	/// Zwraca (pozorna długość ekliptyczna Słońca w stopniach [0, 360),
	/// odległość Ziemia–Słońce w AU). `jd` – data juliańska w czasie
	/// uniwersalnym (UT).
	pub fn sun_position(jd: f64) -> (f64, f64) {
		let jde = jd + DELTA_T_DAYS;
		let tau = (jde - 2451545.0) / 365250.0; // tysiąclecia juliańskie
		let t = tau * 10.0; // stulecia juliańskie

		// Heliocentryczna długość Ziemi L [rad] i promień wodzący R [AU]
		let l = (series(L0, tau)
			+ series(L1, tau) * tau
			+ series(L2, tau) * tau.powi(2)
			+ series(L3, tau) * tau.powi(3)
			+ series(L4, tau) * tau.powi(4)
			+ series(L5, tau) * tau.powi(5))
			* 1e-8;
		let r = (series(R0, tau)
			+ series(R1, tau) * tau
			+ series(R2, tau) * tau.powi(2)
			+ series(R3, tau) * tau.powi(3)
			+ series(R4, tau) * tau.powi(4))
			* 1e-8;

		// Geocentryczna długość Słońca = L + 180°, z poprawką FK5 (−0,09033″)
		let geometric = l.to_degrees() + 180.0 - 0.09033 / 3600.0;
		let nutation = nutation_in_longitude_deg(t);
		let aberration = -20.4898 / r / 3600.0; // −20,4898″ / R

		let apparent_lon = (geometric + nutation + aberration).rem_euclid(360.0);
		(apparent_lon, r)
	}
}
