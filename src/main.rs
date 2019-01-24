extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("aeron-dpdk-driver")
        .version("0.1.0")
        .author("Kean H Lau <khklau@users.noreply.github.com>")
        .about("DPDK Based Aeron Media Driver")
        .arg(
            Arg::with_name("conductor-idle")
                .help("Sets the idle strategy for the conductor")
                .long("conductor-idle")
                .takes_value(true)
                .value_name("STRATEGY")
                .default_value("spin"),
        )
        .arg(
            Arg::with_name("receiver-idle")
                .help("Sets the idle strategy for the receiver")
                .long("receiver-idle")
                .takes_value(true)
                .value_name("STRATEGY")
                .default_value("spin"),
        )
        .arg(
            Arg::with_name("sender-idle")
                .help("Sets the idle strategy for the sender")
                .long("sender-idle")
                .takes_value(true)
                .value_name("STRATEGY")
                .default_value("spin"),
        )
        .arg(
            Arg::with_name("verbosity")
                .help("Sets the level of verbosity")
                .short("v")
                .multiple(true),
        )
        .get_matches();
}
