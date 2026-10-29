use std::cmp::Ordering;

// Reprezentacja ułamka zwykłego
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction(pub u64, pub u64);

impl PartialOrd for Fraction {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Fraction {
	fn cmp(&self, other: &Self) -> Ordering { (self.0 * other.1).cmp(&(other.0 * self.1)) }
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
	while b != 0 {
		let temp = b;
		b = a % b;
		a = temp;
	}
	a
}
