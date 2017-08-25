# cargo-board

A cargo subcommand for managing embedded boards

## Demo
The configuration for boards is specified in a `boards.toml` such as:

```toml
[soc]
STM32F2XXXB = { arch = "thumbv7m-none-eabi", flash = "128K", ram = "64K" }
STM32F2XXXE = { arch = "thumbv7m-none-eabi", flash = "512K", ram = "128K" }

[board]
flexperiment_mini = { soc = "STM32F2XXXB" }
netboard = { soc = "STM32F2XXXE" }
```

## Usage
`cargo board [board_name] <ARGUMENTS TO CARGO>`

e.g. `cargo board netboard build -p blink`

## What it does
- enables the `board_[board_name]` feature of the compiled crate (passes `--feature board_[board_name]` to cargo)
- passes `--target [soc.arch]` to cargo
- sets `CARGO_TARGET_DIR` to `target/[board_name]`  
  (TODO: this leads to duplication of compiled shared crates)
- exposes `LD_FLASH_SIZE` and `LD_RAM_SIZE` as environmental variables to cargo so it can be used in build scripts  
  (TODO: make this generic and not hardcoded attributes?)
- exposes `BOARD_FEATURES` so we can expose the following features  
  (exposed using rustc, so only available in code and not in cargo-configuration):
   - the `soc_[soc_name]` feature, you can check for a specific soc using `#[cfg(feature="soc_[soc_name]")]`   

  [This needs to be in your build.rs](https://github.com/steffengy/cargo-board/blob/c56b2a0ff1b0da3c91c49fb15af132191529d139/src/main.rs#L75-L83).
  
