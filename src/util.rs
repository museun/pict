use std::path::PathBuf;
use std::ptr;

use winapi::shared::minwindef;
use winapi::um::libloaderapi;

pub trait ToWide {
    fn to_wide(&self) -> Vec<u16>;
}

impl<T: AsRef<str>> ToWide for T {
    fn to_wide(&self) -> Vec<u16> {
        if self.as_ref().is_empty() {
            return "\0".encode_utf16().collect();
        }

        let mut s = self.as_ref().to_owned();
        let c = s.chars().rev().take(1).next().unwrap();
        if c != '\0' {
            s += "\0"
        };
        s.encode_utf16().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_non_null() {
        let s = "this is a test";
        let w = s.to_wide();
        assert_eq!(s.len() + 1, w.len());
    }

    #[test]
    fn test_string_empty_non_null() {
        let s = "";
        let w = s.to_wide();
        assert_eq!(s.len() + 1, w.len(), "{},{}", s.len(), w.len());
    }

    #[test]
    fn test_string_null() {
        let s = "this is a test\0";
        let w = s.to_wide();
        assert_eq!(s.len(), w.len());
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
