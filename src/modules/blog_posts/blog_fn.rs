use leptos::*; // Assuming you're using the Leptos framework
use crate::modules::blog_posts::blog_compo::Post; // Assuming `Post` is defined in your main or another module
use crate::modules::blog_posts::blog_comms::*;



// Create post signals
pub fn create_post_signal() -> (ReadSignal<Vec<Post>>, WriteSignal<Vec<Post>>) {
    let (posts, set_posts) = create_signal(vec![]);

    spawn_local(async move {
        match get_posts_from_api().await {
            Ok(fetched_posts) => {
                set_posts(fetched_posts);
            }
            Err(err) => {
                log::error!("Error fetching posts: {:?}", err);
            }
        }
    });

    (posts, set_posts)
}

// Fetch post vector
pub async fn get_post_vector(value: Vec<Post>) -> Vec<Post> {
    match get_posts_from_api().await {
        Ok(fetched_posts) => {
            //logging::log!("Fetched posts:\n{:#?}", fetched_posts);
            fetched_posts
        }
        Err(err) => {
            logging::log!("Error fetching posts: {:?}", err);
            value // Returns old vector in case of error
        }
    }
}


// Function to delete a post resource by ID
pub fn delete_post_resource_by_id(id: i32, set_posts: WriteSignal<Vec<Post>>) {
    set_posts.update(|posts| {
        *posts = posts.iter()
                      .cloned() // Clone each post
                      .filter(|post| post.post_id != id) // Filter out the post with the given ID
                      .collect(); // Collect the remaining posts
    });
}
// Handle delete action


pub fn handle_post_delete(id: i32, set_posts: WriteSignal<Vec<Post>>) {
    // First delete from the resource synchronously
    delete_post_resource_by_id(id, set_posts);

    // Spawn the async API call in the background
        spawn_local(async move {
        logging::log!("Before calling delete_post_from_api for id: {}", id);
        let result = delete_post_from_api(id).await;
        logging::log!("After calling delete_post_from_api for id: {}", id);

        // Log the result (success or failure)
        match result {
            Ok(_) => {
                logging::log!("Successfully deleted post from API with id: {}", id);
            },
            Err(e) => {
                logging::log!("Failed to delete post from API: {}", e);
            }
        }
    });
}
