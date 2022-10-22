mod patcher;

use clap::Parser;

use crate::patcher::Patcher;

#[derive(Parser)]
#[command(name = "oxips")]
#[command(author = "Orel Lazri")]
#[command(version = "0.1")]
#[command(about = "IPS patching tool", long_about = None)]
struct Args {
    #[arg(short, long)]
    rom: String,

    #[arg(short, long)]
    patch: String,

    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    let mut patcher = Patcher::default();
    patcher.patch(args.rom, args.patch, args.output);
}
