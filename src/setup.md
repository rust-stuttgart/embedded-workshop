# Development environment setup

Dealing with microcontrollers involves several tools,
as we'll be dealing with an architecture different than your laptop's,
and we'll have to run and debug programs on a "remote" device.

## Documentation

Without documentation it is pretty much impossible to work with microcontrollers.

## Tools

We'll use all the tools listed below. Where a minimum version is not specified,
any recent version should work but we have listed the version we have tested.

- Cargo & `rustc`.
- `picocom` or similar on linux and macOS
- `PuTTY` on Windows.

Next, follow OS-agnostic installation instructions for a the tools:
## Tools


Install rustup by following the instructions at [https://rustup.rs](https://rustup.rs).

Follow the instructions to get the latest stable version of rust.
To make sure, just execute
``` sh
rustup default stable
```

Then you'll need to install the `thumbv6m-none-eabi` target for the cortex-m0
cpu inside the lpc845:
``` sh
rustup target add thumbv6m-none-eabi
```

Then we'll install `cargo-flash` using cargo:
``` sh
cargo install cargo-flash
```

It's from the great [probe-rs](https://github.com/probe-rs/probe-rs) project,
and is used for getting your code onto the microcontroller.

### Linux
Unfortunately, most linux distributions don't allow non-root applications to
talk directly to usb devices. This means, you'd need to use root to talk to the
debug probe, but we can just use a udev rule to fix this for these devices.

You wanna add the following into a file under `/etc/udev/rules.d/`, for example `70-cmsis-dap.rules`

```
# CMSIS-DAP devices
SUBSYSTEMS=="usb", ATTRS{product}=="*CMSIS-DAP*", GROUP="wheel", MODE="0660"
```

Now tell udev to reload the rules with
```sh
sudo udevadm control --reload
```
