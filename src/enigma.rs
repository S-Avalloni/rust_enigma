mod enigma {
	
	// rotors from fouth table in https://en.wikipedia.org/wiki/Enigma_rotor_details#Rotor_wiring_tables	
	pub const ROTOR_I: Rotor = Rotor([['E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O', 'W', 'Y', 'H', 'X', 'U', 'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J'], ['U', 'W', 'Y', 'G', 'A', 'D', 'F', 'P', 'V', 'Z', 'B', 'E', 'C', 'K', 'M', 'T', 'H', 'X', 'S', 'L', 'R', 'I', 'N', 'Q', 'O', 'J']]);
	pub const ROTOR_II: Rotor = Rotor([['A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W', 'T', 'M', 'C', 'Q', 'G', 'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E'], ['A', 'J', 'P', 'C', 'Z', 'W', 'R', 'L', 'F', 'B', 'D', 'K', 'O', 'T', 'Y', 'U', 'Q', 'G', 'E', 'N', 'H', 'X', 'M', 'I', 'V', 'S']]);
	pub const ROTOR_III: Rotor = Rotor([['B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R', 'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I', 'W', 'G', 'A', 'K', 'M', 'U', 'S', 'Q', 'O'], ['T', 'A', 'G', 'B', 'P', 'C', 'S', 'D', 'Q', 'E', 'U', 'F', 'V', 'N', 'Z', 'H', 'Y', 'I', 'X', 'J', 'W', 'L', 'R', 'K', 'O', 'M']]);
	pub const ROTOR_IV: Rotor = Rotor([['E', 'S', 'O', 'V', 'P', 'Z', 'J', 'A', 'Y', 'Q', 'U', 'I', 'R', 'H', 'X', 'L', 'N', 'F', 'T', 'G', 'K', 'D', 'C', 'M', 'W', 'B'], ['H', 'Z', 'W', 'V', 'A', 'R', 'T', 'N', 'L', 'G', 'U', 'P', 'X', 'Q', 'C', 'E', 'J', 'M', 'B', 'S', 'K', 'D', 'Y', 'O', 'I', 'F']]);
	pub const ROTOR_V: Rotor = Rotor([['V', 'Z', 'B', 'R', 'G', 'I', 'T', 'Y', 'U', 'P', 'S', 'D', 'N', 'H', 'L', 'X', 'A', 'W', 'M', 'J', 'Q', 'O', 'F', 'E', 'C', 'K'], ['Q', 'C', 'Y', 'L', 'X', 'W', 'E', 'N', 'F', 'T', 'Z', 'O', 'S', 'M', 'V', 'J', 'U', 'D', 'K', 'G', 'I', 'A', 'R', 'P', 'H', 'B']]);
	pub const ROTOR_VI: Rotor = Rotor([['J', 'P', 'G', 'V', 'O', 'U', 'M', 'F', 'Y', 'Q', 'B', 'E', 'N', 'H', 'Z', 'R', 'D', 'K', 'A', 'S', 'X', 'L', 'I', 'C', 'T', 'W'], ['S', 'K', 'X', 'Q', 'L', 'H', 'C', 'N', 'W', 'A', 'R', 'V', 'G', 'M', 'E', 'B', 'J', 'P', 'T', 'Y', 'F', 'D', 'Z', 'U', 'I', 'O']]);
	pub const ROTOR_VII: Rotor = Rotor([['N', 'Z', 'J', 'H', 'G', 'R', 'C', 'X', 'M', 'Y', 'S', 'W', 'B', 'O', 'U', 'F', 'A', 'I', 'V', 'L', 'P', 'E', 'K', 'Q', 'D', 'T'], ['Q', 'M', 'G', 'Y', 'V', 'P', 'E', 'D', 'R', 'C', 'W', 'T', 'I', 'A', 'N', 'U', 'X', 'F', 'K', 'Z', 'O', 'S', 'L', 'H', 'J', 'B']]);
	pub const ROTOR_VIII: Rotor = Rotor([['F', 'K', 'Q', 'H', 'T', 'L', 'X', 'O', 'C', 'B', 'J', 'S', 'P', 'D', 'Z', 'R', 'A', 'M', 'E', 'W', 'N', 'I', 'U', 'Y', 'G', 'V'], ['Q', 'J', 'I', 'N', 'S', 'A', 'Y', 'D', 'V', 'K', 'B', 'F', 'R', 'U', 'H', 'M', 'C', 'P', 'L', 'E', 'W', 'Z', 'T', 'G', 'X', 'O']]);
	
	/*
	All letters must be ascii uppercase and appear exactly once
	*/
	#[derive(Debug)]
	pub struct Rotor([[char; 26];2]);
	impl Rotor {
		pub fn new(rotor: [char; 26]) -> Result<Rotor, &'static str> {
			// 'a' as default is to circumvent the use of Option<char> and it works because we only use uppercase letters
			let mut inverse_rotor: [char; 26] = ['a'; 26]; 
			for i in 0..26 {
				let c = rotor[i];
				if !(c.is_ascii_uppercase()) {
					return Err("All characters need to be ASCII uppercase");
				}
				if inverse_rotor[c as usize - 65] != 'a' {
					return Err("Not all characters pressent");
				}

				inverse_rotor[c as usize - 65] = (i+65) as u8 as char;
			}
			Ok(Rotor([rotor, inverse_rotor]))
		}
	}
	
	/*
	All letters must be ascii uppercase
	pairing between letters (Plugboard[a: char] == b: char <=> Plugboard[b: char] == a: char) a letter can be paired with itself,
	in that case the letter remains the same.
	Because of the simetry it's enough with one vec of length 26
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
	Because of the simetry it's enough with one vec of length 26
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
			let output: String = String::with_capacity(text.len());
			for mut c in text.chars() {
				c = self.plugboard.0[c.to_ascii_uppercase() as usize - 65];
				
				c = self.rotors[0].0[0][c as usize - 65];
			}
			output
		}

	}
}