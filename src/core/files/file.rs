use std::{fs::Metadata, path::{Path, PathBuf}};

use anyhow::Result;
use tokio::fs;

#[derive(Clone, Debug)]
pub struct File {
	pub name:        String,
	pub path:        PathBuf,
	pub meta:        Metadata,
	pub length:      Option<u64>,
	pub is_link:     bool,
	pub is_hidden:   bool,
	pub is_selected: bool,
}

impl File {
	#[inline]
	pub async fn from(path: &Path) -> Result<File> {
		let meta = fs::metadata(path).await?;
		Ok(Self::from_meta(path, meta).await)
	}

	pub async fn from_meta(path: &Path, mut meta: Metadata) -> File {
		let is_link = meta.is_symlink();
		if is_link {
			meta = fs::metadata(&path).await.unwrap_or(meta);
		}

		let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
		let length = if meta.is_dir() { None } else { Some(meta.len()) };
		let is_hidden = name.starts_with('.');
		File { name, path: path.to_path_buf(), meta, length, is_link, is_hidden, is_selected: false }
	}
}
