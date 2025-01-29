use super::*;

pub enum AnyItem {
    Post(Post),
    Comment(Comment),
    GalleryPost(GalleryPost),
    ArticlePost(ArticlePost),
    Repost(Repost),
}

macro_rules! defItem {
    ($name:ident { $( $t:tt )* } )=> {
        pub struct $name {
            pub posted: chrono::DateTime<chrono::Utc>,
            pub poster: UserId,
            pub id: ItemId,
            $( $t )*
        }
        impl PartialItem for $name {
            fn _get_id(&self) -> ItemId {
                self.id
            }
            fn _get_poster(&self) -> UserId {
                self.poster
            }
            fn _get_publish_date(&self) -> &chrono::DateTime<chrono::Utc> {
                &self.posted
            }
        }
    };
}

defItem!(Post {
    pub text: String,
});

impl Item for Post {
    fn get_link(&self) -> String {
        format!("https://manse.dev.br/soc/post/{}", self.id.0)
    }
    async fn get_body(&self, _: &PgPool) -> String {
        self.text.clone()
    }
}

defItem!(Comment {
    pub parent: ItemId,
    pub text: String,
});

impl Item for Comment {
    fn get_link(&self) -> String {
        format!("https://manse.dev.br/soc/comment/{}", self.id.0)
    }
    async fn get_body(&self, _: &PgPool) -> String {
        self.text.clone()
    }
}

defItem!(GalleryPost {
    pub title: String,
    pub images: Vec<ResourceId>,
});

impl Item for GalleryPost {
    fn get_link(&self) -> String {
        format!("https://manse.dev.br/soc/gallery/{}", self.id.0)
    }
    async fn get_body(&self, pool: &PgPool) -> String {
        let mut st = String::new();
        for img in &self.images {
            st += &img.get_link(pool).await.unwrap().as_str();
            st += "\n";
        }
        st
    }
}

defItem!(ArticlePost {
    pub title: String,
    pub article: ResourceId,
});

impl Item for ArticlePost {
    fn get_link(&self) -> String {
        format!("https://manse.dev.br/soc/article/{}", self.id.0)
    }
    async fn get_body(&self, pool: &PgPool) -> String {
        self.article.get_link(pool).await.unwrap().to_string()
    }
}

defItem!(Repost {
    pub comment: String,
    pub original: ItemId,
});


impl Item for Repost {
    fn get_link(&self) -> String {
        format!("https://manse.dev.br/soc/repost/{}", self.id.0)
    }
    async fn get_body(&self, _: &PgPool) -> String {
        format!("https://manse.dev.br/soc/post/{}", self.original.0)
    }
}

