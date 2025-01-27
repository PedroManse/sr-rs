use super::*;

pub enum AnyItem {
    Post(Post),
    Comment(Comment),
    ImagePost(ImagePost),
    ArticlePost(ArticlePost),
}

pub struct Post {
    pub posted: chrono::DateTime<chrono::Utc>,
    pub poster: UserId,
    pub id: ItemId,
    pub text: String,
}

impl Item for Post {
    fn get_id(&self) -> ItemId {
        self.id
    }
    fn get_poster(&self) -> UserId {
        self.poster
    }
    fn get_link(&self) -> String {
        format!("https://manse.dev.br/soc/post/{}", self.id.0)
    }
    fn get_body(&self) -> &str {
        &self.text
    }
    fn get_publish_date(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.posted
    }
}

pub struct Comment {
    pub posted: chrono::DateTime<chrono::Utc>,
    pub poster: UserId,
    pub id: ItemId,
    pub parent: Option<ItemId>,
    pub text: String,
}

impl Item for Comment {
    fn get_id(&self) -> ItemId {
        self.id
    }
    fn get_poster(&self) -> UserId {
        self.poster
    }
    fn get_link(&self) -> String {
        format!("https://manse.dev.br/soc/comment/{}", self.id.0)
    }
    fn get_publish_date(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.posted
    }
    fn get_body(&self) -> &str {
        &self.text
    }
}

pub struct ImagePost {
    pub posted: chrono::DateTime<chrono::Utc>,
    pub poster: UserId,
    pub id: ItemId,
    pub images: Vec<ResourceId>,
}

pub struct ArticlePost {

}

