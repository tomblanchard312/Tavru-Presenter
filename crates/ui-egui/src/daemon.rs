//! Start `sharedai-daemon` unless something is already listening, using the same
//! search order as Tavru Sheets.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

/// Start the daemon on a background thread the first time the speech panel is open.
pub fn launch_when_open(open: bool) {
    if !open {
        return;
    }
    static STARTED: AtomicBool = AtomicBool::new(false);
    if STARTED.swap(true, Ordering::Relaxed) {
        return;
    }
    let _ = std::thread::Builder::new().name("sharedai-launch".into()).spawn(|| {
        let _ = ensure_running();
    });
}

/// Find and start `sharedai-daemon` for the per-user endpoint.
pub fn ensure_running() -> Result<(), String> {
    let endpoint = endpoint()?;
    #[cfg(unix)]
    {
        return unix::ensure(&endpoint);
    }
    #[cfg(windows)]
    {
        return windows::ensure(&endpoint);
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = endpoint;
        Err("sharedai-daemon isn't installed".into())
    }
}

fn endpoint() -> Result<PathBuf, String> {
    if let Some(socket) = std::env::var_os("SHAREDAI_SOCKET").filter(|value| !value.is_empty()) {
        return Ok(PathBuf::from(socket));
    }
    #[cfg(unix)]
    {
        let dir = std::env::var_os("XDG_RUNTIME_DIR").filter(|value| !value.is_empty()).ok_or("Oqelith has no local endpoint.")?;
        let dir = PathBuf::from(dir);
        if !dir.is_absolute() {
            return Err("Oqelith has no local endpoint.".into());
        }
        return Ok(dir.join("sharedai").join("ipc.sock"));
    }
    #[cfg(windows)]
    {
        return std::env::var_os("SHAREDAI_PIPE")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
            .ok_or_else(|| "Oqelith has no local endpoint.".to_string());
    }
    #[cfg(not(any(unix, windows)))]
    Err("Oqelith has no local endpoint.".into())
}

fn daemon_name() -> &'static str {
    if cfg!(windows) { "sharedai-daemon.exe" } else { "sharedai-daemon" }
}

fn find_daemon() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("SHAREDAI_DAEMON").filter(|path| !path.is_empty()) {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Some(path);
        }
    }
    let exe = std::env::current_exe().ok();
    candidates(exe.as_deref(), std::env::var_os("PATH").as_deref()).into_iter().find(|path| path.is_file())
}

fn candidates(exe: Option<&Path>, path: Option<&OsStr>) -> Vec<PathBuf> {
    let name = daemon_name();
    let mut out = Vec::new();
    if let Some(exe) = exe {
        let Some(dir) = exe.parent() else { return out; };
        out.push(dir.join("enki").join(name));
        if let (Some(prefix), Some(stem)) = (dir.parent(), exe.file_stem()) {
            out.push(prefix.join("lib").join("celltrellis").join("enki").join(name));
            out.push(prefix.join("lib").join(stem).join(name));
        }
        out.push(dir.join(name));
    }
    if let Some(path) = path {
        out.extend(std::env::split_paths(path).map(|dir| dir.join(name)));
    }
    out
}

#[cfg(unix)]
mod unix {
    use super::find_daemon;
    use std::path::Path;
    use std::time::{Duration, Instant};

    pub fn ensure(endpoint: &Path) -> Result<(), String> {
        use std::os::unix::net::UnixStream;
        use std::os::unix::process::CommandExt;
        if UnixStream::connect(endpoint).is_ok() {
            return Ok(());
        }
        let daemon = find_daemon().ok_or_else(|| "sharedai-daemon isn't installed".to_string())?;
        if let Some(dir) = endpoint.parent() {
            use std::os::unix::fs::DirBuilderExt;
            match std::fs::DirBuilder::new().mode(0o700).recursive(true).create(dir) {
                Err(error) if error.kind() != std::io::ErrorKind::AlreadyExists => return Err(error.to_string()),
                _ => {}
            }
        }
        let mut command = std::process::Command::new(&daemon);
        command
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .env("SHAREDAI_SOCKET", endpoint)
            .process_group(0);
        let mut child = command.spawn().map_err(|error| format!("sharedai-daemon didn't start: {error}"))?;
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if UnixStream::connect(endpoint).is_ok() {
                return Ok(());
            }
            if let Ok(Some(status)) = child.try_wait() {
                return Err(format!("sharedai-daemon didn't start: exited with {status}"));
            }
            if Instant::now() >= deadline {
                return Err("sharedai-daemon didn't start: not listening after 10 s".into());
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

#[cfg(windows)]
mod windows {
    use super::find_daemon;
    use std::path::Path;
    use std::time::{Duration, Instant};

    fn listening(endpoint: &Path) -> bool {
        std::fs::OpenOptions::new().read(true).write(true).open(endpoint).is_ok()
    }

    pub fn ensure(endpoint: &Path) -> Result<(), String> {
        use std::os::windows::process::CommandExt;
        if listening(endpoint) {
            return Ok(());
        }
        let daemon = find_daemon().ok_or_else(|| "sharedai-daemon isn't installed".to_string())?;
        let mut command = std::process::Command::new(&daemon);
        command
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .creation_flags(0x0800_0000)
            .env("SHAREDAI_PIPE", endpoint);
        let mut child = command.spawn().map_err(|error| format!("sharedai-daemon didn't start: {error}"))?;
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if listening(endpoint) {
                return Ok(());
            }
            if let Ok(Some(status)) = child.try_wait() {
                return Err(format!("sharedai-daemon didn't start: exited with {status}"));
            }
            if Instant::now() >= deadline {
                return Err("sharedai-daemon didn't start: not listening after 10 s".into());
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}
