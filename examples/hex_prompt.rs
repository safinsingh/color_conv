use std::env::{self};

use anyhow::{ensure, Context, Result};
use color_conv::{Color, Hsl};
use rustyline::{error::ReadlineError, Editor};

fn main() -> Result<()> {
	ensure!(
		matches!(
			env::var_os("COLORTERM")
				.context("$COLORTERM is not set!")?
				.to_str(),
			Some("truecolor") | Some("24bit")
		),
		"Your terminal does not support 24-bit true color!"
	);

	let mut rl = Editor::<()>::new();
	println!("Welcome! Enter a sequence of HSL values like so: `200,50,32` to get started, and `exit` to exit!");

	loop {
		let readline = rl.readline("conv> ");

		match readline {
			Ok(line) => {
				if line == "exit" {
					break;
				}

				let mut values = line.split(",").map(|s| s.parse::<u16>());
				let mut _get = || {
					values
						.next()
						.context("Failed to read next integer from input!")?
						.context("Failed to parse integer!")
				};

				let hsl = Hsl::new(_get()?, _get()? as u8, _get()? as u8)?;
				let rgb = hsl.to_rgb();

				// Print in true color!
				println!(
					"\x1b[38;2;{};{};{}mHello, world!\x1b[0m",
					rgb.red, rgb.green, rgb.blue
				)
			}
			Err(ReadlineError::Interrupted) => {
				eprintln!("CTRL-C");
				break;
			}
			Err(ReadlineError::Eof) => {
				eprintln!("CTRL-D");
				break;
			}
			Err(err) => {
				eprintln!("Error: {:?}", err);
				break;
			}
		}
	}

	Ok(())
}
