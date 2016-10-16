#![crate_type = "staticlib"]
#![feature(plugin, start, core_intrinsics)]
#![no_std]
#![plugin(macro_platformtree)]

extern crate zinc;

use zinc::hal::spi::Spi;
use zinc::hal::timer::Timer;
use zinc::drivers::chario::CharIO;
use zinc::hal::tiva_c::spi;

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

struct Colour {
	r: u8,
	g: u8,
	b: u8,
}

fn colour_to_raw(input: Colour) -> Vec<u8> {

}

fn run(args: &pt::run_args) {
	let spi = spi::Spi::new(spi::SpiId::Spi0);

	args.uart.puts("Started\r\n");

	loop {
		for _ in 0..24 {
			spi.write(0b00010001);
		}

		args.timer.wait_us(60);

		spi.write(0b01110111);
		spi.write(0b01110111);
		spi.write(0b01110111);
		spi.write(0b01110111);

		spi.write(0b00010001);
		spi.write(0b00010001);
		spi.write(0b00010001);
		spi.write(0b00010001);

		spi.write(0b01110111);
		spi.write(0b01110111);
		spi.write(0b01110111);
		spi.write(0b01110111);

		// ---

		spi.write(0b00010001);
		spi.write(0b00010001);
		spi.write(0b00010001);
		spi.write(0b00010001);

		spi.write(0b01110111);
		spi.write(0b01110111);
		spi.write(0b01110111);
		spi.write(0b01110111);

		spi.write(0b00010001);
		spi.write(0b00010001);
		spi.write(0b00010001);
		spi.write(0b00010001);

		args.timer.wait(1);
	}
}