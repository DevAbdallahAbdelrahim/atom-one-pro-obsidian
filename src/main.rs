use grass::{from_path, Options};
use notify::{RecursiveMode, Result as NotifyResult, Watcher};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn resolve_path(raw_path: &str) -> PathBuf {
    let expanded = if raw_path.starts_with('~') {
        if let Some(home) = env::var_os("HOME").or_else(|| env::var_os("USERPROFILE")) {
            let mut p = PathBuf::from(home);
            p.push(raw_path.trim_start_matches("~/").trim_start_matches("~\\"));
            p
        } else {
            PathBuf::from(raw_path)
        }
    } else {
        PathBuf::from(raw_path)
    };

    if expanded.is_absolute() {
        expanded
    } else {
        env::current_dir().unwrap_or_default().join(expanded)
    }
}

fn build_theme() {
    println!("⚡ [Rust Builder] Compiling SCSS to CSS...");

    let entry_file = Path::new("src").join("scss").join("main.scss");
    let options = Options::default();

    match from_path(&entry_file, &options) {
        Ok(css_content) => {
            let header = "/* ==========================================================================\n   Atom One Pro Theme for Obsidian v1.0.0\n   Built with Native Rust Engine (Cross-Platform & Zero Node.js)\n   ========================================================================== */\n\n";
            let final_css = format!("{}{}", header, css_content);

            if let Err(e) = fs::write("theme.css", &final_css) {
                eprintln!("❌ Failed to write local theme.css: {}", e);
                return;
            }
            println!("✅ Generated theme.css successfully!");

            if let Ok(vault_path_str) = env::var("OBSIDIAN_PATH") {
                let target_dir = resolve_path(&vault_path_str);
                if let Err(e) = fs::create_dir_all(&target_dir) {
                    eprintln!("❌ Could not create target directory: {}", e);
                    return;
                }

                let target_css = target_dir.join("theme.css");
                let target_manifest = target_dir.join("manifest.json");

                let _ = fs::write(target_css, &final_css);
                let _ = fs::copy("manifest.json", target_manifest);

                println!("🚀 Synced with Obsidian Vault at: {:?}", target_dir);
            }
        }
        Err(err) => eprintln!("❌ SCSS Compilation Error: {}", err),
    }
}

fn main() -> NotifyResult<()> {
    dotenvy::dotenv().ok();

    build_theme();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--watch" {
        println!("👀 Listening for SCSS changes across OS native events...");

        let mut watcher = notify::recommended_watcher(|res| match res {
            Ok(_) => build_theme(),
            Err(e) => eprintln!("Watch error: {:?}", e),
        })?;

        let watch_path = Path::new("src").join("scss");
        watcher.watch(&watch_path, RecursiveMode::Recursive)?;

        std::thread::park();
    }

    Ok(())
}
