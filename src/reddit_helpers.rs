use roux::util::{FeedOption, TimePeriod};
use roux::Subreddit;
use log::{info, error};

pub async fn get_top_image_from_subreddit(subreddit: &str, time_period: TimePeriod) -> String{
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

    let post = &posts.data.children[rand::random::<usize>() % posts.data.children.len()].data;

    match &post.url {
        Some(s) => s.to_owned(),
        None => "idk, man".to_string(),
    }
}