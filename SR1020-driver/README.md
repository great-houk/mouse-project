# SR1020

A currently not implemented driver for the SR1020 UWB radio. My goal for this is to draw inspiration from the [official spark SDK](https://www.sparkmicro.com/sdk-docs/), but provide lower level access to the hardware. It should be agnostic to whatever networking protocol you decide to implement on top of it. There is also an overall goal for this project to move everything from custom traits to embedded_hal traits, but currently I don't know if that's worth it.
