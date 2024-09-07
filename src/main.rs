use iced::{Application, Settings, Result};
use rustic_rover2::RusticRover2;

fn main()->Result
{
    RusticRover2::run(Settings::default())
}