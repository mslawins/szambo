use grep::regex::RegexMatcher;
use grep::searcher::{Searcher, sinks::UTF8};
use ignore::WalkBuilder;
use regex;

pub fn find_unused_paths(patterns: Vec<String>, dir: String) -> Result<Vec<String>, String> {
    let mut unused = Vec::new();

    for pattern in patterns {
        let matcher = RegexMatcher::new(&regex::escape(&pattern))
            .map_err(|e| format!("Failed to create regex matcher: {}", e))?;
        let mut found = false;

        let walker = WalkBuilder::new(&dir)
            .hidden(false)
            .ignore(true)
            .git_ignore(true)
            .git_exclude(true)
            .build();

        for result in walker {
            let dent = match result {
                Ok(d) => d,
                Err(_) => continue,
            };

            if dent.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                let path = dent.path();

                let search_result = Searcher::new().search_path(
                    &matcher,
                    path,
                    UTF8(|_, _| {
                        found = true;
                        Ok(true)
                    }),
                );

                if search_result.is_err() {
                    continue;
                }

                if found {
                    break;
                }
            }
        }

        if !found {
            unused.push(pattern);
        }
    }

    Ok(unused)
}
