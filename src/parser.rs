use clap::{Parser, Subcommand};

/// szambo - A command-line tool to manage translation files (or similar)
#[derive(Parser)]
#[command(name = "szambo")]
#[command(version = "1.0")]
#[command(about = "Translation manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add entries from a file to a directory
    Add {
        /// Path to the input file (JSON)
        #[arg(long)]
        from: String,

        /// Key to add (e.g., foo.bar.baz)
        #[arg(long)]
        key: String,

        /// Target directory (e.g., lang/)
        #[arg(long)]
        where_: String,
    },

    /// Remove entry by key from a directory
    Remove {
        /// Key to remove (e.g., foo.bar.baz)
        #[arg(long)]
        key: String,

        /// Target directory (e.g., lang/)
        #[arg(long)]
        where_: String,
    },

    /// Replace a key's value from input file in a directory
    Replace {
        /// Key to replace
        #[arg(long)]
        key: String,

        /// Path to the input file (JSON)
        #[arg(long)]
        from: String,

        /// Target directory (e.g., lang/)
        #[arg(long)]
        where_: String,
    },
}
