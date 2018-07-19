use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;

pub trait ToWide {
    fn to_wide_sized(&self) -> Vec<u16>;
    fn to_wide(&self) -> Vec<u16>;
}

impl<T> ToWide for T
where
    T: AsRef<OsStr>,
{
    fn to_wide_sized(&self) -> Vec<u16> {
        self.as_ref().encode_wide().collect()
    }
    fn to_wide(&self) -> Vec<u16> {
        self.as_ref().encode_wide().chain(Some(0)).collect()
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
