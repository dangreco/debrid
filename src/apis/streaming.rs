use bon::bon;

use crate::{models, Debrid, Error, Result};

/// RealDebrid streaming API
pub struct StreamingApi<'rd>(pub(crate) &'rd Debrid);

#[bon]
impl<'rd> StreamingApi<'rd> {
    /// Get transcoding links for a given file.
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
    ///     let transcode = client.streaming().transcode()
    ///         .id("ABCDEFGHIJKLMNOP".to_string())
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(transcode) = transcode {
    ///         println!("{:?}", transcode);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn transcode(
        &self,
        /// RealDebrid file ID
        id: String,
    ) -> Result<models::streaming::Transcode> {
        self.0
            .get::<_, ()>(format!("/streaming/transcode/{}", id), None)
            .await?
            .json::<models::streaming::Transcode>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Get detailled media information for a given file.
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
    ///     let info = client.streaming().media_info()
    ///         .id("ABCDEFGHIJKLMNOP".to_string())
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(info) = info {
    ///         println!("{} ({} bytes)", info.filename, info.size);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn media_info(
        &self,
        /// RealDebrid file ID
        id: String,
    ) -> Result<models::streaming::MediaInfo> {
        self.0
            .get::<_, ()>(format!("/streaming/mediaInfos/{}", id), None)
            .await?
            .json::<models::streaming::MediaInfo>()
            .await
            .map_err(Error::Reqwest)
    }
}
