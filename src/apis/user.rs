use bon::bon;

use crate::{models, Debrid, Error, Result};

/// RealDebrid user API
pub struct UserApi<'rd>(pub(crate) &'rd Debrid);

#[bon]
impl<'rd> UserApi<'rd> {
    /// Returns information on the current user.
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
    ///     let user = client.user().get()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(user) = user {
    ///         println!("{}", user.username);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn get(&self) -> Result<models::user::User> {
        self.0
            .get::<_, ()>("/user", None)
            .await?
            .json::<models::user::User>()
            .await
            .map_err(Error::Reqwest)
    }
}
