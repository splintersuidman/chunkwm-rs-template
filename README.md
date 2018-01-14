# `chunkwm-rs` plugin template
A template for using [`chunkwm-rs`](https://github.com/splintah/chunkwm-rs).

## Usage
- Clone [chunkwm](https://github.com/koekoeishiya/chunkwm).
- Clone this repository into `chunkwm/src/plugin`.
- `cd` into it.
- Run `make install`.
- Copy `bin/template.so` to `your-chunkwm-plugins/template.so` (where 'your-chunkwm-plugins' should probably be `~/.chunkwm_plugins`, or the homebrew directory).
- Run `chunkc core::load template.so`.

To change the plugin, edit `src/lib.rs`.
