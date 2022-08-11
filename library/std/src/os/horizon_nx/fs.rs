#![stable(feature = "metadata_ext", since = "1.1.0")]

use crate::fs::Metadata;

/// OS-specific extensions to [`fs::Metadata`].
///
/// [`fs::Metadata`]: crate::fs::Metadata
#[stable(feature = "metadata_ext", since = "1.1.0")]
pub trait MetadataExt {
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_dev(&self) -> u64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_ino(&self) -> u64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_mode(&self) -> u32;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_nlink(&self) -> u64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_uid(&self) -> u32;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_gid(&self) -> u32;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_rdev(&self) -> u64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_size(&self) -> u64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_atime(&self) -> i64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_atime_nsec(&self) -> i64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_mtime(&self) -> i64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_mtime_nsec(&self) -> i64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_ctime(&self) -> i64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_ctime_nsec(&self) -> i64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_blksize(&self) -> u64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_blocks(&self) -> u64;
}

#[stable(feature = "metadata_ext", since = "1.1.0")]
impl MetadataExt for Metadata {
    fn st_dev(&self) -> u64 {
        todo!()
    }
    fn st_ino(&self) -> u64 {
        todo!()
    }
    fn st_mode(&self) -> u32 {
        todo!()
    }
    fn st_nlink(&self) -> u64 {
        todo!()
    }
    fn st_uid(&self) -> u32 {
        todo!()
    }
    fn st_gid(&self) -> u32 {
        todo!()
    }
    fn st_rdev(&self) -> u64 {
        todo!()
    }
    fn st_size(&self) -> u64 {
        todo!()
    }
    fn st_atime(&self) -> i64 {
        todo!()
    }
    fn st_atime_nsec(&self) -> i64 {
        todo!()
    }
    fn st_mtime(&self) -> i64 {
        todo!()
    }
    fn st_mtime_nsec(&self) -> i64 {
        todo!()
    }
    fn st_ctime(&self) -> i64 {
        todo!()
    }
    fn st_ctime_nsec(&self) -> i64 {
        todo!()
    }
    fn st_blksize(&self) -> u64 {
        todo!()
    }
    fn st_blocks(&self) -> u64 {
        todo!()
    }
}
