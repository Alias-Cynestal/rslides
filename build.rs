use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=GSTREAMER_1_0_ROOT_MSVC");
    println!("cargo:rerun-if-env-changed=GSTREAMER_1_0_ROOT_X86_64");

    if !cfg!(target_os = "windows") {
        return;
    }

    let Some(gst_root) = find_gstreamer_root() else {
        println!(
            "cargo:warning=GStreamer runtime not found. Set GSTREAMER_1_0_ROOT_MSVC to bundle DLLs."
        );
        return;
    };

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap_or_default());
    let Some(profile_dir) = find_profile_dir_from_out_dir(&out_dir) else {
        println!(
            "cargo:warning=Unable to resolve Cargo target profile directory from OUT_DIR={}",
            out_dir.display()
        );
        return;
    };

    let bin_dir = gst_root.join("bin");
    let plugin_dir = gst_root.join("lib").join("gstreamer-1.0");

    if !bin_dir.is_dir() {
        println!(
            "cargo:warning=GStreamer bin directory missing: {}",
            bin_dir.display()
        );
        return;
    }

    if let Err(err) = copy_dlls(&bin_dir, &profile_dir) {
        println!("cargo:warning=Failed to copy GStreamer DLLs: {err}");
    }

    let scanner = bin_dir.join("gst-plugin-scanner.exe");
    if scanner.is_file() {
        let _ = fs::copy(&scanner, profile_dir.join("gst-plugin-scanner.exe"));
    }

    if plugin_dir.is_dir() {
        let dst_plugins = profile_dir.join("gstreamer-1.0");
        if let Err(err) = copy_plugin_dlls(&plugin_dir, &dst_plugins) {
            println!("cargo:warning=Failed to copy GStreamer plugins: {err}");
        }
    } else {
        println!(
            "cargo:warning=GStreamer plugin directory missing: {}",
            plugin_dir.display()
        );
    }
}

fn find_gstreamer_root() -> Option<PathBuf> {
    let from_env = env::var("GSTREAMER_1_0_ROOT_MSVC")
        .ok()
        .or_else(|| env::var("GSTREAMER_1_0_ROOT_X86_64").ok())
        .map(PathBuf::from)
        .filter(|p| p.is_dir());

    if from_env.is_some() {
        return from_env;
    }

    let defaults = [
        r"C:\gstreamer\1.0\msvc_x86_64",
        r"C:\Program Files\gstreamer\1.0\msvc_x86_64",
    ];

    defaults
        .iter()
        .map(PathBuf::from)
        .find(|candidate| candidate.is_dir())
}

fn find_profile_dir_from_out_dir(out_dir: &Path) -> Option<PathBuf> {
    let mut cursor = out_dir.to_path_buf();
    while cursor.file_name().and_then(|s| s.to_str()) != Some("build") {
        if !cursor.pop() {
            return None;
        }
    }

    if !cursor.pop() {
        return None;
    }
    Some(cursor)
}

fn copy_dlls(src_bin_dir: &Path, dst_dir: &Path) -> io::Result<()> {
    fs::create_dir_all(dst_dir)?;
    for entry in fs::read_dir(src_bin_dir)? {
        let entry = entry?;
        let src_path = entry.path();
        let is_dll = src_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("dll"))
            .unwrap_or(false);

        if is_dll {
            let dst_path = dst_dir.join(entry.file_name());
            fs::copy(src_path, dst_path)?;
        }
    }
    Ok(())
}

fn copy_plugin_dlls(src_plugins: &Path, dst_plugins: &Path) -> io::Result<()> {
    fs::create_dir_all(dst_plugins)?;
    for entry in fs::read_dir(src_plugins)? {
        let entry = entry?;
        let src_path = entry.path();
        if src_path.is_dir() {
            continue;
        }

        let is_dll = src_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("dll"))
            .unwrap_or(false);

        if is_dll {
            let dst_path = dst_plugins.join(entry.file_name());
            fs::copy(src_path, dst_path)?;
        }
    }
    Ok(())
}
