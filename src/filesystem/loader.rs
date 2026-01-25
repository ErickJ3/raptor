use super::node::FileNode;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct DirectoryContents {
    pub nodes: Vec<FileNode>,
    pub grid_width: i32,
}

#[cfg(target_os = "windows")]
fn is_hidden(entry: &fs::DirEntry) -> bool {
    use std::os::windows::fs::MetadataExt;
    entry.metadata()
        .map(|m| m.file_attributes() & 0x02 != 0)
        .unwrap_or(false)
}

#[cfg(not(target_os = "windows"))]
fn is_hidden(entry: &fs::DirEntry) -> bool {
    entry.file_name().to_string_lossy().starts_with(".")
}

fn is_zfs_snapshot_dir(entry: &fs::DirEntry) -> bool {
    // Ignore well-known ZFS snapshot directories
    if !entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
        return false;
    }
    let name = entry.file_name();
    let name = name.to_string_lossy();
    if name == "@Recently-Snapshot"
        || name == "@auto" || name == "@daily" || name == "@weekly" || name == "@monthly"
        || name == "@yearly" || name == "@manual" || name == "@tmp"
        || name == ".zfs"
        || name.starts_with("@-")
        || (name.starts_with("@") && name.len() > 1 && name.chars().nth(1).map_or(false, |c| c.is_ascii_digit()))
    {
        return true;
    }
    false
}
}

#[cfg(target_os = "macos")]
fn is_macos_package(path: &PathBuf) -> bool {
    // Check if path ends with known macOS package extensions
    match path.extension() {
        Some(ext) => {
            let ext_str = ext.to_string_lossy().to_lowercase();
            matches!(ext_str.as_str(), 
                "app" | "framework" | "bundle" | "plugin" | "wdgt" | 
                "action" | "mdimporter" | "prefpane" | "qlgenerator" | 
                "saver" | "colorpicker" | "scriptSuite" | "scriptTerminology")
        }
        None => false,
    }
}

#[cfg(not(target_os = "macos"))]
fn is_macos_package(_path: &PathBuf) -> bool {
    false
}

fn calculate_directory_size(path: &PathBuf, max_depth: Option<usize>, current_depth: usize) -> u64 {
    // On macOS, skip deep traversal of packages and use faster approximate size
    #[cfg(target_os = "macos")]
    if is_macos_package(path) {
        // Use du command or allocatedFileSize for faster package scanning
        if let Ok(metadata) = fs::metadata(path) {
            return metadata.len();
        }
    }
    
    let max_depth_for_walk = max_depth.map(|d| d.saturating_sub(current_depth));
    
    WalkDir::new(path)
        .max_depth(max_depth_for_walk.unwrap_or(usize::MAX))
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            // Skip well-known ZFS snapshot directories
            let name = entry.file_name().to_string_lossy();
            if entry.file_type().is_dir() && (
                name == "@Recently-Snapshot"
                || name == "@auto" || name == "@daily" || name == "@weekly" || name == "@monthly"
                || name == "@yearly" || name == "@manual" || name == "@tmp"
                || name == ".zfs"
                || name.starts_with("@-")
                || (name.starts_with("@") && name.len() > 1 && name.chars().nth(1).map_or(false, |c| c.is_ascii_digit()))
            ) {
                return false;
            }
            true
        })
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .map(|metadata| metadata.len())
        .sum()
}

pub fn load_directory(
    path: &PathBuf,
    show_hidden: bool,
    deep_scan: bool,
    max_depth: Option<usize>,
) -> Result<DirectoryContents, std::io::Error> {
    let read_dir = fs::read_dir(path)?;

    let mut nodes: Vec<FileNode> = read_dir
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            if !show_hidden {
                return !is_hidden(entry);
            }
            true
        })
        .filter(|entry| !is_zfs_snapshot_dir(entry))
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            let is_dir = metadata.is_dir();
            let size = if is_dir {
                if deep_scan {
                    calculate_directory_size(&entry.path(), max_depth, 1)
                } else {
                    0
                }
            } else {
                metadata.len()
            };
            let children_count = if is_dir {
                fs::read_dir(entry.path()).map(|d| d.count()).unwrap_or(0)
            } else {
                0
            };
            Some(FileNode::new(
                entry.file_name().to_string_lossy().to_string(),
                entry.path(),
                is_dir,
                size,
                children_count,
            ))
        })
        .collect();

    nodes.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name)));

    let grid_width = (nodes.len() as f32).sqrt().ceil() as i32;
    for (i, node) in nodes.iter_mut().enumerate() {
        let x = (i as i32) % grid_width;
        let z = (i as i32) / grid_width;
        node.grid_pos = (x - grid_width / 2, z - grid_width / 2);
    }

    Ok(DirectoryContents { nodes, grid_width })
}

pub fn get_path_components(path: &PathBuf) -> Vec<(String, PathBuf)> {
    let mut components = vec![];
    let mut current = path.clone();

    loop {
        let name = current
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string());
        components.push((name, current.clone()));

        if let Some(parent) = current.parent() {
            if parent == current {
                break;
            }
            current = parent.to_path_buf();
        } else {
            break;
        }
    }

    components.reverse();
    components
}

pub fn count_by_type(nodes: &[FileNode]) -> (usize, usize) {
    let dirs = nodes.iter().filter(|n| n.is_dir).count();
    let files = nodes.iter().filter(|n| !n.is_dir).count();
    (dirs, files)
}
