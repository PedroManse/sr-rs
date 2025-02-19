use super::*;
use models::*;

impl Comment {
    pub async fn get_parent(&self, pool: &PgPool) -> Result<AnyItem> {
        AnyItem::get(pool, self.parent).await
    }
}

impl Repost {
    pub async fn get_original(&self, pool: &PgPool) -> Result<AnyItem> {
        AnyItem::get(pool, self.original).await
    }
}


