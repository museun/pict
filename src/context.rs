use std::rc::Rc;

use winit;

use config::Config;
use filelist::FileList;
use mainwindow::MainWindow;

pub struct Context {
    pub mainwindow: Rc<MainWindow>,
    pub filelist: Rc<FileList>,
    list: Vec<String>,
    index: usize,
    snap: bool,
}

impl Context {
    pub fn new(events: &winit::EventsLoop, conf: &Config) -> Self {
        Self {
            mainwindow: Rc::new(MainWindow::new(&events, &conf)),
            filelist: Rc::new(FileList::new(&events)),
            list: vec![],
            index: 0,
            snap: conf.filelist.snap,
        }
    }

    pub fn get_len(&self) -> usize {
        let len = self.list.len();
        trace!("getting len: {}", len);
        len
    }

    pub fn get_index(&self) -> usize {
        trace!("getting index: {}", self.index);
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        trace!("setting index: {}", index);
        self.index = index
    }

    pub fn get_snap(&self) -> bool {
        trace!("getting snap: {}", self.snap);
        self.snap
    }

    pub fn set_snap(&mut self, snap: bool) {
        trace!("setting snap: {}", snap);
        self.snap = snap
    }

    pub fn clear_list(&mut self) {
        trace!("clearing list");
        self.list.clear();
        self.list.shrink_to_fit();
    }

    pub fn extend_list(&mut self, el: &[String]) {
        trace!("extending list");
        let v = el.to_vec();
        for el in &v {
            trace!("{}", el);
        }

        self.list.extend(v)
    }

    // TODO expose an iterator over the list?
}
