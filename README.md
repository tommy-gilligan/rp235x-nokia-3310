# RP2040 Nokia 3310 Adapter Board

This started as a cute "what if?":

> What if I could typeout messages on my smartphone by using tactile buttons like I used to have on my phone back in high school?

Because the board would need to fit into the chassis of a Nokia 3310 and because there are existing doodads that would be sitting alongside the board anyways, scope has crept to include connecting to these other thingamajigs.

### TODO
#### Hardware
- double-check display connection
- use on-board 2350 instead of connected pico board.
- add pads for more buttons: up/down/select
- sim card connection
- power from battery
- power button (currently it is a second boot button)
- detect battery type to refuse NiMH
- LiPo charging
- add LEDs to lightup keypad and display
- optional pico-w for wifi/bluetooth (using a module avoids need for recertification?)

### Software
- finish no_std RTTTL parsing library and release
- use embedded_graphics for console display
- create example that plays RTTTL (and writes it to screen)
- create keypad abstraction
- test fake keypresses to create multi-tap converter
- implement USB keyboard
- interface with optional pico-w to expose keypad as bluetooth keyboard (speaker for notifications)
- read/write to sim card

### Misc
- Come up with better name
