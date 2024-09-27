use bon::bon;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Body, Client, Method, RequestBuilder, Response,
};

use crate::{
    apis::{
        downloads::DownloadsApi, hosts::HostsApi, settings::SettingsApi, unrestrict::UnrestrictApi,
    },
    models,
};
use crate::{
    apis::{root::RootApi, streaming::StreamingApi, traffic::TrafficApi},
    consts::REAL_DEBRID_BASE_URL,
};
use crate::{
    apis::{torrents::TorrentsApi, user::UserApi},
    error::{DebridError, Error, Result},
};

/// RealDebrid client
#[derive(Debug, Clone)]
pub struct Debrid {
    client: Client,
    base_url: String,
}

#[bon]
impl Debrid {
    /// Creates a new RealDebrid client.
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
    ///     Ok(())
    /// }
    /// ```
    #[builder]
    pub fn new(
        /// RealDebrid API token
        token: Option<String>,
        /// RealDebrid API base url
        base_url: Option<String>,
    ) -> Result<Self> {
        let mut headers = HeaderMap::new();

        if let Some(token) = token {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &token))
                    .map_err(Error::DeserializeHeaderValue)?,
            );
        }

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(Error::Reqwest)?;

        let base_url = base_url.unwrap_or_else(|| REAL_DEBRID_BASE_URL.to_string());

        Ok(Self { client, base_url })
    }

    fn request<P: AsRef<str>>(&self, method: Method, path: P) -> RequestBuilder {
        let url = format!("{}{}", self.base_url, path.as_ref());
        self.client.request(method, url)
    }

    async fn handle(response: Result<Response>) -> Result<Response> {
        let response = response?;

        if response.status().is_success() {
            Ok(response)
        } else {
            if let Ok(error) = response.json::<models::error::Error>().await {
                Err(Error::Debrid(DebridError::from(error.code)))
            } else {
                Err(Error::Debrid(DebridError::InternalError))
            }
        }
    }

    pub(crate) async fn get<P: AsRef<str>, Q: serde::Serialize>(
        &self,
        path: P,
        query: Option<Q>,
    ) -> Result<Response> {
        let mut request = self.request(Method::GET, path);

        if let Some(q) = query {
            request = request.query(&q);
        }

        let response = request.send().await.map_err(Error::Reqwest);

        Self::handle(response).await
    }

    pub(crate) async fn post<P: AsRef<str>, D: serde::Serialize, Q: serde::Serialize>(
        &self,
        path: P,
        data: D,
        query: Option<Q>,
    ) -> Result<Response> {
        let mut request = self.request(Method::POST, path);

        if let Some(q) = query {
            request = request.query(&q);
        }

        let response = request.form(&data).send().await.map_err(Error::Reqwest);

        Self::handle(response).await
    }

    pub(crate) async fn put<P: AsRef<str>, B: Into<Body>, Q: serde::Serialize>(
        &self,
        path: P,
        body: B,
        query: Option<Q>,
    ) -> Result<Response> {
        let mut request = self.request(Method::PUT, path);

        if let Some(q) = query {
            request = request.query(&q);
        }

        let response = request.body(body).send().await.map_err(Error::Reqwest);

        Self::handle(response).await
    }

    pub(crate) async fn delete<P: AsRef<str>, Q: serde::Serialize>(
        &self,
        path: P,
        query: Option<Q>,
    ) -> Result<Response> {
        let mut request = self.request(Method::DELETE, path);

        if let Some(q) = query {
            request = request.query(&q);
        }

        let response = request.send().await.map_err(Error::Reqwest);

        Self::handle(response).await
    }
}

#[bon]
impl Debrid {
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
    pub async fn disable_access_token(&self) -> Result<()> {
        RootApi(&self).disable_access_token().send().await
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
    pub async fn time(&self) -> Result<String> {
        RootApi(&self).time().send().await
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
    pub async fn time_iso(&self) -> Result<String> {
        RootApi(&self).time_iso().send().await
    }

    /// RealDebrid user api.
    pub fn user<'rd>(&'rd self) -> UserApi<'rd> {
        UserApi(&self)
    }

    /// RealDebrid unrestrict api.
    pub fn unrestrict<'rd>(&'rd self) -> UnrestrictApi<'rd> {
        UnrestrictApi(&self)
    }

    /// RealDebrid traffic api.
    pub fn traffic<'rd>(&'rd self) -> TrafficApi<'rd> {
        TrafficApi(&self)
    }

    /// RealDebrid streaming api.
    pub fn streaming<'rd>(&'rd self) -> StreamingApi<'rd> {
        StreamingApi(&self)
    }

    /// RealDebrid downloads api.
    pub fn downloads<'rd>(&'rd self) -> DownloadsApi<'rd> {
        DownloadsApi(&self)
    }

    /// RealDebrid torrents api.
    pub fn torrents<'rd>(&'rd self) -> TorrentsApi<'rd> {
        TorrentsApi(&self)
    }

    /// RealDebrid hosts api.
    pub fn hosts<'rd>(&'rd self) -> HostsApi<'rd> {
        HostsApi(&self)
    }

    /// RealDebrid settings api.
    pub fn settings<'rd>(&'rd self) -> SettingsApi<'rd> {
        SettingsApi(&self)
    }
}
