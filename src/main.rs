use std::env;

mod error;
mod globals;
mod utils;

#[cfg(test)]
mod test_utils;

fn main() -> anyhow::Result<()> {
  let args: Vec<String> = env::args().collect();
  let args: Vec<&str> = args.iter().map(String::as_str).collect();
  let parsed = utils::parse_args(&args);
  let rest: Vec<&str> = parsed.rest.iter().map(String::as_str).collect();
  let dir = utils::get_download_dir(&rest)?;
  let by_date = parsed.by_date;

  if parsed.dry_run {
    println!("Dry run — previewing how {dir} would be organized.");
    let s = utils::organize(&dir, true, by_date)?;
    print_summary(true, &s);
    return Ok(());
  }

  utils::set_write_permissions(&dir)?;
  let created = utils::create_dirs(&dir)?;
  if created != 0 {
    println!(
      "{} created",
      noun(created, "Category directory", "Category directories")
    );
  }
  println!("Organizing {dir}.");
  let s = utils::organize(&dir, false, by_date)?;
  print_summary(false, &s);

  Ok(())
}

fn print_summary(dry_run: bool, s: &utils::OrganizeSummary) {
  let files = noun(s.moved, "file", "files");
  let entries = noun(s.skipped, "entry", "entries");
  println!();
  if dry_run {
    println!("Dry run — would move {files} and leave {entries} untouched.");
  } else {
    println!("Done — moved {files} and left {entries} untouched.");
  }
}

/// Return `count` + noun with correct pluralisation (`1 file`, `2 files`).
fn noun(count: usize, singular: &str, plural: &str) -> String {
  if count == 1 {
    format!("{count} {singular}")
  } else {
    format!("{count} {plural}")
  }
}
