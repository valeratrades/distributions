use std::str::FromStr;

use chrono::prelude::*;
use clap::Args;
use color_eyre::eyre::{bail, Report, Result};
use v_utils::io::Percent;

#[derive(Args)]
pub struct LifeExpectancyArgs {
	/// Year of birth.
	#[arg(value_parser)]
	year: usize,
	/// "male" or "female"
	#[arg(short, long, default_value = "male")]
	gender: Gender,
}

impl LifeExpectancyArgs {
	pub fn age_now(&self) -> Result<u8> {
		let birth_date = NaiveDate::from_ymd_opt(self.year as i32, 6, 15).unwrap();
		let now = Utc::now();
		let age = now.year() - birth_date.year() - if now.ordinal() < birth_date.ordinal() { 1 } else { 0 };
		match TryInto::<u8>::try_into(age) {
			Ok(a) => Ok(a),
			Err(e) => bail!(e),
		}
	}

	pub fn survival_rate_fr(&self) -> f64 {
		let age = self.age_now().unwrap(); //HACK
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
		match self.gender {
			Gender::Male => male_rate,
			Gender::Female => female_rate,
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
enum Gender {
	Male,
	Female,
}
impl FromStr for Gender {
	type Err = Report;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"male" => Ok(Gender::Male),
			"female" => Ok(Gender::Female),
			_ => bail!("Invalid gender"),
		}
	}
}
pub fn die_next_year_france(args: LifeExpectancyArgs) -> Result<Percent> {
	let rate = args.survival_rate_fr();
	let percent: Percent = (rate / 1000.0).into();
	Ok(percent)
}

pub fn expected_age_of_death(args: LifeExpectancyArgs) -> Result<f64> {
	let current_age = args.age_now()?;
	let mut age = current_age as f64;
	let mut total_weight = 0.0;
	let mut total_age = 0.0;

	while age < 110.0 {
		let prob_survival = args.survival_rate_fr();
		let prob_death = 1.0 - prob_survival;

		total_weight += prob_death; // Weight for integration
		total_age += age * prob_death; // Sum of weighted ages

		age += 1.0;
	}

	if total_weight > 0.0 {
		Ok(total_age / total_weight)
	} else {
		bail!("Failed to calculate expected age of death.")
	}
}

pub fn days_left(args: LifeExpectancyArgs) -> Result<u32> {
	let current_age = args.age_now()? as f64;
	let expected_age_of_death = expected_age_of_death(args)?; // Use the previous function

	if expected_age_of_death <= current_age {
		bail!("you are too old");
	} else {
		let remaining_years = expected_age_of_death - current_age;
		let remaining_days = (remaining_years * 365.25) as u32;
		Ok(remaining_days)
	}
}
