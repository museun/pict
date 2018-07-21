use std::fs;
use std::io::Write;

use toml;

const CONFIG_FILE: &str = "pict.toml";

lazy_static! {
    static ref CONFIG: Config = Config::load();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub position: Position,
    pub size: Size,
    pub filelist: FileList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Size {
    pub w: i32,
    pub h: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FileList {
    pub snap: bool,
}

impl Config {
    pub fn get<'a>() -> &'a Self {
        ::lazy_static::initialize(&CONFIG);
        debug!("got cached config");
        &*CONFIG
    }

    pub fn load() -> Self {
        fn try_load_config() -> Option<Config> {
            let s = fs::read_to_string(CONFIG_FILE).ok()?;
            toml::from_str(&s).ok()
        }

        let conf = try_load_config()
            .or_else(|| {
                info!("creating default config");
                Some(Self {
                    position: Position { x: 0, y: 0 },
                    size: Size { w: 400, h: 200 },
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
