use crate::{
    domain::{post::field::Shortcode, Post, User},
    service::{
        action::post::{
            create_post_action, delete_post_action, get_all_posts_action, get_post_action,
            update_post_action,
        },
        object::post::{CreatePostObject, UpdatePostObject},
    },
    web::{
        form::{
            pagination::PaginationForm,
            post::{CreatePostForm, UpdatePostForm},
        },
        response::SuccessResponse,
        server::AppState,
        Result,
    },
};
use axum::{
    debug_handler,
    extract::{Host, Json, OriginalUri, Path, Query, State},
    http::{header, status::StatusCode, HeaderName},
    routing::get,
    Router,
};

async fn get_all_posts(
    Query(pagination): Query<PaginationForm>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<SuccessResponse<Vec<Post>>>)> {
    let posts = get_all_posts_action(pagination, state.database.get_pool()).await?;
    Ok((StatusCode::OK, Json(SuccessResponse::new(posts))))
}

#[debug_handler]
async fn get_post(
    Path(shortcode): Path<Shortcode>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<SuccessResponse<Post>>)> {
    let post = get_post_action(shortcode.into_inner().as_str(), state.database.get_pool()).await?;
    Ok((StatusCode::OK, Json(SuccessResponse::new(post))))
}

#[debug_handler]
async fn create_post(
    user: User,
    State(state): State<AppState>,
    OriginalUri(original_uri): OriginalUri,
    host: Host,
    Json(form): Json<CreatePostForm>,
) -> Result<(
    StatusCode,
    [(HeaderName, String); 1],
    Json<SuccessResponse<Post>>,
)> {
    let form = CreatePostForm {
        author_id: user.user_id,
        ..form
    };
    let object: CreatePostObject = form.try_into()?;
    let post = create_post_action(object, state.database.get_pool()).await?;
    Ok((
        StatusCode::CREATED,
        [(
            header::LOCATION,
            format!("{}/{}/{}", host.0, original_uri.to_string(), post.shortcode),
        )],
        Json(SuccessResponse::new(post)),
    ))
}

#[debug_handler]
async fn update_post(
    user: User,
    Path(shortcode): Path<Shortcode>,
    State(state): State<AppState>,
    Json(form): Json<UpdatePostForm>,
) -> Result<(StatusCode, Json<SuccessResponse<Post>>)> {
    let form = UpdatePostForm {
        shortcode: Some(shortcode.into_inner()),
        ..form
    };
    let object: UpdatePostObject = form.try_into()?;
    let post = update_post_action(object, user, state.database.get_pool()).await?;
    Ok((StatusCode::OK, Json(SuccessResponse::new(post))))
}

#[debug_handler]
async fn delete_post(
    user: User,
    Path(shortcode): Path<Shortcode>,
    State(state): State<AppState>,
) -> Result<StatusCode> {
    let _ = delete_post_action(shortcode.as_str(), user, state.database.get_pool()).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_all_posts).post(create_post))
        .route(
            "/:shortcode",
            get(get_post).delete(delete_post).patch(update_post),
        )
        .with_state(state)
}
