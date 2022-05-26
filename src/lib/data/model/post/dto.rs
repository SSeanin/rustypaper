use crate::data::Id;
use crate::domain::post::field::Shortcode;
use crate::domain::AppDatetime;
use crate::service::object::post::{
    CreatePostObject, DeletePostObject, GetAllPostsObject, GetPostObject, UpdatePostObject,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct CreatePostDto {
    pub(in crate::data) post_id: Uuid,
    pub(in crate::data) title: String,
    pub(in crate::data) content: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) is_published: bool,
    pub(in crate::data) updated_at: DateTime<Utc>,
}

impl From<CreatePostObject> for CreatePostDto {
    fn from(object: CreatePostObject) -> Self {
        Self {
            post_id: Id::new().into_inner(),
            title: object.title.into_inner(),
            content: object.content.into_inner(),
            shortcode: Shortcode::new().into_inner(),
            is_published: object.is_published.into_inner(),
            updated_at: AppDatetime::now().into_inner(),
        }
    }
}

pub struct GetAllPostsDto {
    pub(in crate::data) skip: i64,
    pub(in crate::data) limit: i64,
}

impl From<GetAllPostsObject> for GetAllPostsDto {
    fn from(object: GetAllPostsObject) -> Self {
        Self {
            skip: object.skip.unwrap_or(0),
            limit: object.limit.unwrap_or(50),
        }
    }
}

pub struct GetPostDto {
    pub(in crate::data) shortcode: String,
}

impl From<String> for GetPostDto {
    fn from(shortcode: String) -> Self {
        Self { shortcode }
    }
}

impl From<GetPostObject> for GetPostDto {
    fn from(object: GetPostObject) -> Self {
        Self {
            shortcode: object.shortcode.into_inner(),
        }
    }
}

pub struct UpdatePostDto {
    pub(in crate::data) shortcode: String,
    pub(in crate::data) title: String,
    pub(in crate::data) content: String,
    pub(in crate::data) is_published: bool,
    pub(in crate::data) updated_at: DateTime<Utc>,
}

impl From<UpdatePostObject> for UpdatePostDto {
    fn from(object: UpdatePostObject) -> Self {
        Self {
            shortcode: object.shortcode.into_inner(),
            title: object.title.into_inner(),
            content: object.content.into_inner(),
            is_published: object.is_published.into_inner(),
            updated_at: AppDatetime::now().into_inner(),
        }
    }
}

pub struct DeletePostDto {
    pub(in crate::data) shortcode: String,
}

impl From<DeletePostObject> for DeletePostDto {
    fn from(object: DeletePostObject) -> Self {
        Self {
            shortcode: object.shortcode.into_inner(),
        }
    }
}
