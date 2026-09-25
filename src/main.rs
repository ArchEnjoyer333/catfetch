use image::{imageops::FilterType, GenericImageView, Rgba};
use owo_colors::OwoColorize;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use sysinfo::System;

const ART_WIDTH: u32 = 34;
const ART_HEIGHT: u32 = 24;
const PADDING: usize = 3;
const TEXT_MAX_WIDTH: usize = 42;

#[derive(Debug, Deserialize)]
struct Database {
    facts: Vec<String>,
}

fn pixel_pair_to_ansi(top: Rgba<u8>, bottom: Rgba<u8>) -> String {
    let top_visible = top[3] > 64;
    let bottom_visible = bottom[3] > 64;

    match (top_visible, bottom_visible) {
        (true, true) => {
            format!(
                "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀",
                top[0], top[1], top[2], bottom[0], bottom[1], bottom[2]
            )
        }
        (true, false) => {
            format!("\x1b[0m\x1b[38;2;{};{};{}m▀", top[0], top[1], top[2])
        }
        (false, true) => {
            format!("\x1b[0m\x1b[38;2;{};{};{}m▄", bottom[0], bottom[1], bottom[2])
        }
        (false, false) => "\x1b[0m ".to_string(),
    }
}

fn render_png_to_halfblocks(path: &Path) -> Option<Vec<String>> {
    let img = image::open(path).ok()?;
    let resized = img.resize_exact(ART_WIDTH, ART_HEIGHT, FilterType::Nearest);
    let mut lines = Vec::new();

    for y in (0..ART_HEIGHT).step_by(2) {
        let mut line = String::new();
        for x in 0..ART_WIDTH {
            let top = resized.get_pixel(x, y);
            let bottom = if y + 1 < ART_HEIGHT {
                resized.get_pixel(x, y + 1)
            } else {
                Rgba([0, 0, 0, 0])
            };
            line.push_str(&pixel_pair_to_ansi(top, bottom));
        }
        line.push_str("\x1b[0m");
        lines.push(line);
    }

    Some(lines)
}

fn resolve_paths() -> (PathBuf, PathBuf) {
    // 1. Проверяем локальный конфиг пользователя (~/.config/catfetch)
    if let Some(config_dir) = dirs::config_dir().map(|d| d.join("catfetch")) {
        let logo = config_dir.join("logo");
        let toml = config_dir.join("cats.toml");
        if logo.exists() && toml.exists() {
            return (logo, toml);
        }
    }

    // 2. Системный путь продакшена (куда копирует pacman при установке пакета)
    let system_dir = PathBuf::from("/usr/share/catfetch");
    let system_logo = system_dir.join("logo");
    let system_toml = system_dir.join("cats.toml");

    if system_logo.exists() && system_toml.exists() {
        return (system_logo, system_toml);
    }

    // 3. Резервный путь для разработки (корень текущего проекта)
    let base_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    (base_dir.join("logo"), base_dir.join("cats.toml"))
}


fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.len() + 1 + word.len() <= max_width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines
}

fn render(art_lines: Vec<String>, fact: String) {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut info_column = Vec::new();

    let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    let host = System::host_name().unwrap_or_else(|| "localhost".to_string());
    let header = format!("{}@{}", user, host);
    let len = header.len();
    info_column.push(header.yellow().bold().to_string());
    info_column.push("-".repeat(len).dimmed().to_string());

    let os_name = System::name().unwrap_or_else(|| "Linux".to_string());
    let uptime_hours = System::uptime() / 3600;
    let total_ram = sys.total_memory() / 1024 / 1024;
    let used_ram = sys.used_memory() / 1024 / 1024;
    let cpu_brand = sys.cpus().first().map(|c| c.brand()).unwrap_or("CPU");

    info_column.push(format!("{}: {}", "OS".cyan().bold(), os_name));
    info_column.push(format!("{}: {}", "CPU".cyan().bold(), cpu_brand.trim()));
    info_column.push(format!("{}: {} / {} MB", "RAM".cyan().bold(), used_ram, total_ram));
    info_column.push(format!("{}: {} hours", "Uptime".cyan().bold(), uptime_hours));
    info_column.push(String::new());

    info_column.push("Cat Fact".yellow().dimmed().to_string());
    let wrapped_fact = wrap_text(&fact, TEXT_MAX_WIDTH);
    for line in wrapped_fact {
        info_column.push(line.italic().to_string());
    }
    info_column.push(String::new());

    let palette = (1..=6)
        .map(|c| format!("\x1b[4{}m   \x1b[0m", c))
        .collect::<Vec<String>>()
        .join(" ");
    info_column.push(palette);

    let total_rows = std::cmp::max(art_lines.len(), info_column.len());
    let blank_art = " ".repeat(ART_WIDTH as usize);

    println!();
    for i in 0..total_rows {
        let art = art_lines.get(i).map(|s| s.as_str()).unwrap_or(&blank_art);
        let info = info_column.get(i).map(|s| s.as_str()).unwrap_or("");
        let pad = " ".repeat(PADDING);

        println!(" {}{}{}", art, pad, info);
    }
    println!();
}

fn main() {
    let (logo_dir, toml_path) = resolve_paths();

    let toml_content = match fs::read_to_string(&toml_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("catfetch: cannot read {}", toml_path.display());
            return;
        }
    };

    let db: Database = match toml::from_str(&toml_content) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("catfetch: failed to parse TOML: {}", e);
            return;
        }
    };

    if db.facts.is_empty() {
        eprintln!("catfetch: no facts found in cats.toml");
        return;
    }

    let mut entries = Vec::new();
    if let Ok(mut read_dir) = fs::read_dir(&logo_dir) {
        while let Some(Ok(entry)) = read_dir.next() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("png") {
                entries.push(path);
            }
        }
    }

    if entries.is_empty() {
        eprintln!("catfetch: no .png images found in logo/");
        return;
    }

    let chosen_img = entries.choose(&mut rand::thread_rng()).unwrap();
    let chosen_fact = db.facts.choose(&mut rand::thread_rng()).unwrap().clone();

    let art_lines = match render_png_to_halfblocks(chosen_img) {
        Some(lines) => lines,
        None => {
            eprintln!("catfetch: failed to render {}", chosen_img.display());
            return;
        }
    };

    render(art_lines, chosen_fact);
}
