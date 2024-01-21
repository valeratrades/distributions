use clap::{Args, Parser, Subcommand};
use statrs::distribution::{ContinuousCDF, Normal};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	Std(StdArgs),
	#[command(alias = "rz")]
	ReimannZeta(ReimannZetaArgs),
}

/// `std <-> percent` converter
#[derive(Args)]
struct StdArgs {
	/// Value of the range.
	/// if v < 20 : std_to_percent() : percent_to_std()
	#[arg(value_parser)]
	to_convert: f64,
}

/// Reimann Zeta calculator. Calculates expected value of 1 out of provided `n`.
#[derive(Args)]
struct ReimannZetaArgs {
	/// n of the range.
	#[arg(value_parser)]
	n: usize,
	#[arg(short, long, default_value = "1")]
	positions: usize,
}

fn main() {
	let cli = Cli::parse();

	let output = match cli.command {
		Commands::Std(args) => std(args),
		Commands::ReimannZeta(args) => reimann_zeta(args),
	};

	println!("{}", output);
}

fn std(args: StdArgs) -> f64 {
	let n = Normal::new(0.0, 1.0).unwrap();
	let converted: f64 = {
		if args.to_convert < 20.0 {
			let cp = n.cdf(args.to_convert) - n.cdf(-args.to_convert);
			cp * 100.0
		} else {
			let z = n.inverse_cdf(1.0 - (100.0 - args.to_convert) / 200.0);
			(z * 10.0).round() / 10.0
		}
	};
	converted
}

fn reimann_zeta(args: ReimannZetaArgs) -> f64 {
	let mut sum = 0.0;
	for i in 1..=args.n {
		sum += 1.0 / i as f64;
	}
	let value = (1.0 / args.positions as f64) / sum;
	value
}
