use clap::Parser;
use colored::Colorize;
use ignore::WalkBuilder;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "loc")]
#[command(about = "Count lines of code by language")]
struct Args {
    /// Directories to scan (defaults to current directory)
    #[arg(default_value = ".")]
    paths: Vec<PathBuf>,
}

fn get_language(extension: &str) -> Option<&'static str> {
    match extension {
        "rs" => Some("Rust"),
        "py" => Some("Python"),
        "js" => Some("JavaScript"),
        "ts" => Some("TypeScript"),
        "tsx" => Some("TypeScript"),
        "jsx" => Some("JavaScript"),
        "go" => Some("Go"),
        "rb" => Some("Ruby"),
        "java" => Some("Java"),
        "c" => Some("C"),
        "cpp" | "cc" | "cxx" => Some("C++"),
        "h" | "hpp" => Some("C/C++ Header"),
        "cs" => Some("C#"),
        "php" => Some("PHP"),
        "swift" => Some("Swift"),
        "kt" | "kts" => Some("Kotlin"),
        "scala" => Some("Scala"),
        "html" | "htm" => Some("HTML"),
        "css" => Some("CSS"),
        "scss" | "sass" => Some("SCSS"),
        "json" => Some("JSON"),
        "yaml" | "yml" => Some("YAML"),
        "toml" => Some("TOML"),
        "md" => Some("Markdown"),
        "sh" | "bash" => Some("Shell"),
        "sql" => Some("SQL"),
        "lua" => Some("Lua"),
        "r" => Some("R"),
        "dart" => Some("Dart"),
        "zig" => Some("Zig"),
        "nim" => Some("Nim"),
        "ex" | "exs" => Some("Elixir"),
        "erl" => Some("Erlang"),
        "hs" => Some("Haskell"),
        "ml" | "mli" => Some("OCaml"),
        "vue" => Some("Vue"),
        "svelte" => Some("Svelte"),
        _ => None,
    }
}

fn colorize_language(lang: &str) -> String {
    match lang {
        "Rust" => lang.truecolor(222, 165, 132).to_string(),
        "Python" => lang.truecolor(55, 118, 171).to_string(),
        "JavaScript" => lang.truecolor(247, 223, 30).to_string(),
        "TypeScript" => lang.truecolor(49, 120, 198).to_string(),
        "Go" => lang.truecolor(0, 173, 216).to_string(),
        "Ruby" => lang.truecolor(204, 52, 45).to_string(),
        "Java" => lang.truecolor(176, 114, 25).to_string(),
        "C" => lang.truecolor(85, 85, 85).to_string(),
        "C++" => lang.truecolor(243, 75, 125).to_string(),
        "C/C++ Header" => lang.truecolor(168, 132, 189).to_string(),
        "C#" => lang.truecolor(104, 33, 122).to_string(),
        "PHP" => lang.truecolor(119, 123, 180).to_string(),
        "Swift" => lang.truecolor(255, 172, 69).to_string(),
        "Kotlin" => lang.truecolor(169, 123, 255).to_string(),
        "Scala" => lang.truecolor(194, 45, 64).to_string(),
        "HTML" => lang.truecolor(227, 76, 38).to_string(),
        "CSS" => lang.truecolor(86, 61, 124).to_string(),
        "SCSS" => lang.truecolor(198, 83, 140).to_string(),
        "JSON" => lang.truecolor(41, 41, 41).to_string(),
        "YAML" => lang.truecolor(203, 23, 30).to_string(),
        "TOML" => lang.truecolor(156, 66, 33).to_string(),
        "Markdown" => lang.truecolor(8, 63, 161).to_string(),
        "Shell" => lang.truecolor(137, 224, 81).to_string(),
        "SQL" => lang.truecolor(224, 142, 42).to_string(),
        "Lua" => lang.truecolor(0, 0, 128).to_string(),
        "R" => lang.truecolor(25, 140, 231).to_string(),
        "Dart" => lang.truecolor(0, 180, 171).to_string(),
        "Zig" => lang.truecolor(236, 145, 92).to_string(),
        "Nim" => lang.truecolor(255, 233, 83).to_string(),
        "Elixir" => lang.truecolor(110, 74, 126).to_string(),
        "Erlang" => lang.truecolor(161, 0, 52).to_string(),
        "Haskell" => lang.truecolor(94, 80, 134).to_string(),
        "OCaml" => lang.truecolor(238, 122, 0).to_string(),
        "Vue" => lang.truecolor(65, 184, 131).to_string(),
        "Svelte" => lang.truecolor(255, 62, 0).to_string(),
        _ => lang.white().to_string(),
    }
}

