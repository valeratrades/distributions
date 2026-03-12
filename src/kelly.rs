use clap::Args;
use v_utils::Fraction;

#[derive(Args)]
pub struct KellyArgs {
	/// Certainty of the outcome (0.0 to 1.0)
	#[arg(short, long)]
	certainty: f64,
	/// Risk-reward ratio as a fraction (e.g. 3/1 means you win 3 for every 1 risked)
	#[arg(short, long)]
	risk_reward: Fraction,
}

/// Kelly criterion: f* = p - (1-p)/b
pub fn kelly(args: KellyArgs) -> String {
	let p = args.certainty;
	let b = *args.risk_reward;

	let f = p - (1.0 - p) / b;
	format!("{:.1}%", f * 100.0)
}
