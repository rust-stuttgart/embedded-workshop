# Meet your hardware
TODO Add an annotated picture
## Parts
The LPC845-BRK consists of two microcontrollers:

-  LPC845

    That's the chip you'll be programming

- LPC11U35

    That's the chip in the upper left corner. It acts as a debug bridge,
    allowing you to program, debug and communicate with the LPC845. It runs a
    firmware that implements the
    [CMSIS-DAP](http://www.keil.com/support/man/docs/dapdebug/dapdebug_introduction.htm) protocol.

It also has some extra things:

- RGB Led

  This small thing contains three leds, red, green and blue

- Buttons

  Things you can press

- Potentiometer

  An adjustable resistor, allows you to set a voltage by using it as a voltage divider

- Capacitive Button

  A button that can be activated by touching it. Unfortunately, getting it set
  up is pretty involved, so we're not going to do that.
