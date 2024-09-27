use std::collections::HashMap;

use bon::bon;
use reqwest::Body;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{
    models::{self, torrents::AddedTorrent},
    Debrid, DebridError, Error, Result,
};

/// RealDebrid torrents API
pub struct TorrentsApi<'rd>(pub(crate) &'rd Debrid);

#[bon]
impl<'rd> TorrentsApi<'rd> {
    /// Get user torrents list.
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
    ///     let torrents = client.torrents().get()
    ///         .page(10)
    ///         .limit(25)
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(torrents) = torrents {
    ///         for torrent in torrents {
    ///             println!("[{}] {}", torrent.hash, torrent.filename);
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
        offset: Option<u64>,
        /// Pagination system
        page: Option<u64>,
        /// Entries returned per page / request (must be within 0 and 5000, default: 100)
        limit: Option<u64>,
        /// Filter to apply (e.g. "active" to list active torrents only)
        filter: Option<String>,
    ) -> Result<Vec<models::torrents::Torrent>> {
        #[derive(serde::Serialize)]
        struct Query {
            offset: Option<u64>,
            page: Option<u64>,
            limit: Option<u64>,
            filter: Option<String>,
        }

        self.0
            .get::<_, Query>(
                "/torrents",
                Some(Query {
                    offset,
                    page,
                    limit,
                    filter,
                }),
            )
            .await?
            .json::<Vec<models::torrents::Torrent>>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Get length of user torrents list
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
    ///     let n = client.torrents().len()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(n) = n {
    ///         println!("{} torrents", n);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn len(&self) -> Result<usize> {
        let response = self.0.get::<_, ()>("/torrents", None).await?;

        match response.headers().get("X-Total-Count") {
            Some(value) => Ok(usize::from_str_radix(
                value.to_str().map_err(Error::SerializeHeaderValue)?,
                10,
            )
            .map_err(Error::ParseInt)?),
            None => Err(Error::Debrid(DebridError::InternalError)),
        }
    }

    /// Get all information for the given torrent.
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
    ///     let info = client.torrents().info()
    ///         .id("ABCDEFGHIJKLMNOP".to_string())
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(info) = info {
    ///         println!("[{}] {}", info.hash, info.filename);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn info(
        &self,
        /// RealDebrid torrent ID
        id: String,
    ) -> Result<models::torrents::TorrentInfo> {
        self.0
            .get::<_, ()>(format!("/torrents/info/{}", id), None)
            .await?
            .json::<models::torrents::TorrentInfo>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Get list of instantly available file IDs by hoster.
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
    ///     let available = client.torrents().instant_availability()
    ///         .hashes(vec![
    ///             "ABCDEFGHIJKLMNOP".to_string(),
    ///             "ABCDEFGHIJKLMNOP".to_string(),
    ///             "ABCDEFGHIJKLMNOP".to_string(),
    ///          ])
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(available) = available {
    ///         for (hash, hosters) in available {
    ///             println!("[{}] {} hosters", hash, hosters.len());
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn instant_availability(
        &self,
        /// Torrent hashes to check
        hashes: Vec<String>,
    ) -> Result<HashMap<String, models::torrents::InstantAvailability>> {
        self.0
            .get::<_, ()>(
                format!("/torrents/instantAvailability/{}", hashes.join(",")),
                None,
            )
            .await?
            .json::<HashMap<String, models::torrents::InstantAvailability>>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Get the number of currently active torrents and the current maximum limit.
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
    ///     let count = client.torrents().active_count()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(count) = count {
    ///         println!("{}/{}", count.nb, count.limit);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn active_count(&self) -> Result<models::torrents::ActiveCount> {
        self.0
            .get::<_, ()>("/torrents/activeCount", None)
            .await?
            .json::<models::torrents::ActiveCount>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Get available hosts to upload the torrent to.
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
    ///     let hosts = client.torrents().available_hosts()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(hosts) = hosts {
    ///         for host in hosts {
    ///             println!("{} {}", host.host, host.max_file_size);
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn available_hosts(&self) -> Result<Vec<models::torrents::AvailableHost>> {
        self.0
            .get::<_, ()>("/torrents/availableHosts", None)
            .await?
            .json::<Vec<models::torrents::AvailableHost>>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Add a torrent file to download.
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
    ///     let file = tokio::fs::File::open("example.torrent").await;
    ///
    ///     if let Ok(file) = file {
    ///         let added = client.torrents().add_torrent()
    ///             .file(file)
    ///             .send()
    ///             .await;
    ///
    ///         if let Ok(added) = added {
    ///             println!("{}", added.id);
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    ///
    /// ```
    #[builder(finish_fn = send)]
    pub async fn add_torrent(
        &self,
        /// Torrent file to upload
        file: File,
        host: Option<String>,
    ) -> Result<AddedTorrent> {
        #[derive(serde::Serialize)]
        struct Query {
            host: Option<String>,
        }

        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);

        self.0
            .put::<_, _, Query>("/torrents/addTorrent", body, Some(Query { host }))
            .await?
            .json::<AddedTorrent>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Add a magnet link to download.
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
    ///     let added = client.torrents().add_magnet()
    ///         .magnet("ABCDEFGHIJKLMNOP".to_string())
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(added) = added {
    ///         println!("{}", added.id);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn add_magnet(
        &self,
        /// Magnet link to add
        magnet: String,
        host: Option<String>,
    ) -> Result<AddedTorrent> {
        #[derive(serde::Serialize)]
        struct Body {
            magnet: String,
            host: Option<String>,
        }

        self.0
            .post::<_, Body, ()>("/torrents/addMagnet", Body { magnet, host }, None)
            .await?
            .json::<AddedTorrent>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Select files of a torrent to start it.
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
    ///     let ok = client.torrents().select_files()
    ///         .id("ABCDEFGHIJKLMNOP".to_string())
    ///         .files(vec![
    ///             "file0".to_string(),
    ///             "file1".to_string(),
    ///             "file2".to_string(),
    ///          ])
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(()) = ok {
    ///         println!("Torrent started");
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn select_files(
        &self,
        /// RealDebrid torrent ID
        id: String,
        /// Selected file IDs
        files: Vec<String>,
    ) -> Result<()> {
        #[derive(serde::Serialize)]
        struct Body {
            files: String,
        }

        self.0
            .post::<_, Body, ()>(
                format!("/torrents/selectFiles/{}", id),
                Body {
                    files: files.join(","),
                },
                None,
            )
            .await?;

        Ok(())
    }

    /// Delete a torrent from torrents list.
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
    ///     let ok = client.torrents().delete()
    ///         .id("ABCDEFGHIJKLMNOP".to_string())
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(()) = ok {
    ///         println!("Torrent deleted");
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn delete(
        &self,
        /// RealDebrid torrent ID
        id: String,
    ) -> Result<()> {
        self.0
            .delete::<_, ()>(format!("/torrents/delete/{}", id), None)
            .await?;

        Ok(())
    }
}
