use std::{fmt, num::ParseIntError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error deserializing header value: {0}")]
    DeserializeHeaderValue(reqwest::header::InvalidHeaderValue),

    #[error("Error serializing header value: {0}")]
    SerializeHeaderValue(reqwest::header::ToStrError),

    #[error("Error parsing int: {0}")]
    ParseInt(ParseIntError),

    #[error("Reqwest error: {0}")]
    Reqwest(reqwest::Error),

    #[error("Debrid error: {0}")]
    Debrid(DebridError),

    #[error("Regex error: {0}")]
    Regex(regex::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// RealDebrid-specific error
#[derive(Debug, PartialEq)]
pub enum DebridError {
    InternalError,
    MissingParameter,
    BadParameterValue,
    UnknownMethod,
    MethodNotAllowed,
    SlowDown,
    RessourceUnreachable,
    ResourceNotFound,
    BadToken,
    PermissionDenied,
    TwoFactorAuthenticationNeeded,
    TwoFactorAuthenticationPending,
    InvalidLogin,
    InvalidPassword,
    AccountLocked,
    AccountNotActivated,
    UnsupportedHoster,
    HosterInMaintenance,
    HosterLimitReached,
    HosterTemporarilyUnavailable,
    HosterNotAvailableForFreeUsers,
    TooManyActiveDownloads,
    IPAddressNotAllowed,
    TrafficExhausted,
    FileUnavailable,
    ServiceUnavailable,
    UploadTooBig,
    UploadError,
    FileNotAllowed,
    TorrentTooBig,
    TorrentFileInvalid,
    ActionAlreadyDone,
    ImageResolutionError,
    TorrentAlreadyActive,
    TooManyRequests,
    InfringingFile,
    FairUsageLimit,
}

impl From<i32> for DebridError {
    fn from(code: i32) -> Self {
        match code {
            -1 => DebridError::InternalError,
            1 => DebridError::MissingParameter,
            2 => DebridError::BadParameterValue,
            3 => DebridError::UnknownMethod,
            4 => DebridError::MethodNotAllowed,
            5 => DebridError::SlowDown,
            6 => DebridError::RessourceUnreachable,
            7 => DebridError::ResourceNotFound,
            8 => DebridError::BadToken,
            9 => DebridError::PermissionDenied,
            10 => DebridError::TwoFactorAuthenticationNeeded,
            11 => DebridError::TwoFactorAuthenticationPending,
            12 => DebridError::InvalidLogin,
            13 => DebridError::InvalidPassword,
            14 => DebridError::AccountLocked,
            15 => DebridError::AccountNotActivated,
            16 => DebridError::UnsupportedHoster,
            17 => DebridError::HosterInMaintenance,
            18 => DebridError::HosterLimitReached,
            19 => DebridError::HosterTemporarilyUnavailable,
            20 => DebridError::HosterNotAvailableForFreeUsers,
            21 => DebridError::TooManyActiveDownloads,
            22 => DebridError::IPAddressNotAllowed,
            23 => DebridError::TrafficExhausted,
            24 => DebridError::FileUnavailable,
            25 => DebridError::ServiceUnavailable,
            26 => DebridError::UploadTooBig,
            27 => DebridError::UploadError,
            28 => DebridError::FileNotAllowed,
            29 => DebridError::TorrentTooBig,
            30 => DebridError::TorrentFileInvalid,
            31 => DebridError::ActionAlreadyDone,
            32 => DebridError::ImageResolutionError,
            33 => DebridError::TorrentAlreadyActive,
            34 => DebridError::TooManyRequests,
            35 => DebridError::InfringingFile,
            36 => DebridError::FairUsageLimit,
            _ => DebridError::InternalError, // Default to InternalError for unknown codes
        }
    }
}

impl fmt::Display for DebridError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DebridError::InternalError => "Internal error",
                DebridError::MissingParameter => "Missing parameter",
                DebridError::BadParameterValue => "Bad parameter value",
                DebridError::UnknownMethod => "Unknown method",
                DebridError::MethodNotAllowed => "Method not allowed",
                DebridError::SlowDown => "Slow down",
                DebridError::RessourceUnreachable => "Ressource unreachable",
                DebridError::ResourceNotFound => "Resource not found",
                DebridError::BadToken => "Bad token",
                DebridError::PermissionDenied => "Permission denied",
                DebridError::TwoFactorAuthenticationNeeded => "Two-Factor authentication needed",
                DebridError::TwoFactorAuthenticationPending => "Two-Factor authentication pending",
                DebridError::InvalidLogin => "Invalid login",
                DebridError::InvalidPassword => "Invalid password",
                DebridError::AccountLocked => "Account locked",
                DebridError::AccountNotActivated => "Account not activated",
                DebridError::UnsupportedHoster => "Unsupported hoster",
                DebridError::HosterInMaintenance => "Hoster in maintenance",
                DebridError::HosterLimitReached => "Hoster limit reached",
                DebridError::HosterTemporarilyUnavailable => "Hoster temporarily unavailable",
                DebridError::HosterNotAvailableForFreeUsers =>
                    "Hoster not available for free users",
                DebridError::TooManyActiveDownloads => "Too many active downloads",
                DebridError::IPAddressNotAllowed => "IP Address not allowed",
                DebridError::TrafficExhausted => "Traffic exhausted",
                DebridError::FileUnavailable => "File unavailable",
                DebridError::ServiceUnavailable => "Service unavailable",
                DebridError::UploadTooBig => "Upload too big",
                DebridError::UploadError => "Upload error",
                DebridError::FileNotAllowed => "File not allowed",
                DebridError::TorrentTooBig => "Torrent too big",
                DebridError::TorrentFileInvalid => "Torrent file invalid",
                DebridError::ActionAlreadyDone => "Action already done",
                DebridError::ImageResolutionError => "Image resolution error",
                DebridError::TorrentAlreadyActive => "Torrent already active",
                DebridError::TooManyRequests => "Too many requests",
                DebridError::InfringingFile => "Infringing file",
                DebridError::FairUsageLimit => "Fair Usage Limit",
            }
        )
    }
}
