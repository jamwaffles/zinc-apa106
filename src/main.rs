#![crate_type = "staticlib"]
#![feature(plugin, start, core_intrinsics)]
#![no_std]
#![plugin(macro_platformtree)]

extern crate zinc;

use zinc::hal::spi::Spi;
use zinc::hal::timer::Timer;
use zinc::drivers::chario::CharIO;
use zinc::hal::tiva_c::spi;
use zinc::hal::tiva_c::sysctl;

platformtree!(
	tiva_c@mcu {
		// Tiva C ends up with an 80MHz clock from 16MHz external xtal and x5 PLL
		clock {
			source = "MOSC";
			xtal   = "X16_0MHz";
			pll    = true;
			div    = 5;
		}

		gpio {
			f {
				led1@1 { direction = "out"; }
				led2@2 { direction = "out"; }
			}

			a {
				uart_rx@0 {
					direction = "in";
					function  = 1;
				}

				uart_tx@1 {
					direction = "in";
					function  = 1;
				}

				spi_ck@2 {
					direction = "out";
					function  = 2;
				}

				spi_cs@3 {
					direction = "out";
					function  = 2;
				}

				spi_rx@4 {
					direction = "in";
					function  = 2;
				}

				spi_tx@5 {
					direction = "out";
					function  = 2;
				}
			}
		}

		timer {
			// The mcu contain both 16/32bit and "wide" 32/64bit timers.
			timer@w0 {
				// Prescale sysclk (here 80MHz) to 1Mhz since the wait code expects 1us granularity
				prescale = 80;
				mode = "periodic";
			}
		}

		uart {
			uart@0 {
				mode = "115200,8n1";
			}
		}
	}

	os {
		single_task {
			loop = "run";
			args {
				timer = &timer;
				spi_tx = &spi_tx;
				uart = &uart;
			}
		}
	}
);

struct Apa106Led {
	red: u8,
	green: u8,
	blue: u8,
}

const ON_BYTE: u8 = 0b1111_1100;
const OFF_BYTE: u8 = 0b1100_0000;

fn bit_is_set(byte: u8, bit_index: u8) -> bool {
	(byte & (1 << bit_index)) != 0
}

/// Send a Colour struct out the SPI port
/// Each byte in a colour triplet is converted into 8 nibbles and sent as 4 sequential bytes down the SPI line
fn colour_to_byte_raw(input: &Apa106Led) -> [u8; 24] {
	// ((a << 4) | (b & 0b1111)).toString(2)

	let mut bytes: [u8; 24] = [0; 24];

	// SPI transmits MSB first
	for pos in 0..8 {
		bytes[7 - pos as usize] = if bit_is_set(input.red, pos as u8) { ON_BYTE } else { OFF_BYTE };

		bytes[8 + (7 - pos as usize)] = if bit_is_set(input.green, pos as u8) { ON_BYTE } else { OFF_BYTE };

		bytes[16 + (7 - pos as usize)] = if bit_is_set(input.blue, pos as u8) { ON_BYTE } else { OFF_BYTE };
	}

	bytes
}

const ON_NIBBLE: u8 = 0b1110;
const OFF_NIBBLE: u8 = 0b1000;

/// Send a Colour struct out the SPI port
/// Each byte in a colour triplet is converted into 8 nibbles and sent as 4 sequential bytes down the SPI line
fn colour_to_raw(input: &Apa106Led) -> [u8; 12] {
	// ((a << 4) | (b & 0b1111)).toString(2)

	let mut bytes: [u8; 12] = [0; 12];

	// SPI transmits MSB first, so first bit = upper nibble
	for pos in 0..4 {
		let red_upper = if bit_is_set(input.red, pos * 2 + 1) { ON_NIBBLE } else { OFF_NIBBLE };
		let red_lower = if bit_is_set(input.red, pos * 2) { ON_NIBBLE } else { OFF_NIBBLE };

		bytes[3 - pos as usize] = (red_upper << 4) | (red_lower & 0b1111);

		let green_upper = if bit_is_set(input.green, pos * 2 + 1) { ON_NIBBLE } else { OFF_NIBBLE };
		let green_lower = if bit_is_set(input.green, pos * 2) { ON_NIBBLE } else { OFF_NIBBLE };

		bytes[4 + (3 - pos) as usize] = (green_upper << 4) | (green_lower & 0b1111);

		let blue_upper = if bit_is_set(input.blue, pos * 2 + 1) { ON_NIBBLE } else { OFF_NIBBLE };
		let blue_lower = if bit_is_set(input.blue, pos * 2) { ON_NIBBLE } else { OFF_NIBBLE };

		bytes[8 + (3 - pos) as usize] = (blue_upper << 4) | (blue_lower & 0b1111);
	}

	bytes
}

fn run(args: &pt::run_args) {
	let spi = spi::Spi::new(spi::SpiConf {
		peripheral: spi::SpiId::Spi0,

		frequency: 2_339_181
	});

	args.spi_tx.enable_pulldown();

	args.uart.puts("Started\r\n");

	let mut counter = 0;

	let blank = Apa106Led { red: 0x00, green: 0x00, blue: 0x00 };
	let blank_bytes = colour_to_raw(&blank);

	for _ in 0..64 {
		for byte in blank_bytes.into_iter() {
			spi.write(*byte);
		}
	}

	loop {
		args.timer.wait(1);

		let led = Apa106Led { red: counter, green: 0x00, blue: 0x00 };

		for _ in 0..64 {
			for byte in colour_to_raw(&led).into_iter() {
				spi.write(*byte);
			}
		}

		counter += 1;

		if counter > 250 {
			counter = 0;
		}
	}
}