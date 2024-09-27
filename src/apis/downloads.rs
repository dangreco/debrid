use bon::bon;

use crate::{models, Debrid, DebridError, Error, Result};

/// RealDebrid downloads API
pub struct DownloadsApi<'rd>(pub(crate) &'rd Debrid);

#[bon]
impl<'rd> DownloadsApi<'rd> {
    /// Get user downloads list
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
    ///     let downloads = client.downloads().get()
    ///         .page(10)
    ///         .limit(25)
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(downloads) = downloads {
    ///         for download in downloads {
    ///             println!("{} - {}", download.id, download.filename);
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn get(
        &self,
        /// Starting offset (must be within 0 and len())
        offset: Option<usize>,
        /// Pagination system
        page: Option<usize>,
        /// Entries returned per page / request (must be within 0 and 5000, default: 100)
        limit: Option<usize>,
    ) -> Result<Vec<models::downloads::Download>> {
        #[derive(serde::Serialize)]
        struct Query {
            offset: Option<usize>,
            page: Option<usize>,
            limit: Option<usize>,
        }

        self.0
            .get::<_, Query>(
                "/downloads",
                Some(Query {
                    offset,
                    page,
                    limit,
                }),
            )
            .await?
            .json::<Vec<models::downloads::Download>>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Get length of user downloads list
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
    ///     let n = client.downloads().len()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(n) = n {
    ///         println!("{} downloads", n);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn len(&self) -> Result<usize> {
        let response = self.0.get::<_, ()>("/downloads", None).await?;

        match response.headers().get("X-Total-Count") {
            Some(value) => Ok(usize::from_str_radix(
                value.to_str().map_err(Error::SerializeHeaderValue)?,
                10,
            )
            .map_err(Error::ParseInt)?),
            None => Err(Error::Debrid(DebridError::InternalError)),
        }
    }

    /// Delete a link from user downloads list
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
    ///     let id = "LOREMIPSUM";  
    ///
    ///     let deleted = client.downloads().delete()
    ///         .id(id.to_string())
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(()) = deleted {
    ///         println!("Deleted {}", id);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn delete(
        &self,
        /// ID of RealDebrid download to delete
        id: String,
    ) -> Result<()> {
        self.0
            .delete::<_, ()>(format!("/downloads/delete/{}", id), None)
            .await?;

        Ok(())
    }
}
