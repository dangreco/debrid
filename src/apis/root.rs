use bon::bon;

use crate::{Debrid, Error, Result};

/// RealDebrid root API
pub struct RootApi<'rd>(pub(crate) &'rd Debrid);

#[bon]
impl<'rd> RootApi<'rd> {
    /// Disable current access token.
    //
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
    ///     let disabled = client.disable_access_token()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(()) = disabled {
    ///         println!("Access token disabled");
    ///     } else {
    ///         println!("Access token not disabled");
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub(crate) async fn disable_access_token(&self) -> Result<()> {
        self.0.get::<_, ()>("/disable_access_token", None).await?;
        Ok(())
    }

    /// Get server time.
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
    ///     let time = client.time()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(time) = time {
    ///         println!("{:?}", time);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub(crate) async fn time(&self) -> Result<String> {
        self.0
            .get::<_, ()>("/time", None)
            .await?
            .text()
            .await
            .map_err(Error::Reqwest)
    }

    /// Get server time in ISO.
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
    ///     let time = client.time_iso()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(time) = time {
    ///         println!("{:?}", time);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub(crate) async fn time_iso(&self) -> Result<String> {
        self.0
            .get::<_, ()>("/time/iso", None)
            .await?
            .text()
            .await
            .map_err(Error::Reqwest)
    }
}
