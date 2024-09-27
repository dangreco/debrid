use std::collections::HashMap;

use bon::bon;
use regex::Regex;

use crate::{models, Debrid, Error, Result};

/// RealDebrid hosts API
pub struct HostsApi<'rd>(pub(crate) &'rd Debrid);

#[bon]
impl<'rd> HostsApi<'rd> {
    /// Get supported hosts. This request does not require authentication.
    ///
    /// # Examples
    /// ```
    /// use debrid::{Debrid, Error, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Debrid::builder().build()?;
    ///
    ///     let hosts = client.hosts().get()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(hosts) = hosts {
    ///         for (domain, host) in hosts {
    ///             println!("{}: {} ({})", host.id, host.name, domain);
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn get(&self) -> Result<HashMap<String, models::hosts::Host>> {
        self.0
            .get::<_, ()>("/hosts", None)
            .await?
            .json::<HashMap<String, models::hosts::Host>>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Get status of supported hosters or not and their status on competitors.
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
    ///     let hosts = client.hosts().status()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(hosts) = hosts {
    ///         for (domain, host) in hosts {
    ///             println!("{}: {} ({}) [{:?}]", host.id, host.name, domain, host.status);
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn status(&self) -> Result<HashMap<String, models::hosts::HostInfo>> {
        self.0
            .get::<_, ()>("/hosts/status", None)
            .await?
            .json::<HashMap<String, models::hosts::HostInfo>>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Get all supported links regex, useful to find supported links inside a document. 
    /// This request does not require authentication.
    //
    /// # Examples
    /// ```
    /// use debrid::{Debrid, Error, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Debrid::builder().build()?;
    ///
    ///     let regexes = client.hosts().regex()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(regexes) = regexes {
    ///         for regex in regexes {
    ///             // Do something
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn regex(&self) -> Result<Vec<Regex>> {
        let regex_strings = self
            .0
            .get::<_, ()>("/hosts/regex", None)
            .await?
            .json::<Vec<String>>()
            .await
            .map_err(Error::Reqwest)?;

        regex_strings
            .into_iter()
            .map(|s| {
                let unescaped = s.replace("\\\\", "\\");
                let trimmed = unescaped.trim_matches('/');
                Regex::new(&trimmed)
            })
            .collect::<std::result::Result<Vec<Regex>, _>>()
            .map_err(Error::Regex)
    }

    /// Get all supported folder regex, useful to find supported links inside a document.
    /// This request does not require authentication.
    //
    /// # Examples
    /// ```
    /// use debrid::{Debrid, Error, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Debrid::builder().build()?;
    ///
    ///     let regexes = client.hosts().regex_folder()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(regexes) = regexes {
    ///         for regex in regexes {
    ///             // Do something
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn regex_folder(&self) -> Result<Vec<Regex>> {
        let regex_strings = self
            .0
            .get::<_, ()>("/hosts/regexFolder", None)
            .await?
            .json::<Vec<String>>()
            .await
            .map_err(Error::Reqwest)?;

        regex_strings
            .into_iter()
            .map(|s| {
                let unescaped = s.replace("\\\\", "\\");
                let trimmed = unescaped.trim_matches('/');
                Regex::new(&trimmed)
            })
            .collect::<std::result::Result<Vec<Regex>, _>>()
            .map_err(Error::Regex)
    }

    /// Get all hoster domains supported on the service.
    /// This request does not require authentication.
    //
    /// # Examples
    /// ```
    /// use debrid::{Debrid, Error, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Debrid::builder().build()?;
    ///
    ///     let domains = client.hosts().domains()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(domains) = domains {
    ///         for domain in domains {
    ///             // Do something
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn domains(&self) -> Result<Vec<String>> {
        self.0
            .get::<_, ()>("/hosts/domains", None)
            .await?
            .json::<Vec<String>>()
            .await
            .map_err(Error::Reqwest)
    }
}
