#![allow(dead_code)]
use std::fs;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::{mem, ptr};

use image;

use common::*;

pub struct ImageViewport {}

pub struct ImageView {
    hwnd: windef::HWND,
}

#[derive(Debug)]
pub enum ImageError {
    Loading(io::Error),
    Reading(io::Error),
    UnsupportedFormat(image::ImageError),
    Other(image::ImageError),
    NotFinished,
}

impl From<image::ImageError> for ImageError {
    fn from(e: image::ImageError) -> Self {
        ImageError::Other(e)
    }
}

struct InnerImage {
    frames: Vec<image::Frame>,
    animated: bool,
}

pub struct ImageBuffer<'a> {
    current: Option<&'a image::Frame>, // this should always be in a valid state. maybe?
    inner: InnerImage,
}

impl<'a> ImageBuffer<'a> {
    // TODO figure out which Path I should convert into
    pub fn new(path: &str) -> Result<Self, ImageError> {
        let mut rd = BufReader::new(fs::File::open(path).map_err(ImageError::Loading)?);

        let mut buf = [0; 17];
        rd.read_exact(&mut buf).map_err(ImageError::Reading)?;
        let _ = rd.seek(SeekFrom::Start(0)); // reset position

        let format = image::guess_format(&buf).map_err(ImageError::UnsupportedFormat)?;
    }

    pub fn width(&self) -> u32 {
        if let Some(ref img) = self.current {
            img.buffer().width()
        } else {
            0
        }
    }

    pub fn height(&self) -> u32 {
        if let Some(ref img) = self.current {
            img.buffer().height()
        } else {
            0
        }
    }

    pub fn is_animated(&self) -> bool {
        self.inner.animated || self.inner.frames.len() > 1
    }

    pub fn frames(&self) -> usize {
        self.inner.frames.len()
    }

    // these might need to be mut
    pub fn next(&self) {
        unimplemented!()
    }

    pub fn previous(&self) {
        unimplemented!()
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_load_image() {
        let images = ["test.gif", "test.png", "test.jpg"];
        for image in &images {
            match ImageBuffer::new(&image) {
                Ok(img) => eprintln!(
                    "{}:{},{} {}",
                    image,
                    img.width(),
                    img.height(),
                    img.is_animated()
                ),
                Err(err) => eprintln!("{}: {:?}", image, err),
            }
        }
    }
}

impl ImageView {
    pub fn new(parent: windef::HWND) -> Self {
        use winapi::um::commctrl::*;
        use winapi::um::winuser::{WS_CHILD, WS_VISIBLE};
        unsafe {
            let mut rect = mem::zeroed::<windef::RECT>();
            winuser::GetClientRect(parent, &mut rect);
        }

        unimplemented!()
    }
}

/* design

MainWindow -> ImageViewPort <--> ImageView -> ImageBuffer


mainwindow:
    the mainwindow handles the viewport, and draws other controls ontop of it

imageviewport <-- is this needed? the imageview could just keep the multiple sets of coords..
    the imageviewport will basically be a bounding box which helps translate coords

imageview
    the image will display the visible imagebuffer, and be responsible for the clipping 

imagebuffer
    the imagebuffer from the image crate

*/
