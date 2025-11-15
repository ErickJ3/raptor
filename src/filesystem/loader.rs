use super::node::FileNode;
use std::fs;
use std::path::PathBuf;

pub struct DirectoryContents {
    pub nodes: Vec<FileNode>,
    pub grid_width: i32,
}

pub fn load_directory(path: &PathBuf) -> Result<DirectoryContents, std::io::Error> {
    let read_dir = fs::read_dir(path)?;

    let mut nodes: Vec<FileNode> = read_dir
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            let is_dir = metadata.is_dir();
            let size = if is_dir { 0 } else { metadata.len() };
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
