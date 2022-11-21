use crate::config::{Account, Proxy};
use ahash::HashMap;
use color_eyre::{Report, Result};
use dashmap::DashMap;
use fantoccini::wd::Capabilities;
use fantoccini::{ClientBuilder, Locator};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

pub const BROWSER_LOCATION: &str = "localhost:4444";
pub const TWITTER_LOGIN: &str = "https://twitter.com/i/flow/login";
pub const TWITTER_HOME: &str = "https://twitter.com/home";
pub const HOME: &str = "about:home";
pub const USERNAME_SELECT: Locator<'static> = Locator::Css("input[type=\"username\"]");
pub const PASSWORD_SELECT: Locator<'static> = Locator::Css("input[type=\"password\"]");
pub const NEXT_SELECT: Locator<'static> = Locator::Css("div[role=\"button\"][tabindex=\"0\"]");
pub const LOGIN_SELECT: Locator<'static> =
    Locator::Css("div[role=\"button\"][tabindex=\"0\"][testid=\"LoginForm_Login_Button\"]");
pub const MAX_WAIT_SECS: Duration = Duration::from_secs(10);
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";
#[instrument]
pub async fn log_in_using_browser(
    account: &Account,
    proxies: Arc<DashMap<String, Proxy>>,
) -> Result<HashMap<String, String>> {
    // connect to browser
    let mut browbuild = ClientBuilder::rustls();
    let mut caps = Capabilities::new();

    if let Some(proxy) = &account.assigned_proxy {
        let proxy = proxies.get(proxy).ok_or(Report::msg("no proxy"))?;
        let ip = &proxy.ip;
        caps.insert(
            "proxy".to_string(),
            json!({
                "proxyType": "pac",
                "proxyAutoconfigUrl": ip
            }),
        );
    }

    browbuild.capabilities(caps);

    let browser = browbuild.connect(BROWSER_LOCATION).await?;
    browser.set_ua(USER_AGENT);
    browser.delete_all_cookies().await?;
    browser.goto(TWITTER_LOGIN).await?;
    browser
        .wait()
        .at_most(MAX_WAIT_SECS)
        .for_url(TWITTER_LOGIN.parse().unwrap())
        .await?;

    // select username password clicknext login
    let username = browser.find(USERNAME_SELECT).await?;
    username.click().await?;
    username.send_keys(&account.username).await?;
    let next = browser
        .find_all(NEXT_SELECT)
        .await?
        .last()
        .ok_or(Report::new("no next button"))?;
    next.click().await?;
    let password = browser
        .wait()
        .at_most(MAX_WAIT_SECS)
        .for_element(PASSWORD_SELECT)
        .await?;
    password.click().await?;
    password.send_keys(&account.password).await?;
    sleep(Duration::from_millis(100)).await;
    let login = browser.find(LOGIN_SELECT).await?;
    login.click().await?;

    browser
        .wait()
        .at_most(MAX_WAIT_SECS)
        .for_url(TWITTER_HOME.parse().unwrap())
        .await?;
    let cookies = browser
        .get_all_cookies()
        .await?
        .into_iter()
        .map(|cookie| (cookie.name().to_string(), cookie.value().to_string()))
        .collect();

    browser.delete_all_cookies().await?;

    Ok(cookies)
}
