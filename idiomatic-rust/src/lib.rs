use std::fmt::format;
use derive_new::new;
use rand::prelude::*;

#[derive(Debug)]
pub enum Role{
    Guest,
    Viewer,
    Creator,
    Admin
}

#[derive(Debug)]
pub struct User{
    id: u32,
    pub username: String,
    pub role: Role,
}

impl User {
    pub fn new(username: String) -> Result<Self, String> {
        if username == "testuser123" {
            return Err("Username already exists".to_owned());
        }
        Ok(Self {
            id: thread_rng().gen_range(0..9999999),
            username,
            role: Role::Creator,
        })
    }
}

impl Default for User {
    fn default() -> Self {
        let id = thread_rng().gen_range(0..9999999);
        Self {
            id,
            username: format!("guest{id}"),
            role: Role::Guest
        }
    }
}

//all types inside the struct implement the default trait, thus we can derive default for the post struct
#[derive(Debug, Default, new)]
pub struct Post {
    content: String,
    #[new(value = "vec![\"rusty\".to_owned()]")]
    tags: Vec<String>,
    #[new(default)]
    likes: u32,
}