use crate::browser::log_in_using_browser;
use crate::config::{Account, Proxy};
use color_eyre::Report;
use dashmap::DashMap;
use deadpool::managed::RecycleResult;
use deadpool::{async_trait, managed};
use nanorand::{Rng, WyRand};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::{error, instrument};
use twtscrape::scrape::{Scraper, ScraperBuilder};

const STATUS_CHECK_URL: &str = "https://twitter.com/i/api/graphql/iugWi6fZBxE7Qzt_5PiIYw/Viewer?variables=%7B%22withCommunitiesMemberships%22%3Atrue%2C%22withCommunitiesCreation%22%3Atrue%2C%22withSuperFollowsUserFields%22%3Atrue%7D&features=%7B%22responsive_web_twitter_blue_verified_badge_is_enabled%22%3Atrue%2C%22verified_phone_label_enabled%22%3Afalse%2C%22responsive_web_graphql_timeline_navigation_enabled%22%3Atrue%7D";

pub struct TwitterScraperManager {
    pub accounts: Arc<RwLock<Vec<Account>>>,
    pub proxies: Arc<DashMap<String, Proxy>>,
    pub rng: Mutex<WyRand>,
}

#[async_trait]
impl managed::Manager for TwitterScraperManager {
    type Type = Scraper;
    type Error = Report;

    #[instrument]
    async fn create(&self) -> Result<Self::Type, Self::Error> {
        if self.accounts.len() == 0 {
            return Err(Report::msg("There must be more than 1 account"));
        }
        let accs = &self.accounts.read().await;
        let accidx = {
            let mut rnglck = self.rng.lock().await;
            rnglck.generate_range(0..accs.len())
        };
        let account = accs.get(accidx).unwrap();
        // TODO: Scraper Account Login and Proxy with Authentications
        let mut scraper_bld = ScraperBuilder::new();

        if let Some(assigned) = &account.assigned_proxy {
            match self.proxies.get(assigned) {
                Some(p) => {
                    scraper_bld = scraper_bld.with_proxy(p.ip.to_string());
                    scraper_bld = scraper_bld
                        .with_proxy_authentication(p.username.clone(), p.password.clone())
                } // FIXME: Basic Credentials
                None => {
                    let username = account.username.clone();
                    let proxy = assigned.clone();
                    error!(user = username, assigned = proxy, "Not found!");
                    Err(Report::msg("Proxy Not Found"))
                }
            }
        }

        let cookies = log_in_using_browser(account, self.proxies.clone()).await?;
        scraper_bld = scraper_bld.with_cookies(cookies);
        scraper_bld = scraper_bld.with_ua("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".to_string());

        let scraper = scraper_bld.finish().await?;
        // status check
        let req_status_check = scraper.make_get_req(STATUS_CHECK_URL);
        scraper.api_req_raw_request(req_status_check).await?;
        Ok(scraper)
    }

    #[instrument]
    async fn recycle(&self, obj: &mut Self::Type) -> RecycleResult<Self::Error> {
        let req_status_check = obj.make_get_req(STATUS_CHECK_URL);
        obj.api_req_raw_request(req_status_check).await?;
        Ok(())
    }
}

pub struct AnonymousScraperManager {
    pub proxies: Arc<DashMap<String, Proxy>>,
    pub rng: Mutex<WyRand>,
}

impl managed::Manager for AnonymousScraperManager {
    type Type = Scraper;
    type Error = Report;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        let mut scraperbld = ScraperBuilder::new();
        scraperbld = scraper_bld.with_ua("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".to_string());
        let random_v = self.proxies.iter().collect::<Vec<_>>();
        let rng = self.rng.lock().await.generate_range(0..random_v.len());
        scraperbld = scraperbld.with_proxy(random_v[rng].ip.clone());
        scraperbld = scraperbld.with_proxy_authentication(
            random_v[rng].username.clone(),
            random_v[rng].password.clone(),
        );

        let scraper = scraper_bld.finish().await?;
        // status check
        let req_status_check = scraper.make_get_req(STATUS_CHECK_URL);
        scraper.api_req_raw_request(req_status_check).await?;
        Ok(scraper)
    }

    async fn recycle(&self, obj: &mut Self::Type) -> RecycleResult<Self::Error> {
        let req_status_check = obj.make_get_req(STATUS_CHECK_URL);
        obj.api_req_raw_request(req_status_check).await?;
        Ok(())
    }
}
