use blog::Post;

fn main(){
  let must post = Post::new();
  post.add_text("I ate a salad for lunch today");
  post.request_review();
  post.approve();
  assert_eq!("I ate a salad for lunch today", post.content());
}