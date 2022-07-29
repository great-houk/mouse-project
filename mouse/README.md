# Mouse

This is the main code repository for the mouse. This is what runs on the mouse and does literally everything. This directory contains several sub crates all of which implement and provide basic APIs to various MAX32625 features which where required to build this project. To change any functionality with the mouse or fix any bugs, this is where you want to be.

Goals:
1. The scroll wheel is currently rather buggy at any speed except medium, so possible come up with a better way to debounce/denoise the signals coming from the GPIOs.
2. Refactor, refactor, refactor. While the code in here isn't horrible, it isn't great. Many things could be rewritten in better and more efficient ways, especially the USB implementation.
