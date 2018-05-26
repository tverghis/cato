#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate redis;

use redis::PipelineCommands;

use std::process;

#[derive(Deserialize)]
struct RedditResponse {
    data: Data,
}

#[derive(Deserialize)]
struct Data {
    children: Vec<Post>,
}

#[derive(Deserialize)]
struct Post {
    data: PostData,
}

#[derive(Deserialize)]
struct PostData {
    id: String,
    is_self: bool,
    title: String,
}

fn main() {
    let self_posts = get_posts(20).unwrap_or_else(|e| {
        eprintln!("Unable to fetch posts: {:?}", e);
        process::exit(1);
    });

    let redis_client = redis::Client::open("redis://127.0.0.1").unwrap_or_else(|e| {
        eprintln!("Unable to create the store client: {:?}", e);
        process::exit(1);
    }); 

    let redis_conn = redis_client.get_connection().unwrap_or_else(|e| {
        eprintln!("Unable to get a connection to the store: {:?}", e);
        process::exit(1);
    });

    match add_posts_to_store(&redis_conn, self_posts) {
        Ok(n) => println!("Stored {} posts.", n),
        Err(e) => eprintln!("Could not store posts: {:?}", e),
    };
}

fn get_posts(num: usize) -> reqwest::Result<Vec<Post>> {
    let url = format!("https://www.reddit.com/r/dota2/new.json?sort=new&limit={}", num);
    let posts: RedditResponse = reqwest::get(&url)?.json()?;

    Ok(
        posts.data.children
            .into_iter()
            .filter(|p| p.data.is_self)
            .collect()
    )
}

fn add_posts_to_store(store: &redis::Connection, posts: Vec<Post>) -> redis::RedisResult<usize> {
    let mut pipe = redis::pipe();
    let num_posts = posts.len();

    for p in posts {
        pipe.set(p.data.id, p.data.title).ignore();
    }

    pipe.query(store)?;

    Ok(num_posts)
}
