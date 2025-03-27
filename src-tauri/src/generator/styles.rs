use std::{collections::HashMap, sync::LazyLock};

use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Style {
    ThinBorder,
    ThinBorderTextCenter,
    ThinBorderTextWrap,
    TextCenterColorRed,
    TextBoldCentered,
    BackgroundSilver,
    BackgroundBlack,
    BackgroundGreen,
    BackgroundRed
}

pub struct Styles {
    pub data: HashMap<Style, Format>
}

impl Styles {
    pub fn get(&self, style: &Style) -> Result<&Format, crate::error::Error> {
        Ok(self.data.get(style).ok_or(crate::error::Error::Other("Incorrect style".to_string()))?)
    }
}

pub const STYLES: LazyLock<Styles> = LazyLock::new(|| {
    Styles {
        data: HashMap::from([
            (Style::ThinBorder, Format::new().set_border(FormatBorder::Thin)),
            (Style::ThinBorderTextCenter, Format::new().set_border(FormatBorder::Thin).set_align(FormatAlign::Center)),
            (Style::ThinBorderTextWrap, Format::new().set_border(FormatBorder::Thin).set_align(FormatAlign::Center).set_text_wrap()),
            (Style::TextCenterColorRed, Format::new().set_align(FormatAlign::VerticalCenter).set_align(FormatAlign::Center).set_background_color(Color::Red)),
            (Style::TextBoldCentered, Format::new()
                .set_align(FormatAlign::Center).set_align(FormatAlign::CenterAcross).set_bold().set_text_wrap().set_border(FormatBorder::Thin)),
            (Style::BackgroundSilver, Format::new().set_border(FormatBorder::Thin).set_background_color(Color::Silver)),
            (Style::BackgroundBlack, Format::new().set_border(FormatBorder::Thin).set_background_color(Color::Black)),
            (Style::BackgroundGreen, Format::new().set_border(FormatBorder::Thin).set_background_color(Color::Green).set_text_wrap().set_align(FormatAlign::Center)),
            (Style::BackgroundRed, Format::new().set_border(FormatBorder::Thin).set_background_color(Color::Red).set_text_wrap().set_align(FormatAlign::Center))
        ])  
    }
});