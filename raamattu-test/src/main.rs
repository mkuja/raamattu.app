mod test_front_page;
use std::time::Duration;

use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // Specify Chrome options
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    driver
        .set_implicit_wait_timeout(Duration::from_secs(10))
        .await?;

    driver.goto("http://192.168.1.80:8080").await?;

    test_front_page::test_front_page(&driver).await?;

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}
