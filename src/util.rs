use std::path::PathBuf;
use std::{mem, ptr};

use common::*;

pub trait ToWide {
    fn to_wide(&self) -> *const u16;
    fn to_wide_mut(&self) -> *mut u16;
}

impl<T: AsRef<str>> ToWide for T {
    fn to_wide(&self) -> *const u16 {
        if self.as_ref().is_empty() {
            let data = "\0".encode_utf16().collect::<Vec<_>>();
            let res = data.as_ptr();
            mem::forget(data);
            return res;
        }

        let mut s = self.as_ref().to_owned();
        let c = s.chars().rev().take(1).next().unwrap();
        if c != '\0' {
            s += "\0"
        };
        let data = s.encode_utf16().collect::<Vec<_>>();
        let res = data.as_ptr();
        mem::forget(data);
        res
    }

    fn to_wide_mut(&self) -> *mut u16 {
        if self.as_ref().is_empty() {
            let mut data = "\0".encode_utf16().collect::<Vec<_>>();
            let res = data.as_mut_ptr();
            mem::forget(data);
            return res;
        }

        let mut s = self.as_ref().to_owned();
        let c = s.chars().rev().take(1).next().unwrap();
        if c != '\0' {
            s += "\0"
        };
        let mut data = s.encode_utf16().collect::<Vec<_>>();
        let res = data.as_mut_ptr();
        mem::forget(data);
        res
    }
}

const SUFFIXES: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
pub fn humanize_size(sz: usize) -> String {
    if sz == 0 {
        return "0.00 B".into();
    }

    let powers = (0..SUFFIXES.len())
        .map(|x| 1024u64.pow(x as u32) as f64)
        .collect::<Vec<f64>>();

    let i = (sz as f64).log(1024.0).floor() as usize;
    let val = sz as f64 / powers[i];
    format!("{:.2} {}", (val * 1000.0).round() / 1000.0, SUFFIXES[i])
}

const ACCEPTED_EXTENSIONS: [&str; 4] = ["PNG", "JPG", "JPEG", "GIF"];
pub fn is_accepted_image_type<P: Into<PathBuf>>(path: P) -> bool {
    fn find(path: &PathBuf) -> Option<()> {
        let ext = path.extension()?.to_str()?.to_ascii_uppercase();
        for e in &ACCEPTED_EXTENSIONS {
            if *e == ext {
                return Some(());
            }
        }
        None
    }
    find(&path.into()).is_some()
}

pub fn hinstance() -> minwindef::HINSTANCE {
    unsafe { libloaderapi::GetModuleHandleW(ptr::null_mut()) }
}
