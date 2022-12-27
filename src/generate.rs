pub mod generator {
	extern crate base64;
	extern crate cipher_crypt;
	use cipher_crypt::{Cipher, Rot13, Caesar, Vigenere, Porta, Scytale};

	use std::io::Bytes;
	use std::path::PathBuf;
	use qrcode_png::{Color, QrCode, QrCodeEcc};

	pub fn encode_data(algorithm: &String, message: &String, output: &String) {
		match algorithm.as_str() {
			"txt" => generate_qrcode(&message, &output),
			"hex" => generate_qrcode(&hex::encode(&message), &output),
			"rot13" => generate_qrcode(&Rot13::encrypt(&message), &output),
			"morse" => generate_qrcode(&crypto_morse::encode(&message), &output),

			"base64" => {
				let msg_bytes = String::from(message).into_bytes();
				generate_qrcode(&base64::encode(msg_bytes), &output);
			},

			_ => {
				panic!("Unknown encoding algorithm!");
			}
		};
	}

	pub fn encrypt_data(algorithm: &String, message: &String, key: &String, output: &String) {
		let encrypted_data = match algorithm.as_str() {
			"vigenere" => {
				let cipher = Vigenere::new((&key).to_string());
				cipher.encrypt(&message).unwrap()
			},

			"porta" => {
				let cipher = Porta::new((&key).to_string());
				cipher.encrypt(&message).unwrap()
			},

			"caesar" => {
				let num = key.parse::<i64>().unwrap();
				let cipher = Caesar::new(num.try_into().unwrap());
				cipher.encrypt(&message).unwrap()
			},

			"scytale" => {
				let num = key.parse::<i64>().unwrap();
				let cipher = Scytale::new(num.try_into().unwrap());
				cipher.encrypt(&message).unwrap()
			},

			_ => {
				panic!("Unknown encrypting algorithm!");
			}
		};

		generate_qrcode(&encrypted_data, &output);
	}

	fn generate_qrcode(encoded_data: &str, output_file: &str) {
		let mut qrcode = QrCode::new(encoded_data, QrCodeEcc::Medium)
			.unwrap();

			qrcode.margin(12)
			.zoom(12);

		let color = Color::Grayscale(0, 255);
		let image_data = qrcode.generate(color).unwrap();
		std::fs::write(output_file, image_data).unwrap();
	}
}

#[cfg(test)]
mod test {

	use super::*;

	fn test_start() {

	}

}
