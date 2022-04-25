pub struct CreatePostDto {
    pub(self) post_id: String,
    pub(self) title: String,
    pub(self) content: String,
    pub(self) shortcode: String,
    pub(self) is_published: String,
    pub(self) created_at: String,
    pub(self) updated_at: String,
}

pub struct GetPostDto {
    pub(self) shortcode: String,
}

pub struct UpdatePostDto {
    pub(self) shortcode: String,
    pub(self) title: String,
    pub(self) content: String,
    pub(self) is_published: String,
    pub(self) updated_at: String,
}

pub struct DeletePostDto {
    pub(self) shortcode: String,
}
