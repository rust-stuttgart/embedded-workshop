# GPIO Input
Until now, the GPIO pins were only used in the output state. As the name implies,
they can also be used as inputs. And, as was already covered, there are multiple
buttons on the board we can use.

## [Contact bounce](https://en.wikipedia.org/wiki/Switch#Contact_bounce)
As buttons are phyiscal devices, they're unfortunately not perfect. When
pressing or depressing a button it flips between the pressed & not state for a
short amount of time, which needs to be kept in mind. The act of filtering this
out is called debouncing, a common way to do it is to simply wait a few
milliseconds after the input is changed and ignore all the changes in the
meantime.

## Controlling the led
You can switch gpio pin to input mode with:
TODO: impl
Now, the `InputPin` trait can be used to check the state. Try controlling the
led with the button, so it's on when the button is pressed and off when not.

Next, try to switch the led state with `ToggleableOutputPin`.
TODO: impl

Here, you'll need to consider debouncing the input signal, since otherwise the led can be
toggled so fast, that you won't notice.
