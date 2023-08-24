use async_std::task;

async fn say_hello() {
    println!("Hello catpic!");
}

fn main() {
    task::block_on(say_hello())
}