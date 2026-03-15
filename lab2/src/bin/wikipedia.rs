use reqwest;
use reqwest::IntoUrl;
use std::io::Write;

fn needs_rebuild(out_path: &dyn AsRef<std::path::Path>, in_paths: &[&dyn AsRef<std::path::Path>]) -> bool {
    if !out_path.as_ref().exists() {
        true
    } else {
        let out_modified_time = out_path.as_ref().metadata().unwrap().modified().unwrap();
        in_paths.iter().map(|p| p.as_ref().metadata().unwrap()).any(|m| m.modified().unwrap() > out_modified_time)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_agent = "MyRustWikiBot/1.0 (contact: your-email@example.com)";
    let client = reqwest::Client::builder().user_agent(user_agent).build()?;

    let url = "https://en.wikipedia.org/wiki/Special:Random";
    let build_dir = std::path::Path::new("build");
    if needs_rebuild(&build_dir, &[]) {
        std::fs::create_dir(&build_dir).unwrap();
    }
    let mut pages_path = build_dir.join("wikipedia");
    let _ = std::fs::remove_dir_all(&pages_path);
    std::fs::create_dir(&pages_path).unwrap();

    for i in (0..1000) {
        let page = client.get(url).send().await.unwrap().text().await.unwrap();
        let page_path = pages_path.join(i.to_string()).with_extension("html");
        let mut file = std::fs::File::create(&page_path).unwrap();
        file.write_all(page.as_bytes()).unwrap();
        println!("{i} downloaded");
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
    Ok(())
}
