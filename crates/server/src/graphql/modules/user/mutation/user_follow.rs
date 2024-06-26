use async_graphql::{Context, Result, SimpleObject};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use townhall::auth::service::Token;
use townhall::user::service::FollowPeers;

use crate::context::SharedContext;
use crate::graphql::modules::user::types::{UserError, UserErrorCode};

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct UserFollow {
    error: Option<UserError>,
}

impl UserFollow {
    pub async fn exec(ctx: &Context<'_>, followee_id: Pxid) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let user_id = ctx.data_unchecked::<Token>().user_id();

        match context
            .services
            .user
            .follow(FollowPeers {
                followee_id,
                follower_id: user_id,
            })
            .await
        {
            Ok(_) => Ok(Self { error: None }),
            Err(err) => Ok(Self {
                error: Some(UserError {
                    code: UserErrorCode::Internal,
                    message: format!("An error ocurred: {err}"),
                }),
            }),
        }
    }
}
