#![allow(unused)]
use super::*;

#[enum_dispatch] /*impl PartialItem*/
pub enum AnyDBItem {
    Post(DBPost),
    Comment(DBComment),
    GalleryPost(DBGalleryPost),
    ArticlePost(DBArticlePost),
    Repost(DBRepost),
}

impl GetItemInfo for AnyDBItem {
    async fn get_info(common: CommonItem, pool: &PgPool) -> Result<Self> {
        use AnyDBItem::*;
        use ItemType as IT;
        match common.item_type {
            IT::DBPost => DBPost::get_info(common, pool).await.map(AnyDBItem::from),
            IT::DBComment => DBComment::get_info(common, pool).await.map(AnyDBItem::from),
            IT::DBGalleryPost => DBGalleryPost::get_info(common, pool)
                .await
                .map(AnyDBItem::from),
            IT::DBArticlePost => DBArticlePost::get_info(common, pool)
                .await
                .map(AnyDBItem::from),
            IT::DBRepost => DBRepost::get_info(common, pool).await.map(AnyDBItem::from),
        }
    }
}

impl IntoItem for AnyDBItem {
    type Target = AnyItem;
    async fn into_item(self, pool: &PgPool) -> Result<Self::Target> {
        use AnyDBItem::*;
        Ok(match self {
            Post(p) => p.into_item(pool).await?.into(),
            Comment(p) => p.into_item(pool).await?.into(),
            GalleryPost(p) => p.into_item(pool).await?.into(),
            ArticlePost(p) => p.into_item(pool).await?.into(),
            Repost(p) => p.into_item(pool).await?.into(),
        })
    }
}

macro_rules! defDBItem {
    ($name:ident { $( $t:tt )* } )=> {
        pub struct $name {
            pub posted: chrono::DateTime<chrono::Utc>,
            pub poster: UserId,
            pub id: ItemId,
            $( $t )*
        }
        impl DBItem for $name {
            const ITEM_TYPE: ItemType = ItemType::$name;
        }
        impl PartialItem for $name {
            fn get_id(&self) -> ItemId {
                self.id
            }
            fn get_poster(&self) -> UserId {
                self.poster
            }
            fn get_publish_date(&self) -> &chrono::DateTime<chrono::Utc> {
                &self.posted
            }
        }
    };
}

defDBItem!(DBPost {
    pub text: String,
});

impl GetItemInfo for DBPost {
    async fn get_info(common: CommonItem, pool: &PgPool) -> Result<Self> {
        let info = sqlx::query!(r#"SELECT text FROM soc.post WHERE item_id=$1"#, common.id.0,)
            .fetch_one(pool)
            .await?;
        Ok(Self {
            id: common.id,
            poster: common.poster,
            posted: common.posted_time,
            text: info.text,
        })
    }
}

impl IntoItem for DBPost {
    type Target = Post;
    async fn into_item(self, pool: &PgPool) -> Result<Self::Target> {
        Ok(Post {
            posted: self.posted,
            poster: self.poster,
            id: self.id,
            text: self.text,
        })
    }
}

defDBItem!(DBComment {
    pub parent: ItemId,
    pub text: String,
});

impl GetItemInfo for DBComment {
    async fn get_info(common: CommonItem, pool: &PgPool) -> Result<Self> {
        let info = sqlx::query!(
            r#"SELECT text, parent FROM soc.comment WHERE item_id=$1"#,
            common.id.0
        )
        .fetch_one(pool)
        .await?;
        Ok(DBComment {
            poster: common.poster,
            posted: common.posted_time,
            id: common.id,
            text: info.text,
            parent: ItemId(info.parent),
        })
    }
}

impl IntoItem for DBComment {
    type Target = Comment;
    async fn into_item(self, _: &PgPool) -> Result<Self::Target> {
        Ok(Comment {
            poster: self.poster,
            posted: self.posted,
            id: self.id,
            text: self.text,
            parent: self.parent,
        })
    }
}

// id -> gallery's photos
defDBItem!(DBGalleryPost {
    pub title: String,
});

impl GetItemInfo for DBGalleryPost {
    async fn get_info(common: CommonItem, pool: &PgPool) -> Result<Self> {
        let info = sqlx::query!(
            r#"SELECT title FROM soc.gallery WHERE item_id=$1"#,
            common.id.0,
        )
        .fetch_one(pool)
        .await?;
        Ok(DBGalleryPost {
            title: info.title,
            posted: common.posted_time,
            poster: common.poster,
            id: common.id,
        })
    }
}

impl IntoItem for DBGalleryPost {
    type Target = GalleryPost;
    async fn into_item(self, pool: &PgPool) -> Result<Self::Target> {
        let photos = sqlx::query!(
            r#"SELECT url FROM soc.gallery_photo WHERE gallery=$1"#,
            self.id.0
        )
        .fetch_all(pool)
        .await?;
        let urls: Vec<_> = photos
            .into_iter()
            .map(|r| Url::parse(&r.url).map_err(SocError::from))
            .collect::<Result<Vec<Url>>>()?;
        Ok(GalleryPost {
            poster: self.poster,
            posted: self.posted,
            id: self.id,
            title: self.title,
            images: urls,
        })
    }
}

defDBItem!(DBArticlePost {
    pub title: String,
    pub article: String, // -> URL
});

impl GetItemInfo for DBArticlePost {
    async fn get_info(common: CommonItem, pool: &PgPool) -> Result<Self> {
        let info = sqlx::query!(
            r#"SELECT title, url FROM soc.article WHERE item_id=$1"#,
            common.id.0,
        )
        .fetch_one(pool)
        .await?;
        Ok(DBArticlePost {
            posted: common.posted_time,
            poster: common.poster,
            id: common.id,
            title: info.title,
            article: info.url,
        })
    }
}

impl IntoItem for DBArticlePost {
    type Target = ArticlePost;
    async fn into_item(self, _pool: &PgPool) -> Result<Self::Target> {
        Ok(ArticlePost {
            posted: self.posted,
            poster: self.poster,
            id: self.id,
            title: self.title,
            article: Url::parse(&self.article)?,
        })
    }
}

defDBItem!(DBRepost {
    pub comment: String,
    pub original: ItemId,
});

impl GetItemInfo for DBRepost {
    async fn get_info(common: CommonItem, pool: &PgPool) -> Result<Self> {
        let info = sqlx::query!(
            r#"SELECT comment, original FROM soc.repost WHERE item_id=$1"#,
            common.id.0,
        )
        .fetch_one(pool)
        .await?;
        Ok(DBRepost {
            posted: common.posted_time,
            poster: common.poster,
            id: common.id,
            comment: info.comment,
            original: ItemId(info.original),
        })
    }
}

impl IntoItem for DBRepost {
    type Target = Repost;
    async fn into_item(self, _pool: &PgPool) -> Result<Self::Target> {
        Ok(Repost {
            posted: self.posted,
            poster: self.poster,
            id: self.id,
            comment: self.comment,
            original: self.original,
        })
    }
}
