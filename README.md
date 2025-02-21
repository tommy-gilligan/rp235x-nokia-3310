# RP235x Nokia 3310 Adapter Board

This started as a cute "what if?":

> What if I could type out messages on my smart phone by using tactile buttons like I used to have on my phone back in high school?

Because the board would need to fit into the chassis of a Nokia 3310 and because there are existing doodads that would be sitting alongside the board anyways, scope has crept to include connecting to these other thingamajigs.

![3D render of front of adapter board](./front.jpg)
![3D render of back of adapter board](./back.jpg)

### Ordering Board

### TODO
#### Sooner
##### Hardware
- Come up with better name
- dress up repo
- create example that plays RTTTL (and writes it to screen)
- USB-C (USB needs to be replaced anyways, JLCPCB does not stock part in current design)
- swap positions of USB with debug connectors
- move debug connections further inside chassis
- LiPo charging
- power button (circuit 'down-stream' from charging, jumper for bypass)
- main speaker connection
- backlight LEDs (many options but prudent to go with whatever uses least power, is simplest)
- accomodate headset connection TRRS 2.5 (probably just swap for 3.5): mic shares onboard mic connection.  left channel = big speaker, right channel = buzzer
- accomodate onboard microphone.  use TRRS connection as 'breakout'

##### Software
- Snake
- USB text entry
- use text_input for inputing secret for TOTP (drives the need for inputting numeric digits easily and RTC)

#### Later
- optional pico-w for wifi/bluetooth (using a module avoids need for recertification?)
    - looks like RP will release such a module (RM2) so go ahead with designing with that in mind
- detect battery type to refuse NiMH
- battery gauge
- flex pcb for keypad?
- LTE modem?
- e-ink display?

https://serdisplib.sourceforge.net/ser/pcd8544.html
