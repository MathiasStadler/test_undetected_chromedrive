:dep undetected-chromedriver = "0.1.2"
:dep tokio
:dep thirtyfour 

use undetected_chromedriver::chrome;
use tokio;

#[tokio::main]
async fn call_finviz_website() -> Result<(), Box<dyn std::error::Error>> {
    let driver = chrome().await?;

    driver.goto("https://www.finviz.com/").await?;

    let title = driver.title().await?;
    println!("Title: {}", title);

    driver.quit().await?;

    Ok(())
}

call_finviz_website();
