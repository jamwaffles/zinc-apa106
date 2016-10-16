#![crate_type = "staticlib"]
#![feature(plugin, start, core_intrinsics)]
#![no_std]
#![plugin(macro_platformtree)]

extern crate zinc;

use zinc::hal::spi::Spi;
// use zinc::hal::pin::Gpio;
use zinc::hal::timer::Timer;
use zinc::drivers::chario::CharIO;
use zinc::hal::tiva_c::spi;
// use zinc::hal::tiva_c::pin::reg;

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

			// d {
			// 	spi_tx@3 {
			// 		direction = "out";
			// 		function = 2;
			// 	}
			// }
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

// From https://forums.electricimp.com/discussion/1738/controlling-ws2812-leds-how-to-do-faster
// Modified by me
// const ONBIT: u8 = 0b11111100;		// 1.2 / .25us = 1.45us, 83% on time
// const OFFBIT: u8 = 0b11000000;	// 0.5 / .91us = 1.41us, 35% on time

fn run(args: &pt::run_args) {
	let spi = spi::Spi::new(spi::SpiId::Spi0);

	args.uart.puts("Started\r\n");

	// args.spi_tx.enable_pullup();
	// args.spi_tx.enable_pulldown();

	loop {
		// args.uart.puts("Loop\r\n");
		// args.uart.puti(args.timer.get_counter());

		// let value: u8 = (ws_low << 4) | ws_low;

		for _ in 0..24 {
			spi.write(0b00010001);
		}

		args.timer.wait_us(60);

		// for i in 0..4 {
			// args.spi_tx.set_low();
			// args.spi_tx.set_function(reg::Port_afsel_afsel::PERIPHERAL);

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

			// args.spi_tx.set_function(reg::Port_afsel_afsel::GPIO);

			// args.spi_tx.set_high();

		// }

		// for i in 0..4 {
		// 	spi.write(0b00010001);
		// }

		// for i in 0..4 {
		// 	spi.write(0b00010001);
		// }

		// spi.write(0b0000000000010001);

		// for i in 0..4 {
		// 	spi.write(0b00010001);
		// }

		// for i in 0..4 {
		// 	spi.write(0b00010001);
		// }

		args.timer.wait(1);

		// // toggles pin values
		// args.led1.set_high();
		// args.led2.set_low();
		// // wait for 1 second
		// (args.timer as &zinc::hal::timer::Timer).wait(1);

		// args.led1.set_low();
		// args.led2.set_high();
		// (args.timer as &zinc::hal::timer::Timer).wait(1);
	}
}