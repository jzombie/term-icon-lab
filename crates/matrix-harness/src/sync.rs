//! File-based out-of-band IPC between the harness and the capture
//! orchestrator.
//!
//! `stdin` is strictly reserved for terminal report bytes; page synchronization
//! flows exclusively through `.ready` / `.ack` marker files under a directory
//! supplied via the required `--sync-dir` flag.

use std::fmt;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Poll cadence for acknowledgment files.
pub const ACK_POLL_INTERVAL: Duration = Duration::from_millis(50);
/// Default deadline for the orchestrator to acknowledge a page.
pub const ACK_TIMEOUT: Duration = Duration::from_secs(60);

/// Errors surfaced by the sync channel; every variant maps to fail-fast exit.
#[derive(Debug)]
pub enum SyncError {
    AckTimeout { path: PathBuf },
    Io { path: PathBuf, source: io::Error },
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyncError::AckTimeout { path } => {
                write!(f, "ack timeout waiting for {}", path.display())
            }
            SyncError::Io { path, source } => {
                write!(f, "io error on {}: {source}", path.display())
            }
        }
    }
}

impl std::error::Error for SyncError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SyncError::AckTimeout { .. } => None,
            SyncError::Io { source, .. } => Some(source),
        }
    }
}

/// Marker-file channel rooted at the orchestrator-provided sync directory.
#[derive(Clone, Debug)]
pub struct SyncChannel {
    dir: PathBuf,
    pid: u32,
}

impl SyncChannel {
    /// Bind the channel to an orchestrator-created directory.
    ///
    /// The directory must already exist; the harness never invents its own
    /// path (missing flag is a CLI usage error handled in `main`).
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self {
            dir: dir.into(),
            pid: std::process::id(),
        }
    }

    /// The sync directory in use.
    #[must_use]
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    fn ready_path(&self, page: u32) -> PathBuf {
        self.dir
            .join(format!("term_icon_{}_page_{page}.ready", self.pid))
    }

    fn ack_path(&self, page: u32) -> PathBuf {
        self.dir
            .join(format!("term_icon_{}_page_{page}.ack", self.pid))
    }

    /// Signal that page `page` is rendered and captured-ready.
    pub fn write_ready(&self, page: u32) -> Result<(), SyncError> {
        let path = self.ready_path(page);
        std::fs::write(&path, b"ready\n").map_err(|source| SyncError::Io {
            path: path.clone(),
            source,
        })
    }

    /// Block until the orchestrator acknowledges page `page`.
    pub fn wait_ack(&self, page: u32, timeout: Duration) -> Result<(), SyncError> {
        let ack = self.ack_path(page);
        let deadline = Instant::now() + timeout;
        while !ack.exists() {
            if Instant::now() >= deadline {
                return Err(SyncError::AckTimeout { path: ack });
            }
            std::thread::sleep(ACK_POLL_INTERVAL);
        }
        Ok(())
    }

    /// Remove both markers for `page` after a successful handshake.
    pub fn cleanup(&self, page: u32) {
        let _ = std::fs::remove_file(self.ready_path(page));
        let _ = std::fs::remove_file(self.ack_path(page));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    fn tempdir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ti_sync_test_{name}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn ready_then_ack_roundtrip_and_cleanup() {
        let dir = tempdir("roundtrip");
        let harness = SyncChannel::new(dir.clone());
        harness.write_ready(3).unwrap();
        let ready = harness
            .dir()
            .join(format!("term_icon_{}_page_3.ready", harness.pid));
        assert!(ready.exists());

        let writer = harness.clone();
        let t = thread::spawn(move || {
            thread::sleep(Duration::from_millis(80));
            std::fs::write(
                writer
                    .dir()
                    .join(format!("term_icon_{}_page_3.ack", writer.pid)),
                b"ack\n",
            )
            .unwrap();
        });
        harness.wait_ack(3, Duration::from_secs(2)).unwrap();
        t.join().unwrap();

        harness.cleanup(3);
        assert!(!ready.exists());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn ack_timeout_fails_fast() {
        let dir = tempdir("timeout");
        let harness = SyncChannel::new(dir.clone());
        let err = harness.wait_ack(9, Duration::from_millis(120)).unwrap_err();
        assert!(matches!(err, SyncError::AckTimeout { .. }));
        let _ = std::fs::remove_dir_all(dir);
    }
}
