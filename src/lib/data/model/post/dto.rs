pub struct CreatePostDto {
    pub(in crate::data) title: String,
    pub(in crate::data) content: String,
    pub(in crate::data) is_published: bool,
}

pub struct GetPostDto {
    pub(in crate::data) shortcode: String,
}

impl From<String> for GetPostDto {
    fn from(shortcode: String) -> Self {
        Self { shortcode }
    }
}

pub struct UpdatePostDto {
    pub(in crate::data) shortcode: String,
    pub(in crate::data) title: String,
    pub(in crate::data) content: String,
    pub(in crate::data) is_published: bool,
}

pub struct DeletePostDto {
    pub(in crate::data) shortcode: String,
}
