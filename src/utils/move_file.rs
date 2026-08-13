use crate::error::AnyError;
use std::fs;
use std::path::Path;

/// Move `from` into directory `to`, keeping the original file name unless it
/// collides with an existing file (in which case a numeric suffix is added so
/// nothing is ever overwritten).
///
/// Returns the fully resolved destination path. When `dry_run` is true, the
/// file is not renamed but the destination it WOULD land at is still returned,
/// so the caller can preview without touching disk.
pub fn move_file(
  from: &Path,
  to: &str,
  dry_run: bool,
  by_date: bool,
) -> Result<String, AnyError> {
  let file_name =
    from.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
      AnyError::FileName {
        path: from.to_path_buf(),
      }
    })?;
  let target_dir = date_target(to, from, by_date)?;
  // Ensure the destination parent directory exists. Lazy creation keeps
  // categories that receive no files from ever being materialised on disk;
  // by-date nesting always needs the year/month dirs, and flat moves need
  // the category directory itself.
  if !dry_run {
    fs::create_dir_all(&target_dir).map_err(|source| AnyError::CreateDir {
      name: target_dir.clone(),
      source,
    })?;
  }
  let dest = unique_dest(&target_dir, file_name);

  if !dry_run {
    fs::rename(from, &dest).map_err(|source| AnyError::MoveFile {
      to: dest.clone(),
      source,
    })?;
  }
  Ok(dest)
}

/// Build a destination path `{dir}/{file_name}` that does not collide with an
/// existing file. If `{dir}/{file_name}` already exists, the name is suffixed
/// with `-1`, `-2`, ... before the extension (or at the end when there is no
/// extension) until a free name is found, so incoming files never overwrite
/// existing ones.
fn unique_dest(dir: &str, file_name: &str) -> String {
  let plain = format!("{dir}/{file_name}");
  if !Path::new(&plain).exists() {
    return plain;
  }

  let path = Path::new(file_name);
  let stem = path
    .file_stem()
    .and_then(|s| s.to_str())
    .unwrap_or(file_name);
  let ext = path.extension().and_then(|e| e.to_str());

  for n in 1.. {
    let candidate = ext.map_or_else(
      || format!("{dir}/{stem}-{n}"),
      |ext| format!("{dir}/{stem}-{n}.{ext}"),
    );
    if !Path::new(&candidate).exists() {
      return candidate;
    }
  }
  String::new()
}

/// Resolve the directory that a file should land in. When `by_date` is true,
/// this appends the file's modification year and month (`{to}/{year}/{month:02}`)
/// so files are grouped chronologically inside their category. Otherwise the
/// category directory `to` is returned unchanged.
fn date_target(
  to: &str,
  from: &Path,
  by_date: bool,
) -> Result<String, AnyError> {
  if !by_date {
    return Ok(to.to_string());
  }
  let metadata =
    fs::metadata(from).map_err(|source| AnyError::GetMetadata {
      path: from.to_path_buf(),
      source,
    })?;
  let modified =
    metadata
      .modified()
      .map_err(|source| AnyError::GetMetadata {
        path: from.to_path_buf(),
        source,
      })?;
  let ts: chrono::DateTime<chrono::Utc> = modified.into();
  Ok(format!("{to}/{}/{:02}", ts.format("%Y"), ts.format("%m")))
}

#[cfg(test)]
mod test {
  #![allow(clippy::expect_used, clippy::unwrap_used, clippy::indexing_slicing)]
  use super::*;
  use crate::test_utils::TempDir;
  use std::fs::File;

  #[test]
  fn moves_file_into_target_dir() {
    let temp = TempDir::new("move");
    let src = temp.path().join("a.txt");
    File::create(&src).expect("create source file");

    let dst_dir = temp.path().join("dst");
    fs::create_dir(&dst_dir).expect("create target dir");

    move_file(&src, dst_dir.to_str().expect("utf-8"), false, false)
      .expect("move should succeed");

    let moved = dst_dir.join("a.txt");
    assert!(moved.exists(), "file should exist in target dir");
    assert!(!src.exists(), "source file should be gone");
  }

  #[test]
  fn move_creates_missing_parent_dir_lazily() {
    let temp = TempDir::new("move_mkdir");
    let src = temp.path().join("a.txt");
    File::create(&src).expect("create source file");

    // Parent does not exist yet — the move must create it.
    let fresh = temp.path().join("brand_new_cat");
    move_file(&src, fresh.to_str().expect("utf-8"), false, false)
      .expect("move creates destination dir");
    assert!(
      fresh.join("a.txt").exists(),
      "file should land inside lazily created dir"
    );
    assert!(!src.exists(), "source file should be gone");
  }

  #[test]
  fn dry_run_leaves_source_and_dest_untouched() {
    let temp = TempDir::new("move_dry");
    let src = temp.path().join("a.txt");
    fs::write(&src, "content").expect("create source file");

    let dst_dir = temp.path().join("dst");
    fs::create_dir(&dst_dir).expect("create target dir");

    move_file(&src, dst_dir.to_str().expect("utf-8"), true, false)
      .expect("dry run should succeed");

    // Nothing moved.
    assert!(src.exists(), "source must remain during dry run");
    assert!(
      !dst_dir.join("a.txt").exists(),
      "no file should land in target during dry run"
    );
    assert_eq!(
      fs::read_to_string(&src).expect("read source"),
      "content",
      "source content unchanged"
    );
  }

