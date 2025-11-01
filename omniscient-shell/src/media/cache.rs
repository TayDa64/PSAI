//! Media cache management

use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Media cache entry
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub last_accessed: std::time::SystemTime,
}

/// Media cache with intelligent pruning
pub struct MediaCache {
    entries: Arc<RwLock<HashMap<String, CacheEntry>>>,
    max_size_mb: u64,
}

impl MediaCache {
    pub fn new(max_size_mb: u64) -> Self {
        MediaCache {
            entries: Arc::new(RwLock::new(HashMap::new())),
            max_size_mb,
        }
    }

    /// Add entry to cache
    pub async fn add(&self, key: String, path: PathBuf, size_bytes: u64) -> Result<()> {
        let entry = CacheEntry {
            path,
            size_bytes,
            last_accessed: std::time::SystemTime::now(),
        };

        let mut entries = self.entries.write().await;
        entries.insert(key, entry);

        // Check if pruning needed
        self.prune_if_needed(&mut entries).await?;

        Ok(())
    }

    /// Get entry from cache
    pub async fn get(&self, key: &str) -> Option<PathBuf> {
        let mut entries = self.entries.write().await;
        if let Some(entry) = entries.get_mut(key) {
            entry.last_accessed = std::time::SystemTime::now();
            Some(entry.path.clone())
        } else {
            None
        }
    }

    /// Prune cache if needed (LRU)
    async fn prune_if_needed(&self, entries: &mut HashMap<String, CacheEntry>) -> Result<()> {
        let total_size: u64 = entries.values().map(|e| e.size_bytes).sum();
        let max_size_bytes = self.max_size_mb * 1024 * 1024;

        if total_size > max_size_bytes {
            tracing::info!(
                "Cache size {} MB exceeds limit {} MB, pruning...",
                total_size / (1024 * 1024),
                self.max_size_mb
            );

            // Sort by last accessed (LRU)
            let mut sorted: Vec<_> = entries.iter().collect();
            sorted.sort_by_key(|(_, entry)| entry.last_accessed);

            // Remove oldest entries until under limit
            let mut current_size = total_size;
            for (key, entry) in sorted.iter() {
                if current_size <= max_size_bytes {
                    break;
                }

                // Delete file
                if let Err(e) = std::fs::remove_file(&entry.path) {
                    tracing::warn!(
                        "Failed to delete cached file {}: {}",
                        entry.path.display(),
                        e
                    );
                }

                entries.remove(*key);
                current_size -= entry.size_bytes;
                tracing::debug!("Pruned cache entry: {}", key);
            }
        }

        Ok(())
    }

    /// Clear entire cache
    pub async fn clear(&self) -> Result<()> {
        let mut entries = self.entries.write().await;

        for (_, entry) in entries.iter() {
            let _ = std::fs::remove_file(&entry.path);
        }

        entries.clear();
        tracing::info!("Media cache cleared");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_operations() {
        let cache = MediaCache::new(100); // 100 MB

        cache
            .add("test".to_string(), PathBuf::from("/tmp/test.jpg"), 1024)
            .await
            .unwrap();

        let path = cache.get("test").await;
        assert!(path.is_some());
    }
}
