//! Shared test utilities.
//!
//! Provides a dependency-free, RAII-guarded temporary directory so tests that
//! touch the real filesystem clean up after themselves even when an assertion
//! panics mid-test (avoids leaking artifacts on repeated runs).

// Panicking on failure is the intended behaviour for a test helper.
#![allow(clippy::expect_used)]

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A unique temporary directory that is automatically removed on `Drop`.
///
/// Removes itself recursively on drop, so it is safe to use with subdirectories
/// and files created beneath it.
pub struct TempDir {
  path: PathBuf,
}

impl TempDir {
  /// Create a new unique temporary directory named `prefix`.
  ///
  /// # Panics
  ///
  /// Panics if the directory cannot be created (e.g. the OS temp dir is
  /// unwritable), since a test cannot meaningfully proceed without it.
  pub fn new(prefix: &str) -> Self {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let base = std::env::temp_dir();
    let path = base.join(format!("{prefix}-{}-{n}", std::process::id()));
    fs::create_dir(&path).expect("unable to create temp dir");
    Self { path }
  }

  /// Absolute path to the temporary directory.
  pub fn path(&self) -> &Path {
    &self.path
  }
}

impl Drop for TempDir {
  fn drop(&mut self) {
    let _ = fs::remove_dir_all(&self.path);
  }
}
