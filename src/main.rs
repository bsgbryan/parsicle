use readie_motherbrain::parse::parse;

#[tokio::main]
async fn main() {
    // let url = "https://time.com/6971144/campus-protests-professors-essay/";
    let url = "https://www.pcmag.com/reviews/samsung-galaxy-book4-ultra";
    let parsed = parse(url).await.ok().unwrap();

    println!("{:#?}", parsed);

    // let blocking_task = tokio::task::spawn_blocking(|| {
    //     let _ = nlp(parsed.content);
    // });

    // blocking_task.await.unwrap();
}
