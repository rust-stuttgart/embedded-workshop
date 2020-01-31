# GPIO Output
Most pins in a microcontroller are usually General Purpose Input Output pins.
These can either output signals or read signals applied to them. They can be
used for various applications, but in this case, we're using an led connected to
the P1.1 pin.

## Getting to blinky

1. Go to the `quickstart` folder & copy it somewhere else. This is the binary
   crate we're going to work with

2. Look at what's in there

   There are already various things in the folder. For now, we can pretty much
   ignore every non-source file, but if you're curious, this is why they're used

   - `.cargo/config`

     In here, the target architecture for our program is defined. See the
     embedonomicon for more details on architecture

   - `memory.x`

     This defines the amount and the location of flash & RAM for the target.

   - `openocd.cfg`

     Openocd needs some configuration to work. This contains it.


  So, what's in the `src/main.rs`?

  There are a couple things needed, that aren't used in normal rust programs.

  Let's go through them line-by-line:
  - `#![no_std]`

    The `std` library isn't implemented for microcontrollers, so it's disabled.

    This needs to be done in all binary & library crates

  - `#![no_main]`

    An embedded

3. The first thing that's needed when starting is getting access to the
   peripherals.

   ```rust
     let p = lpc8xx_hal::Peripherals::take().unwrap();
   ```

4. Next, the system controller `SYSCON`, `GPIO` and the switch matrix `SWM`, that's
   controlling what each pin is used for, are brought into a usable state.
   This means

   ```rust
     let swm = p.SWM.split();
     let mut syscon = p.SYSCON.split();
     let gpio = p.GPIO.enable(&mut syscon.handle);
   ```

5. The blue led is connected to `PIO1_1`. To use it as a gpio pin, it needs to be
   configured as such in the switch matrix.

   ``` rust
     let led = swm.pins.pio1_1.into_gpio_pin(&gpio);
   ```
6. Now that it's a gpio pin, it can be configured to act as an output
   ```rust
     let mut led = led.into_output();
   ```

7. Now all the needed setup is done, the only thing left is getting the LED to
   blink.

   This can be done by setting the pin high & low in a loop with the
   [`OutputPin`](https://docs.rs/embedded-hal/0.2.3/embedded_hal/digital/v2/trait.OutputPin.html)
   trait. Unfortunately this is way too fast for any human to see. It can be
   slowed down, by repeatedly setting it high/low in a loop, since setting the
   pin takes some amount of time. A value of 1 000 000 works well in release
   mode.

    ```rust
      loop {
          for _ in 0..1_000_000 {
              led.set_high().unwrap();
          }
          for _ in 0..1_000_000 {
              led.set_low().unwrap();
          }
      }
    ```

8. Last but not least, we still need to run & compile it. This can be done by
   executing `cargo flash --release --chip LPC845M301JBD48`. If everything was
   done correctly, the blue led should blink now.

Please note that the led is active low, it's on when the pin is set to 0V with `set_low()`.

Now you can try a few more things with your newly gained knowledge:
- Try blinking the other two leds, red on `PIO0_2` and green on `PIO0_0`
- By quickly turning the led on/off, the human eye perceives the led having a
  lowered brightness. This technique is called PWM and is almost always used for
  adjusting the brightness.
- …
