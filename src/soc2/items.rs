#![allow(dead_code)]
use super::*;
use models::*;
use url::Url;

impl ItemCommonInfo {
    fn is_type(&self, it: ItemType) -> Result<()> {
        if self.item_type != it {
            Err(BackError::ItemType(WrongItemType {
                got: self.item_type.clone(),
                expected: it,
                item_id: self.id,
            }))
        } else {
            Ok(())
        }
    }
}

// INSERT item
impl AnyItem {
    async fn new_item(
        pool: &PgPool,
        poster: UserRef,
        item_type: ItemType,
    ) -> Result<ItemCommonInfo> {
        let item = sqlx::query!(
            r#"
INSERT INTO soc.item
    (poster, item_type)
VALUES
    ($1, $2)
RETURNING
    posted_time, id
"#,
            &poster.0,
            &item_type as &_
        )
        .fetch_one(pool)
        .await?;
        Ok(ItemCommonInfo {
            posted_time: item.posted_time,
            poster,
            id: ItemRef(item.id),
            item_type,
        })
    }

    pub(super) async fn new_post(pool: &PgPool, poster: UserRef, text: String) -> Result<Post> {
        let item = AnyItem::new_item(pool, poster, ItemType::Post).await?;
        sqlx::query!(
            r#"
INSERT INTO soc.post
    (item_id, text)
VALUES
    ($1, $2)
"#,
            item.id.0,
            &text
        )
        .execute(pool)
        .await?;
        Ok(Post {
            posted_time: item.posted_time,
            poster: item.poster,
            id: item.id,
            text,
        })
    }

    pub(super) async fn new_comment(
        pool: &PgPool,
        poster: UserRef,
        parent: ItemRef,
        text: String,
    ) -> Result<Comment> {
        let item = AnyItem::new_item(pool, poster, ItemType::Comment).await?;
        sqlx::query!(
            r#"
INSERT INTO soc.comment
    (item_id, parent, text)
VALUES
    ($1, $2, $3)
"#,
            item.id.0,
            parent.0,
            &text
        )
        .execute(pool)
        .await?;
        Ok(Comment {
            posted_time: item.posted_time,
            poster: item.poster,
            id: item.id,
            parent,
            text,
        })
    }

    pub(super) async fn new_gallery(
        pool: &PgPool,
        poster: UserRef,
        title: String,
        photos: Vec<Url>,
    ) -> Result<GalleryPost> {
        let item = AnyItem::new_item(pool, poster, ItemType::Gallery).await?;
        let gallery = sqlx::query!(
            r#"
INSERT INTO soc.gallery
    (item_id, title)
VALUES
    ($1, $2)
RETURNING
    id
"#,
            item.id.0,
            &title
        )
        .fetch_one(pool)
        .await?;
        let photo_urls: Vec<_> = photos.iter().map(Url::to_string).collect();
        sqlx::query!(
            r#"
INSERT INTO soc.gallery_photo
    (gallery, url)
SELECT $1, * FROM UNNEST($2::text[])
"#,
            gallery.id,
            &photo_urls
        )
        .execute(pool)
        .await?;
        Ok(GalleryPost {
            id: item.id,
            poster: item.poster,
            posted_time: item.posted_time,
            photos,
            title,
        })
    }

    pub(super) async fn new_article(
        pool: &PgPool,
        poster: UserRef,
        title: String,
        article: Url,
    ) -> Result<ArticlePost> {
        let item = AnyItem::new_item(pool, poster, ItemType::Article).await?;
        sqlx::query!(
            r#"
INSERT INTO soc.article
    (item_id, title, url)
VALUES
    ($1, $2, $3)
            "#,
            item.id.0,
            title,
            article.to_string()
        )
        .execute(pool)
        .await?;
        Ok(ArticlePost {
            posted_time: item.posted_time,
            poster: item.poster,
            id: item.id,
            title,
            article,
        })
    }

    pub(super) async fn new_repost(
        pool: &PgPool,
        poster: UserRef,
        comment: String,
        original: ItemRef,
    ) -> Result<Repost> {
        let item = AnyItem::new_item(pool, poster, ItemType::Repost).await?;
        sqlx::query!(
            r#"
INSERT INTO soc.repost
    (item_id, original, comment)
VALUES
    ($1, $2, $3)
"#,
            item.id.0,
            original.0,
            comment
        )
        .execute(pool)
        .await?;
        Ok(Repost {
            posted_time: item.posted_time,
            poster: item.poster,
            id: item.id,
            comment,
            original,
        })
    }
}

impl ItemInfo for Post {
    const ITEM_TYPE: ItemType = ItemType::Post;
    const TO_ANYITEM: fn(Self) -> AnyItem = AnyItem::Post;

    async fn _get_self(
        pool: &PgPool,
        ItemCommonInfo {
            posted_time,
            poster,
            id,
            item_type: _,
        }: ItemCommonInfo,
    ) -> Result<Post> {
        let p = sqlx::query!(
            r#"
SELECT text FROM soc.post
WHERE item_id=$1
"#,
            id.0
        )
        .fetch_one(pool)
        .await?;
        Ok(Post {
            posted_time,
            poster,
            id,
            text: p.text,
        })
    }
}

