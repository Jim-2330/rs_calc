use crate::cli::error::{Mode, run_once};
use crate::cli::repl::run_repl;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "a tiny calculator")]
struct Args {
    expr: Option<String>,
    #[arg(short, long, value_enum, default_value_t = Mode::Eval)]
    mode: Mode,
}

pub fn entry() {
    let args = Args::parse(); // clap 内部读 env::args
    if let Some(expr) = args.expr {
        let res = run_once(expr.clone().as_str(), args.mode);
        if let Err(e) = res {
            eprintln!("error: {e:?}");
            std::process::exit(1);
        }
    } else {
        run_repl(args.mode);
    }
}
