use clap::{Parser, Subcommand};

/// szambo - A command-line tool to manage JSON translation files
#[derive(Parser)]
#[command(name = "szambo")]
#[command(version = "1.0")]
#[command(about = "Translation manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum Commands {
    /// Add entries from input file to every file in directory by matching keys
    AddToMany {
        /// Path to the input JSON file (e.g., input.json)
        #[arg(long)]
        from: String,

        /// Key to add (e.g., foo.bar.baz)
        #[arg(long)]
        key: String,

        /// Target directory (e.g., lang/)
        #[arg(long)]
        where_: String,

        /// List of files for partial update (e.g., en.json,sv.json)
        #[arg(long)]
        files: Option<String>,
    },

    /// Add entries from input file to single target file
    AddToSingle {
        /// Path to the input JSON file (e.g., input.json)
        #[arg(long)]
        from: String,

        /// Target file (e.g., en.json)
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
        /// Key to replace (e.g., foo.bar.baz)
        #[arg(long)]
        key: String,

        /// Path to the input JSON file (e.g., input.json)
        #[arg(long)]
        from: String,

        /// Target directory (e.g., lang/)
        #[arg(long)]
        where_: String,

        /// List of files for partial update (e.g., en.json,sv.json)
        #[arg(long)]
        files: Option<String>,
    },

    /// Renames a key in every file in directory
    Rename {
        /// Key to rename (e.g., foo.bar)
        #[arg(long)]
        from: String,

        /// Key to rename (e.g., baz)
        #[arg(long)]
        to: String,

        /// Target directory (e.g., lang/)
        #[clap(long)]
        where_: String,
    },

    /// Sorts every JSON file in directory alphabetically (using keys)
    Sort {
        /// Target directory (e.g., lang/)
        #[clap(long)]
        where_: String,
    },

    /// Compares content of target file to reference file to list missing keys/paths
    Compare {
        /// File, for which missing keys/paths will be listed (e.g., sv.json)
        #[clap(long)]
        target: String,

        /// File, based on which missing keys/paths will be listed (e.g., en.json)
        #[clap(long)]
        reference: String,
    },

    /// Compares content of all files in directory to detect missing translations. Command returns status code
    CompareAll {
        /// Target directory (e.g., lang/)
        #[clap(long)]
        where_: String,
    },
}
