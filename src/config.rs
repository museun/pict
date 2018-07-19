use std::fs;
use std::io::Write;

use toml;

const CONFIG_FILE: &str = "pict.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub position: Position,
    pub size: Size,
    pub filelist: FileList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Size {
    pub w: f64,
    pub h: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FileList {
    pub snap: bool,
}

impl Config {
    pub fn load() -> Self {
        fn try_load_config() -> Option<Config> {
            let s = fs::read_to_string(CONFIG_FILE).ok()?;
            toml::from_str(&s).ok()
        }

        let conf = try_load_config()
            .or_else(|| {
                info!("creating default config");
                Some(Self {
                    position: Position { x: 0.0, y: 0.0 },
                    size: Size { w: 400.0, h: 200.0 },
                    filelist: FileList::default(),
                })
            })
            .expect("to get config");

        debug!("loaded config: {:?}", conf);
        conf
    }

    // App takes ownership, so we can't use drop.
    pub fn save(self) {
        debug!("saving config: {:?}", self);

        let s = toml::to_string_pretty(&self).expect("to serialize config");
        fs::File::create(CONFIG_FILE)
            .and_then(|mut f| writeln!(f, "{}", &s))
            .expect("to write config");
    }
}
