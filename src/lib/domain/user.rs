use crate::data::database::AppDatabase;
use crate::domain::token_generator::AccessTokenClaims;
use crate::domain::{DomainError, TokenGenerator};
use crate::service::action::user::get_user_by_id_action;
use field::{CreatedAt, Email, FirstName, LastName, Password, UpdatedAt, UserId};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, State};
use serde::{Deserialize, Serialize};

pub mod field;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip)]
    pub user_id: UserId,
    pub email: Email,
    pub first_name: FirstName,
    pub last_name: LastName,
    #[serde(skip)]
    pub password: Password,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = DomainError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let access_token = match request.cookies().get_private("Authorization") {
            None => {
                return Outcome::Failure((Status::Unauthorized, DomainError::InvalidToken));
            }
            Some(access_token_cookie) => access_token_cookie.value().to_owned(),
        };

        let token_generator = match request.rocket().state::<TokenGenerator>() {
            None => {
                return Outcome::Failure((Status::InternalServerError, DomainError::TokenGenerator))
            }
            Some(token_generator) => token_generator,
        };

        let claims = match token_generator.verify_token::<AccessTokenClaims>(access_token.as_str())
        {
            Err(e) => {
                return Outcome::Failure((Status::Unauthorized, e));
            }
            Ok(claims) => claims,
        };

        let database = match request.guard::<&State<AppDatabase>>().await {
            Outcome::Success(db) => db,
            _ => return Outcome::Failure((Status::InternalServerError, DomainError::Database)),
        };

        match get_user_by_id_action(claims.sub, database.get_pool()).await {
            Ok(user) => Outcome::Success(user),
            Err(..) => Outcome::Failure((Status::Unauthorized, DomainError::InvalidToken)),
        }
    }
}
