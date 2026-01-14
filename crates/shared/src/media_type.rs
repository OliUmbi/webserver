#[derive(Debug)]
pub struct MediaType {
    parent_type: TopLevelMediaType,
    sub_type: String,
    suffix: Option<String>,
    parameter: Option<String>,
}

impl MediaType {
    pub fn as_str(&self) -> String {
        let mut media_type = format!("{}/{}", self.parent_type.as_str(), self.sub_type);

        if self.suffix.is_some() {
            media_type = format!("{}+{}", media_type, self.suffix.as_ref().unwrap())
        }

        if self.parameter.is_some() {
            media_type = format!("{}; {}", media_type, self.parameter.as_ref().unwrap())
        }

        media_type
    }

    pub fn from_str(s: &str) -> Option<Self> {
        todo!()
    }
}

#[derive(Debug)]
pub enum TopLevelMediaType {
    Application,
    Audio,
    Example,
    Font,
    Haptics,
    Image,
    Message,
    Model,
    Multipart,
    Text,
    Video,
}

impl TopLevelMediaType {
    pub fn as_str(&self) -> &str {
        match self {
            TopLevelMediaType::Application => "application",
            TopLevelMediaType::Audio => "audio",
            TopLevelMediaType::Example => "example",
            TopLevelMediaType::Font => "font",
            TopLevelMediaType::Haptics => "haptics",
            TopLevelMediaType::Image => "image",
            TopLevelMediaType::Message => "message",
            TopLevelMediaType::Model => "model",
            TopLevelMediaType::Multipart => "multipart",
            TopLevelMediaType::Text => "text",
            TopLevelMediaType::Video => "video",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "application" => Some(TopLevelMediaType::Application),
            "audio" => Some(TopLevelMediaType::Audio),
            "example" => Some(TopLevelMediaType::Example),
            "font" => Some(TopLevelMediaType::Font),
            "haptics" => Some(TopLevelMediaType::Haptics),
            "image" => Some(TopLevelMediaType::Image),
            "message" => Some(TopLevelMediaType::Message),
            "model" => Some(TopLevelMediaType::Model),
            "multipart" => Some(TopLevelMediaType::Multipart),
            "text" => Some(TopLevelMediaType::Text),
            "video" => Some(TopLevelMediaType::Video),
            _ => None,
        }
    }
}
