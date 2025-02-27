# Brique: RP235x Nokia 3310 Adapter Board

This started as a cute "what if?":

> What if I could type out messages on my smart phone by using tactile buttons like I used to have on my phone back in high school?

Because the board would need to fit into the chassis of a Nokia 3310 and because there are existing doodads that would be sitting alongside the board anyways, scope has crept to include connecting to these other thingamajigs.

![3D render of front of adapter board](./front.jpg)
![3D render of back of adapter board](./back.jpg)

## [Simulation](https://tommy-gilligan.github.io/brique/simulation)
## [API](https://tommy-gilligan.github.io/brique/doc/shared)

## Setting up web environment

## Setting up rp environment

## Setting up board
### Ordering Board
### Disassemble 3310
### Install Hardware Test Program
### Reassemble 3310
### Manual Test
### Install Custom Software

### TODO
#### v0.3
- app menu
- create example that plays RTTTL (and writes it to screen)
- main branch: logging via USB
- add backlight to API
- Snake
- USB text entry (just on branch because USB app API needs some thought nb. there's maybe a need for device to add/remove class depending on app.  is that possible? what does web simulated device look like?)

- power button (digital latch.  can this be triggered by 'any key'?  ie. any keypad press turns the device on.  there's enough GPIO to spare that we should have a dedicated GPIO for any key too)
- double check power regulation, boot button
- backlight LEDs (many options but prudent to go with whatever uses least power, is simplest.  use transistor to drive LED directly from battery/power)
- add back supercap rtc
- add jlcpcb part numbers, 3d models
- double check usb footprint
- pass more KiCAD checks in CI

- terse, unfriendly instructions (ie. README)
- document app API

#### Later
- optional pico-w for wifi/bluetooth (using a module avoids need for recertification?)
    - looks like RP will release such a module (RM2) so go ahead with designing with that in mind
- detect battery type to refuse NiMH
- battery gauge
- mic connection
- use text_input for inputing secret for TOTP (drives the need for inputting numeric digits easily and RTC)
- power button used for BOOT/RUN?
- how should software versions synchronize with hardware versions. what level of compatibility should be supported.
- institute changelog
- use 'Issues' instead of README for tracking
- USB on simulated device
- optimise GPIO pin mapping.  shorten traces etc.
- connection plate for 3d printing
- increase flash capacity? i'd prefer to remove this part altogether by using rp2354 due for release later in the year
- bring more rigour to 'scheduler'

#### Much Later
- flex pcb for keypad?
- LTE modem?
- e-ink display?

https://serdisplib.sourceforge.net/ser/pcd8544.html
