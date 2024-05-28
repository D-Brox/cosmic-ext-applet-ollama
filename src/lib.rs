mod api;
mod chat;
mod localize;
mod models;
mod stream;
mod window;

use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use cosmic::widget;
use ron::de::from_reader;
use window::Window;

pub fn run() -> cosmic::iced::Result {
    localize::localize();

    cosmic::applet::run::<Window>(false, ())
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    avatar: PathBuf,
    keep_context: bool,
    model: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            avatar: PathBuf::new(),
            keep_context: true,
            model: String::new(),
        }
    }

    pub fn get_avatar_handle(&self) -> widget::image::Handle {
        if self.avatar.exists() {
            widget::image::Handle::from_path(&self.avatar)
        } else {
            let image: &[u8] = include_bytes!("../data/icons/avatar.png");
            widget::image::Handle::from_memory(image)
        }
    }

    pub fn set_avatar(&mut self, path: PathBuf) -> &mut Self {
        self.avatar = path;
        self
    }

    pub fn change_context(&mut self, context: bool) -> &mut Self {
        self.keep_context = context;
        self
    }

    pub fn set_model(&mut self, model: String) -> &mut Self {
        self.model = model;
        self
    }

    pub fn load() -> Settings {
        let data_path = dirs::data_dir()
            .expect("xdg-data not found")
            .join("cosmic-applet-ollama")
            .join("settings.ron");

        let settings: Settings = Settings::new();

        if let Ok(opened) = File::open(data_path) {
            let reader = BufReader::new(opened);
            let settings: Settings = from_reader(reader).expect("Cannot parse settings file");
            return settings
        }
        settings
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let data_path = dirs::data_dir()
            .expect("xdg-data not found")
            .join("cosmic-applet-ollama");

        fs::create_dir_all(&data_path)?;

        let pretty = PrettyConfig::default();
        let ron_string = to_string_pretty(self, pretty).unwrap();

        let mut file = File::create(data_path.join("settings.ron"))?;
        file.write_all(ron_string.as_bytes())?;

        Ok(())
    }
}
