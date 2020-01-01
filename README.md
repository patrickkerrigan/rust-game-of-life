An implementation of Conway's Game of Life using Rust and the Allegro graphics library so I could experiment with 
building games in Rust.

### Building

To build, you'll need an installation of Rust (1.40.0+ should be fine) and the Allegro 5 binaries and headers for your platform.

Set the following environment variables to point to your Allegro installation:

```
ALLEGRO_INCLUDE_DIR=/path/to/allegro/include
ALLEGRO_LINK_DIR=/path/to/allegro/lib
```

Build with cargo:

```
cargo build --release
```

Copy the Allegro shared libraries/DLLs to the `target/release` folder

Run the executable

### Controls

`R` - Reset

`P` - Pause

`+` - Increase speed

`-` - Decrease speed

`Esc` - Exit

### Compatibility

I've only tested this on 64 bit windows. YMMV.
