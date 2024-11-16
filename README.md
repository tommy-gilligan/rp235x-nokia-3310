# RP235x Nokia 3310 Adapter Board

This started as a cute "what if?":

> What if I could type out messages on my smart phone by using tactile buttons like I used to have on my phone back in high school?

Because the board would need to fit into the chassis of a Nokia 3310 and because there are existing doodads that would be sitting alongside the board anyways, scope has crept to include connecting to these other thingamajigs.

![3D render of front of adapter board](./front.jpg)
![3D render of back of adapter board](./back.jpg)

### TODO
#### Hardware
- rtc (rp2040 had one, rp235x does not)

##### Power
- detect battery type to refuse NiMH
- LiPo charging
- optional pico-w for wifi/bluetooth (using a module avoids need for recertification?)
    - looks like RP will release such a module so go ahead with designing with that in mind
- power button (currently it is a second boot button)
- power from battery

#### Software
- finish snake
- QR code for TOTP
- use TRNG for TOTP/snake
- use text_input for inputing secret for TOTP (drives the need for inputting numeric digits easily)
- use embedded_graphics for console display
- finish no_std RTTTL parsing library and release
- create example that plays RTTTL (and writes it to screen)
- read/write to sim card (PIO)
- bring back USB HID
- rp2235x WORM for password management?

#### Misc
- Come up with better name
    - Looks like I am targeting 2350a right now (due to IC availability)
    - Then targeting 2354b (due to marginally increased simplicity for PCB design)
    - Could then target 235[04]b

https://serdisplib.sourceforge.net/ser/pcd8544.html
