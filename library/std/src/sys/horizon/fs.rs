use crate::ffi::OsString;
use crate::fmt;
use crate::hash::Hash;
use crate::io::{self, IoSlice, IoSliceMut, ReadBuf, SeekFrom};
use crate::path::{Path, PathBuf};
use crate::sys::time::SystemTime;
use crate::sys::unsupported;

use horizon_ipcdef::fssrv::{IFile, IDirectory, DirectoryEntry, DirectoryEntryType};

#[derive(Debug)]
pub struct File(IFile);

pub struct FileAttr(!);

#[derive(Debug)]
pub struct ReadDir(IDirectory);

#[derive(Clone)]
pub struct DirEntry(DirectoryEntry);

#[derive(Clone, Debug)]
pub struct OpenOptions {
    // generic
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
    // system-specific
    // (none yet)
}

pub struct FilePermissions(!);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileType(DirectoryEntryType);

#[derive(Debug)]
pub struct DirBuilder {}

impl FileAttr {
    pub fn size(&self) -> u64 {
        self.0
    }

    pub fn perm(&self) -> FilePermissions {
        self.0
    }

    pub fn file_type(&self) -> FileType {
        self.0
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        self.0
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        self.0
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        self.0
    }
}

impl Clone for FileAttr {
    fn clone(&self) -> FileAttr {
        self.0
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        self.0
    }

    pub fn set_readonly(&mut self, _readonly: bool) {
        self.0
    }
}

impl Clone for FilePermissions {
    fn clone(&self) -> FilePermissions {
        self.0
    }
}

impl PartialEq for FilePermissions {
    fn eq(&self, _other: &FilePermissions) -> bool {
        self.0
    }
}

impl Eq for FilePermissions {}

impl fmt::Debug for FilePermissions {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.0 == DirectoryEntryType::Directory
    }

    pub fn is_file(&self) -> bool {
        self.0 == DirectoryEntryType::File
    }

    pub fn is_symlink(&self) -> bool {
        false
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        todo!()
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        todo!()
    }

    pub fn file_name(&self) -> OsString {
        todo!()
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        todo!()
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        todo!()
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            // generic
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
            // system-specific
        }
    }

    pub fn read(&mut self, read: bool) {
        self.read = read;
    }
    pub fn write(&mut self, write: bool) {
        self.write = write;
    }
    pub fn append(&mut self, append: bool) {
        self.append = append;
    }
    pub fn truncate(&mut self, truncate: bool) {
        self.truncate = truncate;
    }
    pub fn create(&mut self, create: bool) {
        self.create = create;
    }
    pub fn create_new(&mut self, create_new: bool) {
        self.create_new = create_new;
    }

}

impl File {
    pub fn open(_path: &Path, _opts: &OpenOptions) -> io::Result<File> {
        unsupported()
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        unsupported()
    }

    pub fn fsync(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn datasync(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        unsupported()
    }

    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn read_vectored(&self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_read_vectored(&self) -> bool {
        todo!()
    }

    pub fn read_buf(&self, _buf: &mut ReadBuf<'_>) -> io::Result<()> {
        unsupported()
    }

    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn write_vectored(&self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_write_vectored(&self) -> bool {
        todo!()
    }

    pub fn flush(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn seek(&self, _pos: SeekFrom) -> io::Result<u64> {
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<File> {
        unsupported()
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        unsupported()
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder {}
    }

    pub fn mkdir(&self, _p: &Path) -> io::Result<()> {
        unsupported()
    }
}

pub fn readdir(_p: &Path) -> io::Result<ReadDir> {
    unsupported()
}

pub fn unlink(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn rename(_old: &Path, _new: &Path) -> io::Result<()> {
    unsupported()
}

pub fn set_perm(_p: &Path, perm: FilePermissions) -> io::Result<()> {
    match perm.0 {}
}

pub fn rmdir(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    unsupported()
}

pub fn try_exists(_path: &Path) -> io::Result<bool> {
    unsupported()
}

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn symlink(_original: &Path, _link: &Path) -> io::Result<()> {
    unsupported()
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported()
}

pub fn stat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn lstat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn copy(_from: &Path, _to: &Path) -> io::Result<u64> {
    unsupported()
}
