use clap::Parser;
use colored::Colorize;
use ignore::WalkBuilder;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = "loc")]
#[command(about = "Count lines of code by language")]
struct Args {
    /// Directories to scan (defaults to current directory)
    #[arg(default_value = ".")]
    paths: Vec<PathBuf>,

    /// Show lines of code history over time (from git)
    #[arg(long, short = 'H')]
    history: bool,

    /// Number of commits to sample for history (default: 10)
    #[arg(long, default_value = "10")]
    samples: usize,
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

fn get_color(lang: &str) -> (u8, u8, u8) {
    match lang {
        "Rust" => (222, 165, 132),
        "Python" => (55, 118, 171),
        "JavaScript" => (247, 223, 30),
        "TypeScript" => (49, 120, 198),
        "Go" => (0, 173, 216),
        "Ruby" => (204, 52, 45),
        "Java" => (176, 114, 25),
        "C" => (85, 85, 85),
        "C++" => (243, 75, 125),
        "C/C++ Header" => (168, 132, 189),
        "C#" => (104, 33, 122),
        "PHP" => (119, 123, 180),
        "Swift" => (255, 172, 69),
        "Kotlin" => (169, 123, 255),
        "Scala" => (194, 45, 64),
        "HTML" => (227, 76, 38),
        "CSS" => (86, 61, 124),
        "SCSS" => (198, 83, 140),
        "JSON" => (41, 41, 41),
        "YAML" => (203, 23, 30),
        "TOML" => (156, 66, 33),
        "Markdown" => (8, 63, 161),
        "Shell" => (137, 224, 81),
        "SQL" => (224, 142, 42),
        "Lua" => (0, 0, 128),
        "R" => (25, 140, 231),
        "Dart" => (0, 180, 171),
        "Zig" => (236, 145, 92),
        "Nim" => (255, 233, 83),
        "Elixir" => (110, 74, 126),
        "Erlang" => (161, 0, 52),
        "Haskell" => (94, 80, 134),
        "OCaml" => (238, 122, 0),
        "Vue" => (65, 184, 131),
        "Svelte" => (255, 62, 0),
        _ => (255, 255, 255),
    }
}

fn colorize(lang: &str, text: &str) -> String {
    let (r, g, b) = get_color(lang);
    text.truecolor(r, g, b).to_string()
}

fn count_lines(path: &PathBuf) -> usize {
    match fs::read_to_string(path) {
        Ok(content) => content.lines().count(),
        Err(_) => 0,
    }
}

fn count_lines_str(content: &str) -> usize {
    content.lines().count()
}

fn scan_directory(path: &Path) -> HashMap<&'static str, usize> {
    let mut stats: HashMap<&'static str, usize> = HashMap::new();

    let builder = WalkBuilder::new(path);
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

    stats
}

fn print_stats(stats: &HashMap<&'static str, usize>) {
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

        let padded_lang = format!("{:>width$}", lang, width = max_lang_width);
        let padded_count = format!("{:>width$}", count, width = max_count_width);

        println!(
            "{}  {}  {:>5.1}%  {}",
            colorize(lang, &padded_lang),
            padded_count,
            percentage,
            colorize(lang, &bar),
        );
    }
    println!();
    let padded_total_label = format!("{:>width$}", "Total", width = max_lang_width);
    let padded_total_count = format!("{:>width$}", total, width = max_count_width);
    println!(
        "{}  {}",
        padded_total_label.bold(),
        padded_total_count.bold(),
    );
}

struct CommitInfo {
    hash: String,
    date: String,
}

fn get_commits(repo_path: &Path, samples: usize) -> Vec<CommitInfo> {
    let output = Command::new("git")
        .args(["log", "--format=%H %as", "--reverse"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to run git log");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let commits: Vec<CommitInfo> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() == 2 {
                Some(CommitInfo {
                    hash: parts[0].to_string(),
                    date: parts[1].to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    if commits.len() <= samples {
        return commits;
    }

    // Sample evenly across history
    let step = commits.len() as f64 / samples as f64;
    let mut sampled = Vec::new();
    for i in 0..samples {
        let idx = (i as f64 * step) as usize;
        if idx < commits.len() {
            sampled.push(CommitInfo {
                hash: commits[idx].hash.clone(),
                date: commits[idx].date.clone(),
            });
        }
    }

    // Always include the latest commit
    if let Some(last) = commits.last() {
        if sampled.last().map(|c| &c.hash) != Some(&last.hash) {
            sampled.push(CommitInfo {
                hash: last.hash.clone(),
                date: last.date.clone(),
            });
        }
    }

    sampled
}

fn count_lines_at_commit(repo_path: &Path, commit: &str) -> usize {
    // Get list of files at this commit
    let output = Command::new("git")
        .args(["ls-tree", "-r", "--name-only", commit])
        .current_dir(repo_path)
        .output()
        .expect("Failed to run git ls-tree");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut total = 0;

    for file in stdout.lines() {
        let ext = Path::new(file)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        if get_language(ext).is_some() {
            // Get file content at this commit
            let content_output = Command::new("git")
                .args(["show", &format!("{}:{}", commit, file)])
                .current_dir(repo_path)
                .output();

            if let Ok(output) = content_output {
                if output.status.success() {
                    let content = String::from_utf8_lossy(&output.stdout);
                    total += count_lines_str(&content);
                }
            }
        }
    }

    total
}

fn show_history(repo_path: &Path, samples: usize) {
    let commits = get_commits(repo_path, samples);

    if commits.is_empty() {
        println!("No git history found.");
        return;
    }

    println!("\nAnalyzing {} commits...\n", commits.len());

    let mut history: Vec<(String, usize)> = Vec::new();
    let mut max_lines: usize = 0;

    for commit in &commits {
        let lines = count_lines_at_commit(repo_path, &commit.hash);
        max_lines = max_lines.max(lines);
        history.push((commit.date.clone(), lines));
        eprint!(".");
    }
    eprintln!();

    // Find max for scaling
    let max_count_width = history
        .iter()
        .map(|(_, count)| count.to_string().len())
        .max()
        .unwrap_or(0);

    let bar_max_width = 40;

    println!();
    for (date, lines) in &history {
        let bar_width = if max_lines > 0 {
            (*lines as f64 / max_lines as f64 * bar_max_width as f64) as usize
        } else {
            0
        };
        let bar = "█".repeat(bar_width);

        println!(
            "{}  {:>width$}  {}",
            date.bright_black(),
            lines,
            bar.green(),
            width = max_count_width,
        );
    }
    println!();
}

fn main() {
    let args = Args::parse();

    if args.history {
        let repo_path = &args.paths[0];
        show_history(repo_path, args.samples);
        return;
    }

    let mut stats: HashMap<&'static str, usize> = HashMap::new();

    for path in &args.paths {
        let path_stats = scan_directory(path);
        for (lang, count) in path_stats {
            *stats.entry(lang).or_insert(0) += count;
        }
    }

    print_stats(&stats);
}