impl ItemInfo for Repost {
    const ITEM_TYPE: ItemType = ItemType::Repost;
    const TO_ANYITEM: fn(Self) -> AnyItem = AnyItem::Repost;

    async fn _get_self(
        pool: &PgPool,
        ItemCommonInfo {
            posted_time,
            poster,
            id,
            item_type: _,
        }: ItemCommonInfo,
    ) -> Result<Repost> {
        let p = sqlx::query!(
            r#"
SELECT original, comment FROM soc.repost
WHERE item_id=$1
"#,
            id.0
        )
        .fetch_one(pool)
        .await?;
        Ok(Repost {
            posted_time,
            poster,
            id,
            comment: p.comment,
            original: ItemRef(p.original),
        })
    }
}

impl ItemInfo for Comment {
    const ITEM_TYPE: ItemType = ItemType::Comment;
    const TO_ANYITEM: fn(Self) -> AnyItem = AnyItem::Comment;

    async fn _get_self(
        pool: &PgPool,
        ItemCommonInfo {
            posted_time,
            poster,
            id,
            item_type: _,
        }: ItemCommonInfo,
    ) -> Result<Comment> {
        let p = sqlx::query!(
            r#"
SELECT parent, text FROM soc.comment
WHERE item_id=$1
"#,
            id.0
        )
        .fetch_one(pool)
        .await?;
        Ok(Comment {
            posted_time,
            poster,
            id,
            parent: ItemRef(p.parent),
            text: p.text,
        })
    }
}

impl ItemInfo for GalleryPost {
    const ITEM_TYPE: ItemType = ItemType::Gallery;
    const TO_ANYITEM: fn(Self) -> AnyItem = AnyItem::GalleryPost;

    async fn _get_self(
        pool: &PgPool,
        ItemCommonInfo {
            posted_time,
            poster,
            id,
            item_type: _,
        }: ItemCommonInfo,
    ) -> Result<GalleryPost> {
        let p = sqlx::query!(
            r#"
SELECT id, title FROM soc.gallery
WHERE item_id=$1
"#,
            id.0
        )
        .fetch_one(pool)
        .await?;
        let gs: Result<Vec<_>> = sqlx::query!(
            r#"
SELECT url FROM soc.gallery_photo
WHERE gallery=$1
            "#,
            p.id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| r.url.parse().map_err(BackError::URL))
        .collect();
        Ok(GalleryPost {
            posted_time,
            poster,
            id,
            title: p.title,
            photos: gs?,
        })
    }
}

impl ItemInfo for ArticlePost {
    const ITEM_TYPE: ItemType = ItemType::Article;
    const TO_ANYITEM: fn(Self) -> AnyItem = AnyItem::ArticlePost;

    async fn _get_self(
        pool: &PgPool,
        ItemCommonInfo {
            posted_time,
            poster,
            id,
            item_type: _,
        }: ItemCommonInfo,
    ) -> Result<ArticlePost> {
        let p = sqlx::query!(
            r#"
SELECT title, url FROM soc.article
WHERE item_id=$1
"#,
            id.0
        )
        .fetch_one(pool)
        .await?;
        Ok(ArticlePost {
            posted_time,
            poster,
            id,
            title: p.title,
            article: p.url.parse()?,
        })
    }
}

impl AnyItem {
    async fn _get_item(pool: &PgPool, id: ItemRef) -> Result<ItemCommonInfo> {
        let item = sqlx::query!(
            r#"
SELECT posted_time, poster, item_type AS "item_type: ItemType" FROM soc.item
WHERE id=$1
"#,
            &id.0
        )
        .fetch_one(pool)
        .await?;
        Ok(ItemCommonInfo {
            posted_time: item.posted_time,
            poster: UserRef(item.poster),
            id,
            item_type: item.item_type,
        })
    }
    pub(super) async fn get(pool: &PgPool, id: ItemRef) -> Result<AnyItem> {
        let ic = AnyItem::_get_item(pool, id).await?;
        use ItemType as IT;
        Ok(match ic.item_type {
            IT::Post => Post::_get_self(pool, ic).await?.into(),
            IT::Repost => Repost::_get_self(pool, ic).await?.into(),
            IT::Comment => Comment::_get_self(pool, ic).await?.into(),
            IT::Gallery => GalleryPost::_get_self(pool, ic).await?.into(),
            IT::Article => ArticlePost::_get_self(pool, ic).await?.into(),
        })
    }
}

pub(super) trait ItemInfo
where
    Self: Sized,
{
    const ITEM_TYPE: ItemType;
    const TO_ANYITEM: fn(Self) -> AnyItem;
    async fn _get_self(pool: &PgPool, ic: ItemCommonInfo) -> Result<Self>;
    async fn get(pool: &PgPool, id: ItemRef) -> Result<Self> {
        let ic = AnyItem::_get_item(pool, id).await?;
        ic.is_type(Self::ITEM_TYPE)?;
        Self::_get_self(pool, ic).await
    }
}

impl<I: ItemInfo> From<I> for AnyItem {
    fn from(value: I) -> Self {
        I::TO_ANYITEM(value)
    }
}
