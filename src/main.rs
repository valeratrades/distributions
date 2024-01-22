use anyhow::{anyhow, Result};
use chrono::prelude::*;
use clap::{Args, Parser, Subcommand};
use statrs::distribution::{ContinuousCDF, Normal};
use std::str::FromStr;

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
	#[command(alias = "dny")]
	DieNextYearFrance(DieNextYearFranceArgs),
}

#[derive(Args)]
struct StdArgs {
	/// Value of the range.
	/// if v < 20 : std_to_percent() : percent_to_std()
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

#[derive(Args)]
struct DieNextYearFranceArgs {
	/// Year of birth.
	#[arg(value_parser)]
	year: usize,
	/// "male" or "female"
	#[arg(short, long, default_value = "male")]
	gender: Gender,
}

#[derive(Debug, Clone, PartialEq)]
enum Gender {
	Male,
	Female,
}
impl FromStr for Gender {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"male" => Ok(Gender::Male),
			"female" => Ok(Gender::Female),
			_ => Err(anyhow!("Invalid gender")),
		}
	}
}

fn main() {
	let cli = Cli::parse();

	let output = match cli.command {
		Commands::Std(args) => std(args),
		Commands::ReimannZeta(args) => reimann_zeta(args),
		Commands::DieNextYearFrance(args) => die_next_year_france(args),
	};

	println!("{}", output);
}

fn std(args: StdArgs) -> String {
	let n = Normal::new(0.0, 1.0).unwrap();

	if args.to_convert < 20.0 {
		let cp = n.cdf(args.to_convert) - n.cdf(-args.to_convert);
		format!("{}%", cp * 100.0)
	} else {
		let z = n.inverse_cdf(1.0 - (100.0 - args.to_convert) / 200.0);
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

fn die_next_year_france(args: DieNextYearFranceArgs) -> String {
	let birth_date = NaiveDate::from_ymd_opt(args.year as i32, 6, 15).unwrap();
	let now = Utc::now();
	let age = now.year() - birth_date.year() - if now.ordinal() < birth_date.ordinal() { 1 } else { 0 };

	let (male_rate, female_rate) = match age {
		0 => (3.3, 2.8),
		1..=4 => (0.2, 0.2),
		5..=9 => (0.1, 0.1),
		10..=14 => (0.1, 0.1),
		15..=19 => (0.3, 0.1),
		20..=24 => (0.5, 0.2),
		25..=29 => (0.7, 0.2),
		30..=34 => (0.8, 0.3),
		35..=39 => (1.1, 0.5),
		40..=44 => (1.6, 0.8),
		45..=49 => (2.7, 1.4),
		50..=54 => (4.2, 2.3),
		55..=59 => (6.7, 3.4),
		60..=64 => (10.7, 5.0),
		65..=69 => (15.6, 7.2),
		70..=79 => (26.9, 13.6),
		80..=89 => (79.6, 51.7),
		90..=110 => (235.6, 185.9),
		_ => (0.0, 0.0),
	};

	let chance = match args.gender {
		Gender::Male => male_rate as f64 / 1000.0,
		Gender::Female => female_rate as f64 / 1000.0,
	};
	format!("{}%", chance * 100.0)
}
