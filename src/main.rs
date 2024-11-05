use clap::{Args, Parser, Subcommand};
use statrs::distribution::{ContinuousCDF, Normal};

mod life_expectancy;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// `std <-> percent` converter
	Std(StdArgs),
	/// Reimann Zeta calculator. Calculates expected value of 1 out of provided `n`.
	#[command(alias = "rz")]
	ReimannZeta(ReimannZetaArgs),
	/// Chance of dying next year, based on birth year and gender (Currently only France).
	#[command(alias = "dny")]
	DieNextYearFrance(life_expectancy::LifeExpectancyArgs),
	/// Approximate number of days you have left to live, following France's statistic
	DaysLeft(life_expectancy::LifeExpectancyArgs),
}

#[derive(Args)]
struct StdArgs {
	/// Value of the range.
	/// if v < 20 : std_to_percent() : v < 100 ? percent_to_std() : takes n for frequency_of_occurence := 1/n, then returns its std.
	#[arg(value_parser)]
	to_convert: f64,
}

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
		Commands::DieNextYearFrance(args) => life_expectancy::die_next_year_france(args).expect("TODO: error type for age out of reasonable range").to_string(),
		Commands::DaysLeft(args) => life_expectancy::days_left(args).expect("TODO: error type for age out of reasonable range").to_string(),
	};

	println!("{}", output);
}

fn std(args: StdArgs) -> String {
	let n = Normal::new(0.0, 1.0).unwrap();

	if args.to_convert < 20.0 {
		let cp = n.cdf(args.to_convert) - n.cdf(-args.to_convert);
		format!("{}%\n1/{}", cp * 100.0, (2.0 / (1.0 - cp)).round()) // 0.5 because here we normally want to know what it takes to exit the curve from the upper bound.
	} else if args.to_convert < 100.0 {
		let z = n.inverse_cdf(1.0 - (100.0 - args.to_convert) / 200.0);
		format!("{}", (z * 10.0).round() / 10.0)
	} else {
		let z = n.inverse_cdf(1.0 - 1.0 / args.to_convert);
		format!("{}", (z * 10.0).round() / 10.0)
	}
}

fn reimann_zeta(args: ReimannZetaArgs) -> String {
	let mut sum = 0.0;
	for i in 1..=args.n {
		sum += 1.0 / i as f64;
	}
	let value = (1.0 / args.positions as f64) / sum;
	format!("{}%", value * 100.0)
}
