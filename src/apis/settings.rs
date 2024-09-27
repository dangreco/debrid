use bon::bon;
use reqwest::Body;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{models, Debrid, Error, Result};

/// RealDebrid settings API
pub struct SettingsApi<'rd>(pub(crate) &'rd Debrid);

#[bon]
impl<'rd> SettingsApi<'rd> {
    /// Get current user settings with possible values to update.
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
    ///     let settings = client.settings().get()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(settings) = settings {
    ///         println!("Locale: {}", settings.locale);
    ///         println!("Quality: {}", settings.streaming_quality);
    ///         println!("Language: {}", settings.streaming_language_preference);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn get(&self) -> Result<models::settings::Settings> {
        self.0
            .get::<_, ()>("/settings", None)
            .await?
            .json::<models::settings::Settings>()
            .await
            .map_err(Error::Reqwest)
    }

    /// Update a user setting.
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
    ///     let set = client.settings().update()
    ///         .name("locale".to_string())
    ///         .value("fr".to_string())
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(()) = set {
    ///         println!("Locale set to French");
    ///     } else {
    ///         println!("Failed to set locale: {:?}", set);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn update(
        &self,
        /// Name of the setting to update
        name: String,
        /// Value of the setting to update
        value: String,
    ) -> Result<()> {
        #[derive(serde::Serialize)]
        struct Body {
            setting_name: String,
            setting_value: String,
        }

        self.0
            .post::<_, Body, ()>(
                "/settings/update",
                Body {
                    setting_name: name,
                    setting_value: value,
                },
                None,
            )
            .await?;

        Ok(())
    }

    /// Convert fidelity points.
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
    ///     let converted = client.settings().convert_points()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(()) = converted {
    ///         println!("Converted fidelity points");
    ///     } else {
    ///         println!("Failed to convert fidelity points: {:?}", converted);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn convert_points(&self) -> Result<()> {
        self.0
            .post::<_, (), ()>("/settings/convertPoints", (), None)
            .await?;

        Ok(())
    }

    /// Send the verification email to change the password.
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
    ///     let sent = client.settings().change_password()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(()) = sent {
    ///         println!("Email sent");
    ///     } else {
    ///         println!("Failed to send email: {:?}", sent);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn change_password(&self) -> Result<()> {
        self.0
            .post::<_, (), ()>("/settings/changePassword", (), None)
            .await?;

        Ok(())
    }

    /// Upload a new user avatar image.
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
    ///     let file = tokio::fs::File::open("avatar.jpg").await;
    ///
    ///     if let Ok(file) = file {
    ///         let uploaded = client.settings().set_avatar()
    ///             .file(file)
    ///             .send()
    ///             .await;
    ///
    ///         if let Ok(()) = uploaded {
    ///             println!("Uploaded avatar image");
    ///         } else {
    ///             println!("Failed to upload avatar image: {:?}", uploaded);
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn set_avatar(
        &self,
        /// Avatar image to upload
        file: File,
    ) -> Result<()> {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);

        self.0
            .put::<_, _, ()>("/settings/avatarFile", body, None)
            .await?;

        Ok(())
    }

    /// Reset user avatar image to default.
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
    ///     let deleted = client.settings().delete_avatar()
    ///         .send()
    ///         .await;
    ///
    ///     if let Ok(()) = deleted {
    ///         println!("Avatar image deleted");
    ///     } else {
    ///         println!("Failed to delete avatar image: {:?}", deleted);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[builder(finish_fn = send)]
    pub async fn delete_avatar(&self) -> Result<()> {
        self.0
            .delete::<_, ()>("/settings/avatarDelete", None)
            .await?;

        Ok(())
    }
}