  #[test]
  fn collision_renames_incoming_file_instead_of_overwriting() {
    let temp = TempDir::new("move_collide");
    let dst_dir = temp.path().join("dst");
    fs::create_dir(&dst_dir).expect("create target dir");

    // Pre-existing file that must not be overwritten.
    fs::write(dst_dir.join("a.txt"), "original")
      .expect("create existing target");

    let src = temp.path().join("a.txt");
    fs::write(&src, "new").expect("create source file");

    move_file(&src, dst_dir.to_str().expect("utf-8"), false, false)
      .expect("move should succeed");

    // Original preserved.
    assert_eq!(
      fs::read_to_string(dst_dir.join("a.txt")).expect("read original"),
      "original",
      "existing file must not be overwritten"
    );
    // Source moved somewhere into dst, not left behind.
    assert!(!src.exists(), "source file should be gone");
    // Exactly one extra file landed in dst with a distinct name.
    let landed: Vec<_> = fs::read_dir(&dst_dir)
      .expect("read dst")
      .map(|e| e.expect("entry").file_name().into_string().expect("name"))
      .collect();
    assert_eq!(landed.len(), 2, "original + renamed incoming: {landed:?}");
    let incoming = landed
      .iter()
      .find(|n| n.as_str() != "a.txt")
      .expect("a renamed copy exists");
    assert_eq!(
      fs::read_to_string(dst_dir.join(incoming)).expect("read incoming"),
      "new",
      "incoming content preserved under renamed path"
    );
  }

  #[test]
  fn collision_then_repeated_move_keeps_original_intact() {
    let temp = TempDir::new("move_collide2");
    let dst_dir = temp.path().join("dst");
    fs::create_dir(&dst_dir).expect("create target dir");
    fs::write(dst_dir.join("a.txt"), "original")
      .expect("create existing target");

    let src = temp.path().join("a.txt");
    fs::write(&src, "copy1").expect("create source");
    move_file(&src, dst_dir.to_str().expect("utf-8"), false, false)
      .expect("move 1");

    let src2 = temp.path().join("a.txt");
    fs::write(&src2, "copy2").expect("create source 2");
    move_file(&src2, dst_dir.to_str().expect("utf-8"), false, false)
      .expect("move 2");

    // Original never touched across both collisions.
    assert_eq!(
      fs::read_to_string(dst_dir.join("a.txt")).expect("read original"),
      "original"
    );
    // Two distinct renamed copies, no overwrite between them.
    let names: Vec<String> = fs::read_dir(&dst_dir)
      .expect("read dst")
      .map(|e| e.expect("entry").file_name().into_string().expect("name"))
      .collect();
    assert_eq!(names.len(), 3, "original + 2 copies: {names:?}");
  }

  #[test]
  fn date_target_flat_when_not_by_date() {
    let temp = TempDir::new("date_flat");
    let src = temp.path().join("a.txt");
    fs::write(&src, "x").expect("create source");

    let result = date_target("/cats", &src, false).expect("no date grouping");
    assert_eq!(result, "/cats", "by_date=false keeps category dir flat");
  }

  #[test]
  fn date_target_appends_year_and_month_when_by_date() {
    let temp = TempDir::new("date_group");
    let src = temp.path().join("a.txt");
    fs::write(&src, "x").expect("create source");

    let result = date_target("/cats", &src, true).expect("date grouping");
    let pieces: Vec<&str> = result.split('/').collect();
    // [""]/cats/<year>/<month>
    assert_eq!(pieces.len(), 4, "category + year + month: {result:?}");
    assert_eq!(pieces[1], "cats");
    let year: u32 = pieces[2].parse().expect("year is numeric");
    assert_eq!(pieces[3].len(), 2, "month zero-padded to 2 digits");
    let month: u32 = pieces[3].parse().expect("month is numeric");
    assert!((1900..=9999).contains(&year), "year in plausible range");
    assert!((1..=12).contains(&month), "month in plausible range");
  }

  #[test]
  fn by_date_move_lands_inside_nested_year_month_dir() {
    let temp = TempDir::new("move_date");
    let src = temp.path().join("a.txt");
    fs::write(&src, "content").expect("create source");

    let dst_dir = temp.path().join("docs");
    let expected_parent =
      date_target(dst_dir.to_str().expect("utf-8"), &src, true)
        .expect("compute target");

    move_file(&src, dst_dir.to_str().expect("utf-8"), false, true)
      .expect("date move succeeds");

    assert!(
      !src.exists(),
      "source moved out during by-date organization"
    );
    let expect = format!("{expected_parent}/a.txt");
    assert!(Path::new(&expect).exists(), "file landed at: {}", &expect);
    assert_eq!(
      fs::read_to_string(&expect).expect("read moved file"),
      "content",
      "content preserved after date move"
    );
  }
}
