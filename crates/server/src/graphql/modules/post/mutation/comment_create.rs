use playa::post::service::CreateCommentDto;
use pxid::graphql::Pxid;
use serde::{Deserialize, Serialize};

use crate::context::SharedContext;
use crate::graphql::modules::post::types::{Post, PostError};
use crate::services::auth::Token;
use async_graphql::{Context, InputObject, Result, SimpleObject};

#[derive(Debug, InputObject)]
pub struct CommentCreateInput {
    pub content: String,
    pub parent_id: Pxid,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct CommentCreate {
    comment: Option<Post>,
    error: Option<PostError>,
}

impl CommentCreate {
    pub async fn exec(ctx: &Context<'_>, input: CommentCreateInput) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let token = ctx.data_unchecked::<Token>();
        let claims = context.services.auth.verify_token(token)?;
        let dto = CreateCommentDto {
            author_id: claims.uid,
            parent_id: input.parent_id.into_inner(),
            content: input.content,
        };

        match context.services.post.create_comment(dto).await {
            Ok(post) => Ok(Self {
                comment: Some(Post::from(post)),
                error: None,
            }),
            Err(err) => Ok(Self {
                comment: None,
                error: Some(PostError {
                    code: crate::graphql::modules::post::types::PostErrorCode::Unknown,
                    message: format!("An error ocurred: {err}"),
                }),
            }),
        }
    }
}
