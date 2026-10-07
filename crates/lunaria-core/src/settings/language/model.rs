use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "zh-CN")]
    SimpleChinese,

    #[default]
    #[serde(rename = "en")]
    English,

    #[serde(rename = "ja")]
    Japanese,
}

impl Language {
    pub const ALL: &'static [Language] = &[Self::SimpleChinese, Self::English, Self::Japanese];

    pub const fn locale(self) -> &'static str {
        match self {
            Language::SimpleChinese => "zh-CN",
            Language::English => "en",
            Language::Japanese => "ja",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Language::SimpleChinese => "简体中文",
            Language::English => "English",
            Language::Japanese => "日本語",
        }
    }
}
