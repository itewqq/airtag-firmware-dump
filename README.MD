# Airtag glitcher and dumper

## Hardware prerequisites:
- An airtag
- A `probe-rs` compatible debug adapter such as a J-Link
- A Raspberry Pi 3b+ 
- An NFET

>Other versions of Pi will also work, but you need to adjust the corresponding pins yourself.

# Usage

### 1. Setting the hardware connection
Connect the following pins from the Raspberry Pi 3b+ to the airtag (`[test point numbering](https://github.com/colinoflynn/airtag-re#test-points)):
| Function | Raspberry Pi 3b+  | Airtag |
|----------|-----|--------|
| Glitch output | wiringPi 3 | 28 (using an NFET) |
| Trigger | wiringPi 2 | 34 (1.8V) |
| Power | wiringPi 0 | VCC1 + VCC2 |

### 2. Start the glitcher

Copy the ```airtag-glitcher``` folder to your Raspberry Pi 3b+, enter it and execute the ```run.sh```.

### 3. Start the dumper

Copy the ```airtag-dump``` folder to your computer where the SWD adapter connected and run ```Cargo run```. Next, pray that your glitch will succeed :)

# Credits
 - [pd0wm](https://github.com/pd0wm/airtag-dump) for his original stm32-version dumper
 - [LimitedResults](https://twitter.com/LimitedResults) for their original research into glitching the NRF52: https://limitedresults.com/2020/06/nrf52-debug-resurrection-approtect-bypass/
 - [Colin O'Flynn](https://twitter.com/colinoflynn) for documenting the test points: https://github.com/colinoflynn/airtag-re
 - [stacksmashing](https://twitter.com/ghidraninja) for his video explaining the procedure: https://www.youtube.com/watch?v=_E0PWQvW-14
