use bing_dict::translate;

#[tokio::main]
async fn main() {
    let hello = translate("hello").await.unwrap().unwrap();
    println!("{hello}");
}
