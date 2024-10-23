use clap::Parser;

/// A tool to keep a PC active when you can't modify its power settings
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Minutes until your PC sleeps
    #[arg(default_value_t = 5.0)]
    pub minutes: f64,

    /// Custom sleep time in minutes (0 = never)
    #[arg(short, long, default_value_t = 0.0)]
    pub r#override: f64,
}
