# Delay

`embedded-hal` provides the
[`delay`](https://docs.rs/embedded-hal/0.2.3/embedded_hal/blocking/delay/index.html)
trait for generating precise delays. The HAL provides an implementation which
you can access by using the
[`SYST`](https://docs.rs/cortex-m/0.6.2/cortex_m/peripheral/struct.SYST.html) in
the
[`Peripherals`](https://docs.rs/lpc8xx-hal/0.6.1/lpc8xx_hal/struct.Peripherals.html)
for the
[`Delay`](https://docs.rs/lpc8xx-hal/0.6.1/lpc8xx_hal/delay/struct.Delay.html)
struct.

Try using it to blink the led and check if it really delays the right amount of time.

## Driver crate

We can now use other driver crates to add more functionality. For this example,
we're using the [`embedded-morse`](https://crates.io/crates/embedded-morse)
crate to output morse messages over the led.

Add the crate to the `Cargo.toml` to the `[dependencies]` section and make an
instance in the `src/main.rs`:
``` rust
let mut delay = lpc8xx_hal::delay::Delay::new(p.SYST);
// the last argument inverts the output, since the led is active-low
let mut morse = embedded_morse::Morse::new_default(delay.clone(), led, true);
```

Now output something using
[`output_str`](https://docs.rs/embedded-morse/0.1.0/embedded_morse/struct.Morse.html#method.output_str).

The `embedded-morse` driver uses *only* traits from `embedded-hal`. Because of
that, it will work on a large swath of microcontrollers, without needing to
modify it.
