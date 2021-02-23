use log::{error, info};
use roux::subreddit::responses::submissions::Submissions;
use roux::util::{FeedOption, TimePeriod};
use roux::Subreddit;

pub async fn get_top_image_from_subreddit(subreddit: &str, time_period: TimePeriod) -> String {
    // Get top 300 top posts from the last week
    let subreddit = Subreddit::new(subreddit);
    let posts;
    match subreddit
        .top(45, Some(FeedOption::new().period(time_period)))
        .await
    {
        Ok(p) => posts = p,
        Err(_) => {
            error!("Could not get posts from reddit");
            return "Idk man".to_string();
        }
    }

    match find_image_in_posts(&posts) {
        Some(s) => s.to_owned(),
        None => "idk, man".to_string(),
    }
}

fn find_image_in_posts(posts: &Submissions) -> Option<String> {
    let mut tries: u8 = 30;
    let mut post = &posts.data.children[rand::random::<usize>() % posts.data.children.len()].data;
    while !post.url.is_none() && !is_url_image(post.url.to_owned().unwrap()) && tries > 0 {
        post = &posts.data.children[rand::random::<usize>() % posts.data.children.len()].data;
        tries -= 1;
    }

    post.url.to_owned()
}

fn is_url_image(url: String) -> bool {
    match &url[url.len() - 3..url.len()] {
        ".gif" | ".jpg" | "jpeg" | ".png" => true,
        _ => false,
    }
}
