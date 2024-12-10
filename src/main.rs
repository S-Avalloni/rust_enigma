

fn main() {
	let b = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let a = enigma::Enigma::new(
		[enigma::ROTOR_I, enigma::ROTOR_II, enigma::ROTOR_III],
		[1, 2, 3],
		enigma::Plugboard::new(b).unwrap(),
		enigma::Reflector::new(['B'; 26]).unwrap(),
	);
}


mod enigma {

	// rotors from fouth table in https://en.wikipedia.org/wiki/Enigma_rotor_details#Rotor_wiring_tables
	pub const ROTOR_I: Rotor = Rotor(['E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O', 'W', 'Y', 'H', 'X', 'U', 'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J']);
	pub const ROTOR_II: Rotor = Rotor(['A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W', 'T', 'M', 'C', 'Q', 'G', 'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E']);
	pub const ROTOR_III: Rotor = Rotor(['B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R', 'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I', 'W', 'G', 'A', 'K', 'M', 'U', 'S', 'Q', 'O']);
	pub const ROTOR_IV: Rotor = Rotor(['E', 'S', 'O', 'V', 'P', 'Z', 'J', 'A', 'Y', 'Q', 'U', 'I', 'R', 'H', 'X', 'L', 'N', 'F', 'T', 'G', 'K', 'D', 'C', 'M', 'W', 'B']);
	pub const ROTOR_V: Rotor = Rotor(['V', 'Z', 'B', 'R', 'G', 'I', 'T', 'Y', 'U', 'P', 'S', 'D', 'N', 'H', 'L', 'X', 'A', 'W', 'M', 'J', 'Q', 'O', 'F', 'E', 'C', 'K']);
	pub const ROTOR_VI: Rotor = Rotor(['J', 'P', 'G', 'V', 'O', 'U', 'M', 'F', 'Y', 'Q', 'B', 'E', 'N', 'H', 'Z', 'R', 'D', 'K', 'A', 'S', 'X', 'L', 'I', 'C', 'T', 'W']);
	pub const ROTOR_VII: Rotor = Rotor(['N', 'Z', 'J', 'H', 'G', 'R', 'C', 'X', 'M', 'Y', 'S', 'W', 'B', 'O', 'U', 'F', 'A', 'I', 'V', 'L', 'P', 'E', 'K', 'Q', 'D', 'T']);
	pub const ROTOR_VIII: Rotor = Rotor(['F', 'K', 'Q', 'H', 'T', 'L', 'X', 'O', 'C', 'B', 'J', 'S', 'P', 'D', 'Z', 'R', 'A', 'M', 'E', 'W', 'N', 'I', 'U', 'Y', 'G', 'V']);
	
	
	/*
	All letters must be ascii uppercase
	Each leter must be conected with another (Rotor[a: char] != a) and each one must appear exactly once
	*/
	pub struct Rotor([char; 26]);
	impl Rotor {
		pub fn new(rotor: [char; 26]) -> Result<Rotor, &'static str> {
			let mut seen_chars = [false; 26];
			for i in 0..26 {
				let c = rotor[i];
				if !(c.is_ascii_uppercase()) {
					return Err("All characters need to be ASCII uppercase");
				}
				if seen_chars[c as usize - 65] {
					return Err("Not all characters pressent");
				} else if i == (c as usize - 65) {
					return Err("Letter cannot be conected with itself")
				}

				seen_chars[c as usize - 65] = true;
			}
			Ok(Rotor(rotor))
		}
	}
	
	/*
	All letters must be ascii uppercase
	pairing between letters (Plugboard[a: char] == b: char <=> Plugboard[b: char] == a: char) a letter can be paired with itself,
	in that case the letter remains the same
	*/
	pub struct Plugboard([char; 26]);
	impl Plugboard {
		pub fn new(plugboard: [char; 26]) -> Result<Plugboard, &'static str> {
			for c in plugboard {
				if !(c.is_ascii_uppercase()) {
					return Err("All characters need to be ASCII uppercase");
				}
			}
			for i in 0..26 {
				if plugboard[plugboard[i] as usize - 65] as usize - 65  != i {
					return Err("Letters not paired correctly")
				}
			}
			Ok(Plugboard(plugboard))
		}
	}
	
	
	
	/*
	All letters must be ascii uppercase
	pairing between letters (Plugboard[a: char] == b: char <=> Plugboard[b: char] == a: char) and all letters need to be paired,
	No letter can be paired with itself
	*/
	pub struct Reflector([char; 26]);
	impl Reflector {
		pub fn new(reflector: [char; 26]) -> Result<Reflector, &'static str> {
			for c in reflector {
				if !(c.is_ascii_uppercase()) {
					return Err("All characters need to be ASCII uppercase");
				}
			}
			for i in 0..26 {
				if reflector[i] as usize - 65 == i {
					return Err("Letter canot be paired with itself")
				} 
				if reflector[reflector[i] as usize - 65] as usize - 65  != i {
					return Err("Letters not paired correctly")
				}
			}
			Ok(Reflector(reflector))
		}
	}
	
	pub struct Enigma {
		rotors: [Rotor; 3],
		positions: [u8; 3],
		plugboard: Plugboard,
		reflector: Reflector,
	}
	
	
	
	impl Enigma {
		pub fn new(rotors: [Rotor; 3], positions: [u8; 3], plugboard: Plugboard, reflector: Reflector) -> Enigma {
			Enigma {
				rotors,
				positions: [positions[0]%26, positions[1]%26, positions[2]%26],
				plugboard,
				reflector,
			}
		}

		pub fn enc_dec(&mut self, text: String) -> String {
			todo!();
		}

	}
}