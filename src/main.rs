use iced::{Application, Settings, Result};
use rustic_rover2::RusticRover2;

fn main()->Result
{
    let mut settings = Settings::default();
    settings.default_font = iced::font::Font::MONOSPACE;
    RusticRover2::run(settings)
}