use downloader::config::parse_config;

use downloader::tui::TuiApp;

async fn main_async() {
    let config = parse_config().unwrap();

    let mut app = TuiApp::new(config);
    let mut terminal = downloader::tui::init().unwrap();
    app.run(&mut terminal).await.unwrap();
    downloader::tui::restore().unwrap();
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(main_async());
}
