# egui + glow SDL Hook Proof of Concept

This is a proof of concept for integrating [egui](https://github.com/emilk/egui) and [glow](https://github.com/grovesNL/glow) into SDL by hooking the shared library functions. This code is a proof of concept and works for my specific case/process, but will not work without some modification, as there is no easy universal way to resolve the SDL function jump table. That is an exercise left for the reader.

This project uses the SDL2-sys bindings for as a way to interact with the external C functions. This project is super unsafe, and the `unsafe` keyword is used liberally throughout the program (not even well, most functions should have unsafe in their declaration and not just wrapping the body, as calls to those functions are inherently unsafe, however the latter option cuts down on verbosity which was beneficial for this proof of concept). This is because poking around the arbitrary points in memory of another running process is not very safe.

It hooks SDL_SwapWindows and SDL_PollEvent. SDL_SwapWindows is functional, and SDL_PollEvent is mostly functional with the exception of key events which I ran out of steam to do.

Hopefully this code will be of use to someone.

## Inspiration

People often present Rust as a comparable language to C/C++, and I wanted to test that. I wanted to write Rust in a situation where everything is unsafe, and see how it felt, or even if it worked. Needless to say it works, and honestly doesn't feel that worse to write than C/C++ does.
