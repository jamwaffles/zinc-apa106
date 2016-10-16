# Zinc on a Stellaris Launchpad/Tiva C

This project contains example code to flash the red and blue LEDs on a Stellaris Launchpad using the [Zinc](https://zinc.rs) embedded development framework. It should also work on a Tiva C, which is supposedly binary- and pin-compatible with the Stellaris Launchpad, but I've been unable to test as I don't have a Tiva board yet.

## Building

You need to have the `arm-none-eabi-gcc` toolchain installed as per the usual Zinc setup instructions. You can install it on macOS with `brew install gcc-arm-none-eabi`.

Once that's done, run `make` which will use Cargo to build the project and produce a binary file ready for flashing to the device.

The flashable bin file will be in `/target/thumbv7em-none-eabi/release/zinc_from_scratch_launchpad.bin` (note the `.bin` extension).

## Flashing

I'm using a Mac, so flashing is done by acquiring [lm4tools](https://github.com/utzig/lm4tools), compiling it with `make` and running `./lm4tools/lm4flash/lm4flash /path/to/binary.bin`.

If all has gone well you should see the red and blue LEDs flashing alternately.