use std::collections::HashMap;

use bon::bon;

use crate::{models, Debrid, Error, Result};

/// RealDebrid traffic API
pub struct TrafficApi<'rd>(pub(crate) &'rd Debrid);

#[bon]
impl<'rd> TrafficApi<'rd> {
    /// Get traffic information for limited hosters (limits, current usage, extra packages).
    ///
    /// # Examples
    /// ```
    /// use debrid::{Debrid, Error, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Debrid::builder()
    ///         .token("LOREMIPSUM".to_string())
    ///         .build()?;
    ///
    ///     let traffic = client.traffic().get()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(traffic) = traffic {
    ///         for (host, info) in traffic {
    ///             println!("{} {:?}", host, info);
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn get(&self) -> Result<HashMap<String, models::traffic::Traffic>> {
        self.0
            .get::<_, ()>("/traffic", None)
            .await?
            .json::<HashMap<String, models::traffic::Traffic>>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Get traffic details on each hoster used during a defined period
    ///
    /// # Examples
    /// ```
    /// use debrid::{Debrid, Error, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Debrid::builder()
    ///         .token("LOREMIPSUM".to_string())
    ///         .build()?;
    ///
    ///     let details = client.traffic().details()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(details) = details {
    ///         for (host, info) in details {
    ///             println!("{} {:?}", host, info);
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn details(
        &self,
        /// Start period, default: a week ago
        start: Option<String>,
        /// End period, default: today
        end: Option<String>,
    ) -> Result<HashMap<String, models::traffic::Detail>> {
        #[derive(serde::Serialize)]
        struct Query {
            start: Option<String>,
            end: Option<String>,
        }

        self.0
            .get::<_, Query>("/traffic/details", Some(Query { start, end }))
            .await?
            .json::<HashMap<String, models::traffic::Detail>>()
            .await
            .map_err(Error::Reqwest)
    }
}
