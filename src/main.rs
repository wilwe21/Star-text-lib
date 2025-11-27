mod text;
mod ansi_comapct;
mod future;

fn main() {
    let mut jp = text::Text::new("test");
    println!("{:#}", jp);
}
