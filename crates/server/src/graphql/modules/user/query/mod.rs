mod users;

use async_graphql::{Context, Object, Result};
use pxid::Pxid;

use self::users::{UserFilterInput, Users, UsersConnection};

#[derive(Debug, Default)]
pub struct UserQueryRoot;

#[Object]
impl UserQueryRoot {
    #[allow(clippy::too_many_arguments)]
    async fn user(
        &self,
        ctx: &Context<'_>,
        after: Option<Pxid>,
        before: Option<Pxid>,
        first: Option<i32>,
        last: Option<i32>,
        filter: Option<UserFilterInput>,
    ) -> Result<UsersConnection> {
        Users::exec(ctx, after, before, first, last, filter).await
    }
}
