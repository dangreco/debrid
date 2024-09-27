use crate::{models, Debrid, Error, Result};
use bon::bon;
use reqwest::Body;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

/// RealDebrid unrestrict API
pub struct UnrestrictApi<'rd>(pub(crate) &'rd Debrid);

#[bon]
impl<'rd> UnrestrictApi<'rd> {
    /// Check if a file is downloadable on the concerned hoster.
    /// This request does not require authentication.
    ///
    /// # Examples
    /// ```
    /// use debrid::{Debrid, Error, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Debrid::builder().build()?;
    ///
    ///     let check = client.unrestrict().check()
    ///         .link("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string())
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(check) = check {
    ///         println!("{} ({} bytes)", check.host, check.filesize);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn check(
        &self,
        /// Original hoster link
        link: String,
        /// Password to unlock the file access hoster side
        password: Option<String>,
    ) -> Result<models::unrestrict::Check> {
        #[derive(serde::Serialize)]
        struct Body {
            link: String,
            password: Option<String>,
        }

        self.0
            .post::<_, Body, ()>("/unrestrict/check", Body { link, password }, None)
            .await?
            .json::<models::unrestrict::Check>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Unrestrict a hoster link and get a new unrestricted link
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
    ///     let link = client.unrestrict().link()
    ///         .link("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string())
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(link) = link {
    ///         println!("{} ({} bytes) - {}", link.host, link.filesize, link.download);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn link(
        &self,
        /// Original hoster link
        link: String,
        /// Password to unlock the file access hoster side
        password: Option<String>,
        /// Use remote traffic, dedicated servers and account sharing protections lifted
        remote: Option<bool>,
    ) -> Result<models::unrestrict::Link> {
        #[derive(serde::Serialize)]
        struct Body {
            link: String,
            password: Option<String>,
            remote: Option<u8>,
        }

        self.0
            .post::<_, Body, ()>(
                "/unrestrict/link",
                Body {
                    link,
                    password,
                    remote: remote.map(|b| if b { 1 } else { 0 }),
                },
                None,
            )
            .await?
            .json::<models::unrestrict::Link>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Unrestrict a hoster folder link and get individual links, returns an empty array if no links found.
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
    ///     let links = client.unrestrict().folder()
    ///         .link("https://docs.google.com/drive/folders/123ABC456def789ghi_jklMNOpqrsTUVwxyz".to_string())
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(links) = links {
    ///         println!("{:?}", links);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn folder(
        &self,
        /// Hoster folder link
        link: String,
    ) -> Result<Vec<String>> {
        #[derive(serde::Serialize)]
        struct Body {
            link: String,
        }

        self.0
            .post::<_, Body, ()>("/unrestrict/folder", Body { link }, None)
            .await?
            .json::<Vec<String>>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Decrypt a container file (RSDF, CCF, CCF3, DLC)
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
    ///     let file = tokio::fs::File::open("example.RSDF").await;
    ///
    ///     if let Ok(file) = file {
    ///         let links = client.unrestrict().container_file()
    ///             .file(file)
    ///             .send()
    ///             .await;
    ///
    ///         if let Ok(links) = links {
    ///             println!("{:?}", links);
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    ///
    /// ```
    #[builder(finish_fn = send)]
    pub async fn container_file(
        &self,
        /// Container file
        file: File,
    ) -> Result<Vec<String>> {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);

        self.0
            .put::<_, _, ()>("/unrestrict/containerFile", body, None)
            .await?
            .json::<Vec<String>>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Decrypt a container file from a link.
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
    ///     let links = client.unrestrict().container_link()
    ///         .link("https://example.com/example.RSDF".to_string())
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(links) = links {
    ///         println!("{:?}", links);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn container_link(
        &self,
        /// HTTP Link of the container file
        link: String,
    ) -> Result<Vec<String>> {
        #[derive(serde::Serialize)]
        struct Body {
            link: String,
        }

        self.0
            .post::<_, Body, ()>("/unrestrict/containerLink", Body { link }, None)
            .await?
            .json::<Vec<String>>()
            .await
            .map_err(Error::Reqwest)
    }
}
