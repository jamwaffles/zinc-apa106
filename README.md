# Rusty APA106

This project is a scratchpad for developing control software for the APA106 serially controllable LED in Rust.

It currently depends on a weird local build of Zinc so is probably not out-of-the-box buildable right now. That should change when [my pull request adding SPI support](https://github.com/hackndev/zinc/pull/401) is merged into the main Zinc project.

## APA106 timing

The APA106 is similar to the WS2812 in that is uses a single wire, timing-based bus to operate, however the timings are slightly different. Cycle time is 1.71us per bit, and a 1 or 0 is dictated by the duty cycle as below:

| Description | Time |
| --- | --- |
| 0 bit on time | 0.35us
| 0 bit off time | 1.36us
| 1 bit on time | 1.36us
| 1 bit off time | 0.35us

There are a lot of libraries out there that use finely tuned assembly routines to generate the correct signalling, however I took the same approach as [Espruino](http://www.espruino.com/WS2811) and used the SPI bus on the TM4C123GH6PM micro. To generate the correct waveform I use two different nibbles (MSB sent first); `0b1000` is an "off" pulse and `0b1110` is an "on" pulse.