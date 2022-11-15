use crate::config::Account;
use crate::AppState;
use color_eyre::Report;
use deadpool::managed::RecycleResult;
use deadpool::{async_trait, managed};
use nanorand::{Rng, WyRand};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::instrument;
use twtscrape::scrape::{Scraper, ScraperBuilder};

const STATUS_CHECK_URL: &str = "https://twitter.com/i/api/graphql/iugWi6fZBxE7Qzt_5PiIYw/Viewer?variables=%7B%22withCommunitiesMemberships%22%3Atrue%2C%22withCommunitiesCreation%22%3Atrue%2C%22withSuperFollowsUserFields%22%3Atrue%7D&features=%7B%22responsive_web_twitter_blue_verified_badge_is_enabled%22%3Atrue%2C%22verified_phone_label_enabled%22%3Afalse%2C%22responsive_web_graphql_timeline_navigation_enabled%22%3Atrue%7D";

pub struct TwitterScraperManager {
    pub accounts: Arc<RwLock<Vec<Account>>>,
    pub rng: Mutex<WyRand>,
}

#[async_trait]
impl<'a> managed::Manager for TwitterScraperManager {
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
            scraper_bld = scraper_bld.with_proxy(assigned.clone());
        }

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
