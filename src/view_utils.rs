use iced;
use iced::widget::combo_box;

#[derive(Clone)]
pub struct ComboBox<T>
{
    pub all:iced::widget::combo_box::State<T>,
    pub selected:Option<T>,
}

impl<T: std::fmt::Display + std::clone::Clone> ComboBox<T> {
    pub fn new(all_list:Vec<T>)->ComboBox<T>
    {
        ComboBox { all: combo_box::State::new(all_list), selected: None }
    }
}

pub fn path_to_image(path:&str, size:u16)->iced::widget::Image<iced::widget::image::Handle>
{
    iced::widget::image::Image::new(iced::widget::image::Handle::from_path(path)).width(size).height(size)
}