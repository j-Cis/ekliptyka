// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

pub mod core {
	pub mod astro;
	pub mod cycle;
}

pub mod gui {
  pub mod theme;
  pub mod app_frame;
	pub mod canvas;
	pub mod controls;
}
pub mod app;
