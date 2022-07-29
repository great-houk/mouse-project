# Custom Mouse V1.0

This project is an all contains every piece you would need to fully order, assemble, and program a completely open source mouse.

This is only the first version of this mouse, and the end goal of this project is no where near complete. However, this first version does 100% work 90% of the time, so the inital goal of making a mouse is complete. In fact, I'm using it to write this readme. Pretty neat.

My immediate goal is to implement the wireless protocol over the SR1020 radio.

Future versions of this project won't use a barebones $15 dollar mouse off of amazon, and will have much higher quality everything, while also hopefully being cheaper. I used a lot of parts on this, like the MAX32625, because they looked cool. The 32625 has a one of a kind PMU, which is something like another set of super low power cores you can manually program through assembly. While this seems awesome at first, and like the perfect tool to reach extra efficient power targets, in the end the sensor draws >20mW anyway, so the difference of ~1-2mW the PMU provides is useless, and so it the extra cost.

# Notes for making your own

This is the mouse I bought and used as a base: [Amazon Link](https://www.amazon.com/gp/product/B07ZHFCM43/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&th=1)

For obtaining the other parts:
1. Most mouse parts can be desoldered from the mouse itself. I only bought one switch for the dongle off of digikey.
2. All other digital parts except the sensor can be obtained from digikey for reasonable prices, and the objects in the Kicad PCB should have digikey links associated with them already.
3. The mouse sensor is hard to get. If you are a student like me, you might be able to get a free sample from Pixart. If you aren't there are a couple of ebay postings or other second hand sellers which might offer them, but they take a while (>month normally) to ship. Pixart does offer to sell the sensors directly, but there is a minimum order quantity of 25.
4. I used JLCPCB to obtain the PCB. I had them make a stencil as well, in order to greatly simply to soldering process. For anyone in the US, chances are JLC is your best bet, for Europe, I've heard of other cheap PCB manufacturers which might do a better job, as long as the shipping isn't prohibitively expensive.
5. Programming the MCU. I used the [MCU Link](https://www.nxp.com/design/microcontrollers-developer-resources/mcu-link-debug-probe:MCU-LINK) with OpenOCD to program the chip, which had all the features I ended up needing.

The PCB is specifically designed around fitting in the mouse, and even then there were some mistakes I made. Most notably, the top right post hole was misplaced, so that needs to be cut flush so the PCB can rest flat. The antenna also sticks out the side, so you need to cut some small segments off of the casing in order to have it fit. Finally, the USB-C port is in a slightly lower and different place, so you need to cut the sides of the USB port out to have it fit and be accessible.

It took me many tries to assemble this properly, partially because it was my first time using solder past (which by the way is 100% what you need to use to solder this), and partially because there are no good resources out there for solder of this kind, at least that I could find. If you order the PCB from JLCPCB like I did, make sure to get a stencil, as it will make everything signifigantly easier. There is no need to order a stencil for the back, however, that only requires some careful soldering.

The most important parts when assembling the board are
1. Make sure you spread the paste over the stencil the fewest possible times, preferably in one strong, slow swipe. A good solder paste application makes this process signifigantly easier.
2. Don't leave the paste sitting for too long. You can take your time placing the parts, but after around 2-3 hours the paste will start drying and harden, making parts have a signifigantly higher risk of being blown away when using the heat gun.
3. Make sure to not have the temps too high. I broke my first few boards by creating shorts through burned flux. Don't do what I did.

And that's mostly it. There are other resources out there to help with the specifics, but that covers most of the general details.

# Project Structure

Each folder has it's own README which describes what it does, how it works, and how to use it, except the PCB.
