use std::path::Path;

pub use yup_oauth2::{
    authorized_user::AuthorizedUserSecret, hyper::client::HttpConnector,
    hyper_rustls::HttpsConnector, AccessToken, ApplicationDefaultCredentialsFlowOpts,
    ApplicationSecret, InstalledFlowReturnMethod, ServiceAccountKey,
};

use crate::{Error, Result};

/// A generic authentication mechanism for FCM, unifying the several methods provided by [yup-oauth2](yup_oauth2).
#[derive(Clone)]
pub struct Authenticator {
    inner: yup_oauth2::authenticator::Authenticator<HttpsConnector<HttpConnector>>,
}

impl Authenticator {
    /// Construct a new access token authenticator.
    pub async fn access_token<S: AsRef<str>>(access_token: S) -> Result<Authenticator> {
        let inner = yup_oauth2::AccessTokenAuthenticator::builder(access_token.as_ref().to_owned())
            .build()
            .await
            .map_err(|_| Error::Auth)?;
        Ok(Authenticator { inner })
    }

    /// Construct a new Authorized User authenticator.
    pub async fn authorized_user(
        authorized_user_secret: AuthorizedUserSecret,
    ) -> Result<Authenticator> {
        let inner = yup_oauth2::AuthorizedUserAuthenticator::builder(authorized_user_secret)
            .build()
            .await
            .map_err(|_| Error::Auth)?;
        Ok(Authenticator { inner })
    }

    /// Construct a new Device Flow authenticator.
    pub async fn device_flow(app_secret: ApplicationSecret) -> Result<Authenticator> {
        let inner = yup_oauth2::DeviceFlowAuthenticator::builder(app_secret)
            .build()
            .await
            .map_err(|_| Error::Auth)?;
        Ok(Authenticator { inner })
    }

    /// Construct a new Installed Flow authenticator.
    pub async fn installed_flow(
        app_secret: ApplicationSecret,
        method: InstalledFlowReturnMethod,
    ) -> Result<Authenticator> {
        let inner = yup_oauth2::InstalledFlowAuthenticator::builder(app_secret, method)
            .build()
            .await
            .map_err(|_| Error::Auth)?;
        Ok(Authenticator { inner })
    }

    /// Construct a new Service Account authenticator.
    pub async fn service_account<S: AsRef<str>>(
        service_account_key: ServiceAccountKey,
    ) -> Result<Authenticator> {
        let inner = yup_oauth2::ServiceAccountAuthenticator::builder(service_account_key)
            .build()
            .await
            .map_err(|_| Error::Auth)?;
        Ok(Authenticator { inner })
    }

    /// Construct a new Service Account authenticator from a JSON credentials file.
    pub async fn service_account_from_file<P: AsRef<Path>>(
        service_account_creds_filepath: P,
    ) -> Result<Authenticator> {
        let service_account_key =
            yup_oauth2::read_service_account_key(service_account_creds_filepath)
                .await
                .map_err(|_| Error::Auth)?;
        let inner = yup_oauth2::ServiceAccountAuthenticator::builder(service_account_key)
            .build()
            .await
            .map_err(|_| Error::Auth)?;
        Ok(Authenticator { inner })
    }

    /// Construct a new Service Account Impersonation authenticator.
    pub async fn service_account_impersonation<S: AsRef<str>>(
        authorized_user_secret: AuthorizedUserSecret,
        service_account_email: S,
    ) -> Result<Authenticator> {
        let inner = yup_oauth2::ServiceAccountImpersonationAuthenticator::builder(
            authorized_user_secret,
            service_account_email.as_ref(),
        )
        .build()
        .await
        .map_err(|_| Error::Auth)?;
        Ok(Authenticator { inner })
    }

    /// Return the current ID token. Use a cached result, if possible.
    pub async fn token<'a, T: AsRef<str>>(&'a self, scopes: &'a [T]) -> Result<AccessToken> {
        self.inner.token(scopes).await.map_err(|_| Error::Auth)
    }
}