fn colorize_bar(lang: &str, bar: &str) -> String {
    match lang {
        "Rust" => bar.truecolor(222, 165, 132).to_string(),
        "Python" => bar.truecolor(55, 118, 171).to_string(),
        "JavaScript" => bar.truecolor(247, 223, 30).to_string(),
        "TypeScript" => bar.truecolor(49, 120, 198).to_string(),
        "Go" => bar.truecolor(0, 173, 216).to_string(),
        "Ruby" => bar.truecolor(204, 52, 45).to_string(),
        "Java" => bar.truecolor(176, 114, 25).to_string(),
        "C" => bar.truecolor(85, 85, 85).to_string(),
        "C++" => bar.truecolor(243, 75, 125).to_string(),
        "C/C++ Header" => bar.truecolor(168, 132, 189).to_string(),
        "C#" => bar.truecolor(104, 33, 122).to_string(),
        "PHP" => bar.truecolor(119, 123, 180).to_string(),
        "Swift" => bar.truecolor(255, 172, 69).to_string(),
        "Kotlin" => bar.truecolor(169, 123, 255).to_string(),
        "Scala" => bar.truecolor(194, 45, 64).to_string(),
        "HTML" => bar.truecolor(227, 76, 38).to_string(),
        "CSS" => bar.truecolor(86, 61, 124).to_string(),
        "SCSS" => bar.truecolor(198, 83, 140).to_string(),
        "JSON" => bar.truecolor(41, 41, 41).to_string(),
        "YAML" => bar.truecolor(203, 23, 30).to_string(),
        "TOML" => bar.truecolor(156, 66, 33).to_string(),
        "Markdown" => bar.truecolor(8, 63, 161).to_string(),
        "Shell" => bar.truecolor(137, 224, 81).to_string(),
        "SQL" => bar.truecolor(224, 142, 42).to_string(),
        "Lua" => bar.truecolor(0, 0, 128).to_string(),
        "R" => bar.truecolor(25, 140, 231).to_string(),
        "Dart" => bar.truecolor(0, 180, 171).to_string(),
        "Zig" => bar.truecolor(236, 145, 92).to_string(),
        "Nim" => bar.truecolor(255, 233, 83).to_string(),
        "Elixir" => bar.truecolor(110, 74, 126).to_string(),
        "Erlang" => bar.truecolor(161, 0, 52).to_string(),
        "Haskell" => bar.truecolor(94, 80, 134).to_string(),
        "OCaml" => bar.truecolor(238, 122, 0).to_string(),
        "Vue" => bar.truecolor(65, 184, 131).to_string(),
        "Svelte" => bar.truecolor(255, 62, 0).to_string(),
        _ => bar.white().to_string(),
    }
}

fn count_lines(path: &PathBuf) -> usize {
    match fs::read_to_string(path) {
        Ok(content) => content.lines().count(),
        Err(_) => 0,
    }
}

fn main() {
    let args = Args::parse();
    let mut stats: HashMap<&'static str, usize> = HashMap::new();

    let mut builder = WalkBuilder::new(&args.paths[0]);
    for path in args.paths.iter().skip(1) {
        builder.add(path);
    }

    for entry in builder.build().filter_map(|e| e.ok()) {
        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            if let Some(ext) = entry.path().extension() {
                if let Some(ext_str) = ext.to_str() {
                    if let Some(lang) = get_language(ext_str) {
                        let lines = count_lines(&entry.path().to_path_buf());
                        *stats.entry(lang).or_insert(0) += lines;
                    }
                }
            }
        }
    }

    if stats.is_empty() {
        println!("No code files found.");
        return;
    }

    let mut sorted: Vec<_> = stats.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    let total: usize = sorted.iter().map(|(_, count)| *count).sum();
    let max_lang_width = sorted.iter().map(|(lang, _)| lang.len()).max().unwrap_or(0);
    let max_count_width = sorted
        .iter()
        .map(|(_, count)| count.to_string().len())
        .max()
        .unwrap_or(0);

    println!();
    for (lang, count) in &sorted {
        let percentage = (**count as f64 / total as f64) * 100.0;
        let bar_width = (percentage / 2.0) as usize;
        let bar = "█".repeat(bar_width);

        println!(
            "{:>width$}  {:>count_width$}  {:>5.1}%  {}",
            colorize_language(lang),
            count,
            percentage,
            colorize_bar(lang, &bar),
            width = max_lang_width,
            count_width = max_count_width,
        );
    }
    println!();
    println!(
        "{:>width$}  {:>count_width$}",
        "Total".bold(),
        total.to_string().bold(),
        width = max_lang_width,
        count_width = max_count_width,
    );
}
