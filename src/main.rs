use std::fs;

mod category;
mod cli;

use category::categorise;
use clap::Parser;
use cli::Args;

fn main() {
    // Input Path
    let args = Args::parse();
    let paths = fs::read_dir(&args.path).unwrap();

    //Loop over directories
    for path in paths {
        let entry_path = path.unwrap().path();
        if !entry_path.is_file() {
            continue;
        }
        let ext = match entry_path.extension() {
            Some(e) => e.to_str().unwrap_or("invalid"),
            None => "unknown",
        };
        let _file_stem = match entry_path.file_stem() {
            Some(n) => n.to_str().unwrap_or("invalid"),
            None => "no file",
        };
        let file_name = match entry_path.file_name() {
            Some(f) => f.to_str().unwrap_or("invalid"),
            None => "no filename",
        };
        let ext_lower = ext.to_lowercase();
        let category = categorise(&ext_lower);
        //println!("{} -> .{} -> {:?}", file_stem, ext, category);

        // Set up destination directory and file
        let dest_dir = args.path.join(category.folder_name());
        let dest_file = dest_dir.join(file_name);

        //println!("{}", dest_file.display());
        let verb1 = if args.dry_run {
            "Would skip"
        } else {
            "Skipping"
        };
        if dest_file.exists() {
            println!("{verb1} {}: already exists", dest_file.display());
            continue;
        }
        let verb2 = if args.dry_run { "Would move" } else { "Moving" };
        println!(
            "{verb2}: {} -> {}",
            entry_path.display(),
            dest_file.display()
        );

        if !args.dry_run {
            fs::create_dir_all(&dest_dir).unwrap();
        }
        if !args.dry_run {
            fs::rename(&entry_path, &dest_file).unwrap();
        }
    }
}
