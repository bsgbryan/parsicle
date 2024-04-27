use readie_motherbrain::parse::parse;

#[tokio::main]
async fn main() {
    let url = "https://time.com/6971144/campus-protests-professors-essay/";
    let parsed = parse(url).await.ok().unwrap();

    println!("{:#?}", parsed.title);

    for c in parsed.content {
        println!("{}", c);
    }

    for l in parsed.links {
        println!("{}", l);
    }

    // let blocking_task = tokio::task::spawn_blocking(|| {
    //     let _ = nlp(parsed.content);
    // });

    // blocking_task.await.unwrap();
}
