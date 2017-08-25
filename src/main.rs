use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process::Command;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

#[derive(Debug, Deserialize)]
struct Config {
    soc: BTreeMap<String, SoC>,
    board: BTreeMap<String, Board>,
}

#[derive(Debug, Deserialize)]
struct SoC {
    arch: String,
    flash: String,
    ram: String,
}

#[derive(Debug, Deserialize)]
struct Board {
    soc: String,
}

/// cargo board <board> ARGS
fn main() {
    let env_args: Vec<_> = env::args().into_iter().collect();

    assert_eq!(&env_args[1], "board");

    // read the configuration file
    let mut content = String::new();
    let _ = File::open("boards.toml")
                 .map(|mut f| f.read_to_string(&mut content))
                 .expect("could not read boards.toml");
    let cfg: Config = toml::from_str(&content).unwrap();

    // read the board argument and get the appropriate SoC instance
    
    let board_name = &env_args[2];
    let board = cfg.board.get(board_name).expect(&format!("board \"{}\" is not defined", board_name));
    let soc = cfg.soc.get(&board.soc).expect(&format!("soc \"{}\" is not defined", &board.soc));
    let board_feature = format!("board_{}", board_name);

    let mut cmd = Command::new("xargo");

    // pass through our arguments
    let mut args = vec![];

    // set our arguments
    let features = [ 
        format!("soc_{}", &board.soc)
    ];
    args.extend_from_slice(&[
        // pass through the cargo command first
        env_args[3].as_str(),
        "--target", &soc.arch,
        "--features", &board_feature,
    ]);
    args.extend(env_args.iter().skip(4).map(|x| x.as_str()));

    // pass the linker flags through the environment
    cmd.env("LD_FLASH_SIZE", &soc.flash);
    cmd.env("LD_RAM_SIZE", &soc.ram);

    // make sure to use a board-specific target directory
    // TODO: not quite optimal since we duplicate shared crates
    cmd.env("CARGO_TARGET_DIR", format!("target/{}", board_name));

    // we do not store the features in RUSTFLAGS since we do not want to 
    // recompile e.g. core or other dependencies for each board
    // so that has to be handled at the build-script level
    // e.g.
    // ```
    // for feature in env::var("BOARD_FEATURES").unwrap().split(" ") {
    //     println!("cargo:rustc-cfg=feature=\"{}\"", feature);
    // }
    // ```
    cmd.env("BOARD_FEATURES", features[1..].join(" "));

    cmd.args(&args);

    print!("Running `xargo");
    for arg in args {
        print!(" {}", arg);
    }
    println!("`:");
    let mut child = cmd.spawn().expect("could not spawn subprocess");
    let _ = child.wait().expect("subprocess returned an error");
}
