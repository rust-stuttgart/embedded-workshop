# Background

## What's a microcontroller?

A microcontroller is a *system* on a chip. Whereas your laptop is made up of several discrete
components: a processor, RAM sticks, a hard drive, an ethernet port, etc.; a microcontroller has all
those components built into a single "chip" or package. This makes it possible to build systems with
minimal part count.

## What can you do with a microcontroller?

Lots of things! Microcontrollers are the central part of systems known as *embedded* systems. These
systems are everywhere but you don't usually notice them. These systems control the brakes of your
car, wash your clothes, print your documents, keep you warm, keep you cool, optimize the fuel
consumption of your car, etc.

The main trait of these systems is that they operate without user intervention even if they expose a
user interface like a washing machine does; most of their operation is done on their own.

The other common trait of these systems is that they *control* a process. And for that these systems
usually have one or more sensors and one or more actuators. For example, an HVAC system has several
sensors, thermometers and humidity sensors spread across some area, and several actuators as well,
heating elements and fans connected to ducts.

## When should I use a microcontroller?

All these application I've mentioned, you can probably implement with a Raspberry Pi, a computer
that runs Linux. Why should I bother with a microcontroller that operates without an OS? Sounds like
it would be harder to develop a program.

The main reason is cost. A microcontroller is much cheaper than a general purpose computer. Not only
the microcontroller is cheaper; it also requires many fewer external electrical components to
operate. This makes Printed Circuit Boards (PCB) smaller and cheaper to design and manufacture.

The other big reason is power consumption. A microcontroller consumes orders of magnitude less power
than a full blown processor. If your application will run on batteries that makes a huge difference.

And last but not least: (hard) *real time* constraints. Some processes require their controllers to
respond to some events within some time interval (e.g. a quadcopter/drone hit by a wind gust). If
this *deadline* is not met, the process could end in catastrophic failure (e.g. the drone crashes to
the ground). A general purpose computer  running a general purpose OS has many services running in
the background. This makes it hard to guarantee execution of a program within tight time constraints.

## When should I *not* use a microcontroller?

Where heavy computations are involved. To keep their power consumption low, microcontrollers have
very limited computational resources available to them. For example, some microcontrollers don't
even have hardware support for floating point operations. On those devices, performing a simple
addition of single precision numbers can take hundreds of CPU cycles.

## What's Rust?

To quote Wikipedia: "Rust is a multi-paradigm system programming language
focused on safety, especially safe concurrency. Rust is syntactically similar to
C++, but is designed to provide better memory safety while maintaining high
performance."

Rust is a relatively new programming language, that tries to tackle the fields
long held by C/C++: Low-level programming, without much overhead. In contrast
it's much more ambitious in it's memory management: It tries to do it's best to
ensure that *no* memory corruption is possible. This includes race conditions,
buffer overflows, use after free and much more.

It also has many other benefits that a lot of other newer languages share: An
integrated build tool, `cargo`,  an opinionated
code formatter, `rustfmt`, that much of the community has converged to and
dependency management, that simplifies sharing code & common interfaces.
