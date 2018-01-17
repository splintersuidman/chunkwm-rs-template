# `chunkwm-rs` plugin template
A template for using [`chunkwm-rs`](https://github.com/splintah/chunkwm-rs).

## Usage
- Clone this repository (`git clone https://github.com/splintah/chunkwm-rs-template`).
- Run `make install`.
- Copy `bin/template.so` to `your-chunkwm-plugins/template.so`, where 'your-chunkwm-plugins' should be replaced with the directory specified in your `chunkwmrc` after `chunkc core::plugin_dir`.
- Run `chunkc core::load template.so`.

To change the plugin, edit `src/lib.rs`.
