<p align="center">
  <img src="assets/TannhauserHappy.png">
</p>

# machitan matsuri mambo

An Uma Musume fan rhythm game where you help Machikane Tannhauser mun along to songs.

## Controls

Machitan uses the home row keys `ASDF` and `JKL;` for input; each key is mapped to a single lane.

## Building

`machitan` currently defaults to building Bevy as a dynamic library for rapid testing purposes. To build with this structure simply run `cargo build --release` to make the initial build (which will likely take a while), and `cargo run --release` to run after the dynamic dependencies are built.

Alternatively, you can edit `Cargo.toml` and remove the `dynamic_linking` feature to compile a single binary with `cargo build --release` and `cargo run --release` if you only want to build the game a single time.
