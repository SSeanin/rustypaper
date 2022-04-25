pub struct CreatePostDto {
    pub(in crate::data) post_id: String,
    pub(in crate::data) title: String,
    pub(in crate::data) content: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) is_published: String,
    pub(in crate::data) created_at: String,
    pub(in crate::data) updated_at: String,
}

pub struct GetPostDto {
    pub(in crate::data) shortcode: String,
}

pub struct UpdatePostDto {
    pub(in crate::data) shortcode: String,
    pub(in crate::data) title: String,
    pub(in crate::data) content: String,
    pub(in crate::data) is_published: String,
    pub(in crate::data) updated_at: String,
}

pub struct DeletePostDto {
    pub(in crate::data) shortcode: String,
}
