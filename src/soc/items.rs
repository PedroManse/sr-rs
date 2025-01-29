use super::*;
use enum_dispatch::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "item_type")]
pub enum ItemType {
    #[sqlx(rename = "post")]
    DBPost,
    #[sqlx(rename = "comment")]
    DBComment,
    #[sqlx(rename = "gallery")]
    DBGalleryPost,
    #[sqlx(rename = "article")]
    DBArticlePost,
    #[sqlx(rename = "repost")]
    DBRepost,
}

#[enum_dispatch] /*impl Item*/
pub enum AnyItem {
    Post(Post),
    Comment(Comment),
    GalleryPost(GalleryPost),
    ArticlePost(ArticlePost),
    Repost(Repost),
}

impl ItemInfo for AnyItem {
    fn get_link(&self) -> String {
        use AnyItem::*;
        match &self {
            Post(p) => p.get_link(),
            Comment(p) => p.get_link(),
            GalleryPost(p) => p.get_link(),
            ArticlePost(p) => p.get_link(),
            Repost(p) => p.get_link(),
        }
    }
    fn get_body(&self) -> String {
        use AnyItem::*;
        match &self {
            Post(p) => p.get_body(),
            Comment(p) => p.get_body(),
            GalleryPost(p) => p.get_body(),
            ArticlePost(p) => p.get_body(),
            Repost(p) => p.get_body(),
        }
    }
}

macro_rules! defItem {
    ($name:ident From<$db:ty> { $( $t:tt )* } )=> {
        pub struct $name {
            pub posted: chrono::DateTime<chrono::Utc>,
            pub poster: UserId,
            pub id: ItemId,
            $( $t )*
        }
        impl Item for $name {
            type DBType = $db;
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

defItem!(Post From<DBPost> {
    pub text: String,
});

impl ItemInfo for Post {
    fn get_link(&self) -> String {
        format!("https://manse.dev.br/soc/post/{}", self.id.0)
    }
    fn get_body(&self) -> String {
        self.text.clone()
    }
}

defItem!(Comment From<DBComment> {
    pub parent: ItemId,
    pub text: String,
});

impl ItemInfo for Comment {
    fn get_link(&self) -> String {
        format!("https://manse.dev.br/soc/comment/{}", self.id.0)
    }
    fn get_body(&self) -> String {
        self.text.clone()
    }
}

defItem!(GalleryPost From<DBGalleryPost> {
    pub title: String,
    pub images: Vec<Url>,
});

impl ItemInfo for GalleryPost {
    fn get_link(&self) -> String {
        format!("https://manse.dev.br/soc/gallery/{}", self.id.0)
    }
    fn get_body(&self) -> String {
        let mut st = String::new();
        for img in &self.images {
            st += img.as_str();
            st += "\n";
        }
        st
    }
}

defItem!(ArticlePost From<DBArticlePost> {
    pub title: String,
    pub article: Url,
});

impl ItemInfo for ArticlePost {
    fn get_link(&self) -> String {
        format!("https://manse.dev.br/soc/article/{}", self.id.0)
    }
    fn get_body(&self) -> String {
        todo!()
    }
}

defItem!(Repost From<DBRepost> {
    pub comment: String,
    pub original: ItemId,
});

impl ItemInfo for Repost {
    fn get_link(&self) -> String {
        format!("https://manse.dev.br/soc/repost/{}", self.id.0)
    }
    fn get_body(&self) -> String {
        todo!()
    }
}
