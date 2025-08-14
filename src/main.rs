mod commands;
mod files;
mod json;
mod parser;
mod search;
mod utils;

use clap::Parser;
use parser::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::AddToMany {
            key,
            from,
            where_,
            files,
        } => commands::add_to_many_command(key, from, where_, files),

        Commands::AddToSingle { from, where_ } => commands::add_to_single_command(from, where_),

        Commands::Remove { key, where_ } => commands::remove_command(key, where_),

        Commands::Replace {
            key,
            from,
            where_,
            files,
        } => commands::replace_command(key, from, where_, files),

        Commands::Rename { from, to, where_ } => commands::rename_command(from, to, where_),

        Commands::Sort { where_ } => commands::sort_command(where_),

        Commands::Compare { reference, target } => commands::compare_command(target, reference),

        Commands::CompareAll { where_ } => commands::compare_all_command(where_),

        Commands::ListUnusedKeys {
            translations,
            source,
        } => commands::list_unused_keys_command(translations, source),

        Commands::RemoveUnusedKeys {
            translations,
            source,
            where_,
        } => commands::remove_unused_keys_command(translations, source, where_),
    }
}
