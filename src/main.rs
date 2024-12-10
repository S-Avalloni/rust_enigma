#![allow(unused)]

fn main() {
    println!("Hello, world!");	
}

const ROTOR_I: [char; 26] =['E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O', 'W', 'Y', 'H', 'X', 'U', 'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J'];
const ROTOR_II: [char; 26] =['A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W', 'T', 'M', 'C', 'Q', 'G', 'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E'];
const ROTOR_III: [char; 26] =['B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R', 'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I', 'W', 'G', 'A', 'K', 'M', 'U', 'S', 'Q', 'O'];
const ROTOR_IV: [char; 26] =['E', 'S', 'O', 'V', 'P', 'Z', 'J', 'A', 'Y', 'Q', 'U', 'I', 'R', 'H', 'X', 'L', 'N', 'F', 'T', 'G', 'K', 'D', 'C', 'M', 'W', 'B'];
const ROTOR_V: [char; 26] =['V', 'Z', 'B', 'R', 'G', 'I', 'T', 'Y', 'U', 'P', 'S', 'D', 'N', 'H', 'L', 'X', 'A', 'W', 'M', 'J', 'Q', 'O', 'F', 'E', 'C', 'K'];
const ROTOR_VI: [char; 26] =['J', 'P', 'G', 'V', 'O', 'U', 'M', 'F', 'Y', 'Q', 'B', 'E', 'N', 'H', 'Z', 'R', 'D', 'K', 'A', 'S', 'X', 'L', 'I', 'C', 'T', 'W'];
const ROTOR_VII: [char; 26] =['N', 'Z', 'J', 'H', 'G', 'R', 'C', 'X', 'M', 'Y', 'S', 'W', 'B', 'O', 'U', 'F', 'A', 'I', 'V', 'L', 'P', 'E', 'K', 'Q', 'D', 'T'];
const ROTOR_VIII: [char; 26] =['F', 'K', 'Q', 'H', 'T', 'L', 'X', 'O', 'C', 'B', 'J', 'S', 'P', 'D', 'Z', 'R', 'A', 'M', 'E', 'W', 'N', 'I', 'U', 'Y', 'G', 'V'];


/*
All letters must be ascii uppercase
Each leter must be conected with another (Rotor[a: char] != a) and each one must appear exactly once
*/
struct Rotor([char; 26]);

/*
All letters must be ascii uppercase
pairing between letters (Plugboard[a: char] == b: char <=> Plugboard[b: char] == a: char) and not all letters need to be pressent,
in that case the letter remains the same
*/
struct Plugboard([char; 26]); 



/*
All letters must be ascii uppercase
pairing between letters (Plugboard[a: char] == b: char <=> Plugboard[b: char] == a: char) and all letters need to be paired,
No letter can be paired with itself
*/
struct Reflector([char; 26]);

struct Enigma {
	rotors: [Rotor; 3],
	positions: [u8; 3],
	plugboard: Plugboard,
	reflector: Reflector,
}