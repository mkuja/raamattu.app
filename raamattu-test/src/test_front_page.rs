use std::time::Duration;

use thirtyfour::{prelude::*, support::base64_decode};

pub async fn test_front_page(driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto("http://192.168.1.80:8080").await?;

    let language_select = driver.find(By::Id("lang")).await?;
    // Assert default language is English.
    assert_eq!(language_select.value().await?, Some("en".to_string()));

    let page_header = driver.find(By::XPath("//h1[1]")).await?;
    // Assert page header title is 'The Bible' on English setting.
    assert_eq!(page_header.text().await?, "The Bible".to_string());

    // Options of the language select.
    let finnish = language_select
        .find(By::XPath("//option[@value='fi']"))
        .await?;
    let english = language_select
        .find(By::XPath("//option[@value='en']"))
        .await?;
    finnish.click().await?;

    // Assert selecting Finnish changes header to read 'Raamattu'
    assert_eq!(page_header.text().await?, "Raamattu".to_string());

    let chapter_links = driver.find_all(By::XPath("//div/a")).await?;
    println!("chapter_links len: {}", chapter_links.len());
    for elem in &chapter_links {
        let book_name = elem.text().await?;
        elem.click().await?;
        let chapters_page_title = driver.find(By::XPath("//h2")).await?;
        assert_eq!(chapters_page_title.text().await?, book_name);

        driver.back().await?;
    }

    Ok(())
}
