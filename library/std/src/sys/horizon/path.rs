use crate::ffi::OsStr;
use crate::io;
use crate::path::{Path, PathBuf, Prefix};

#[inline]
pub fn is_sep_byte(b: u8) -> bool {
    b == b'/'
}

#[inline]
pub fn is_verbatim_sep(b: u8) -> bool {
    b == b'/'
}

// this is an internal type
pub enum ParsedPath<'a> {
    WithoutMountpoint(&'a str),
    WithMountpoint(&'a str, &'a str),
    // TODO: maybe we should prohibit relative paths altogether?
    Relative(&'a str),
}

pub const DEFAULT_MOUNTPOINT: &str = "sdmc";

impl ParsedPath<'_> {
    pub fn is_absolute(&self) -> bool {
        match self {
            ParsedPath::WithoutMountpoint(_) | ParsedPath::WithMountpoint(_, _) => true,
            ParsedPath::Relative(_) => false,
        }
    }

    pub fn mountpoint(&self) -> Option<&str> {
        match self {
            ParsedPath::WithMountpoint(mount, _) => Some(mount),
            ParsedPath::WithoutMountpoint(_) | ParsedPath::Relative(_) => None,
        }
    }

    pub fn device_path(&self) -> &str {
        match self {
            ParsedPath::WithMountpoint(_, path) => path,
            ParsedPath::WithoutMountpoint(path) => path,
            ParsedPath::Relative(_) => unreachable!("device_path is meaningless for a relative path")
        }
    }

    pub fn device_path_ipc(&self) -> Option<horizon_ipcdef::fssrv::Path> {
        horizon_ipcdef::fssrv::Path::try_new(self.device_path())
    }
}

fn parse_path_str(s: &str) -> ParsedPath<'_> {

    if s.starts_with('/') {
        return ParsedPath::WithoutMountpoint(s);
    }

    let first_component_len: usize = s.find('/').unwrap_or(s.len());
    let first_component: &str = &s[..first_component_len];

    if !first_component.ends_with(':') {
        return ParsedPath::Relative(first_component);
    }

    ParsedPath::WithMountpoint(first_component, &s[first_component_len..])
}

// this is an internal function
pub fn parse_path(p: &Path) -> ParsedPath<'_> {
    // horizon uses utf-8 for OsStr, so it's actually infalliable
    let s = p.to_str().unwrap();
    parse_path_str(s)
}

pub fn parse_prefix(_p: &OsStr) -> Option<Prefix<'_>> {
    None
}

pub const MAIN_SEP_STR: &str = "/";
pub const MAIN_SEP: char = '/';

pub(crate) fn absolute(path: &Path) -> io::Result<PathBuf> {
    match parse_path(path) {
        ParsedPath::WithoutMountpoint(_) | ParsedPath::WithMountpoint(_, _)
            => Ok(path.to_path_buf()),
        ParsedPath::Relative(_) => todo!("Resolve relative path"),
    }
}
