use std::convert::Infallible;
use std::str::FromStr;

use deserr::{DeserializeError, Deserr, MergeWithError, ValuePointerRef};
use milli::update::Setting;
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::macros::{format_description, time};
use time::{Date, OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use crate::actions::Action;
use crate::deserr::{immutable_field_error, DeserrError, DeserrJsonError};
use crate::error::deserr_codes::*;
use crate::error::{Code, ErrorCode, ParseOffsetDateTimeError};
use crate::index_uid_pattern::{IndexUidPattern, IndexUidPatternFormatError};

pub type KeyId = Uuid;

impl<C: Default + ErrorCode> MergeWithError<IndexUidPatternFormatError> for DeserrJsonError<C> {
    fn merge(
        _self_: Option<Self>,
        other: IndexUidPatternFormatError,
        merge_location: deserr::ValuePointerRef,
    ) -> std::ops::ControlFlow<Self, Self> {
        DeserrError::error::<Infallible>(
            None,
            deserr::ErrorKind::Unexpected { msg: other.to_string() },
            merge_location,
        )
    }
}

#[derive(Debug, Deserr)]
#[deserr(error = DeserrJsonError, rename_all = camelCase, deny_unknown_fields)]
pub struct CreateApiKey {
    #[deserr(default, error = DeserrJsonError<InvalidApiKeyDescription>)]
    pub description: Option<String>,
    #[deserr(default, error = DeserrJsonError<InvalidApiKeyName>)]
    pub name: Option<String>,
    #[deserr(default = Uuid::new_v4(), error = DeserrJsonError<InvalidApiKeyUid>, try_from(&String) = Uuid::from_str -> uuid::Error)]
    pub uid: KeyId,
    #[deserr(error = DeserrJsonError<InvalidApiKeyActions>, missing_field_error = DeserrJsonError::missing_api_key_actions)]
    pub actions: Vec<Action>,
    #[deserr(error = DeserrJsonError<InvalidApiKeyIndexes>, missing_field_error = DeserrJsonError::missing_api_key_indexes)]
    pub indexes: Vec<IndexUidPattern>,
    #[deserr(error = DeserrJsonError<InvalidApiKeyExpiresAt>, try_from(Option<String>) = parse_expiration_date -> ParseOffsetDateTimeError, missing_field_error = DeserrJsonError::missing_api_key_expires_at)]
    pub expires_at: Option<OffsetDateTime>,
}

impl CreateApiKey {
    pub fn to_key(self) -> Key {
        let CreateApiKey { description, name, uid, actions, indexes, expires_at } = self;
        let now = OffsetDateTime::now_utc();
        Key {
            description,
            name,
            uid,
            actions,
            indexes,
            expires_at,
            created_at: now,
            updated_at: now,
        }
    }
}

fn deny_immutable_fields_api_key(
    field: &str,
    accepted: &[&str],
    location: ValuePointerRef,
) -> DeserrJsonError {
    match field {
        "uid" => immutable_field_error(field, accepted, Code::ImmutableApiKeyUid),
        "actions" => immutable_field_error(field, accepted, Code::ImmutableApiKeyActions),
        "indexes" => immutable_field_error(field, accepted, Code::ImmutableApiKeyIndexes),
        "expiresAt" => immutable_field_error(field, accepted, Code::ImmutableApiKeyExpiresAt),
        "createdAt" => immutable_field_error(field, accepted, Code::ImmutableApiKeyCreatedAt),
        "updatedAt" => immutable_field_error(field, accepted, Code::ImmutableApiKeyUpdatedAt),
        _ => deserr::take_cf_content(DeserrJsonError::<BadRequest>::error::<Infallible>(
            None,
            deserr::ErrorKind::UnknownKey { key: field, accepted },
            location,
        )),
    }
}

#[derive(Debug, Deserr)]
#[deserr(error = DeserrJsonError, rename_all = camelCase, deny_unknown_fields = deny_immutable_fields_api_key)]
pub struct PatchApiKey {
    #[deserr(default, error = DeserrJsonError<InvalidApiKeyDescription>)]
    pub description: Setting<String>,
    #[deserr(default, error = DeserrJsonError<InvalidApiKeyName>)]
    pub name: Setting<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Key {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub uid: KeyId,
    pub actions: Vec<Action>,
    pub indexes: Vec<IndexUidPattern>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl Key {
    pub fn default_admin() -> Self {
        let now = OffsetDateTime::now_utc();
        let uid = Uuid::new_v4();
        Self {
            name: Some("Default Admin API Key".to_string()),
            description: Some("Use it for anything that is not a search operation. Caution! Do not expose it on a public frontend".to_string()),
            uid,
            actions: vec![Action::All],
            indexes: vec![IndexUidPattern::all()],
            expires_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn default_search() -> Self {
        let now = OffsetDateTime::now_utc();
        let uid = Uuid::new_v4();
        Self {
            name: Some("Default Search API Key".to_string()),
            description: Some("Use it to search from the frontend".to_string()),
            uid,
            actions: vec![Action::Search],
            indexes: vec![IndexUidPattern::all()],
            expires_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}

fn parse_expiration_date(
    string: Option<String>,
) -> std::result::Result<Option<OffsetDateTime>, ParseOffsetDateTimeError> {
    let Some(string) = string else { return Ok(None) };
    let datetime = if let Ok(datetime) = OffsetDateTime::parse(&string, &Rfc3339) {
        datetime
    } else if let Ok(primitive_datetime) = PrimitiveDateTime::parse(
        &string,
        format_description!(
            "[year repr:full base:calendar]-[month repr:numerical]-[day]T[hour]:[minute]:[second]"
        ),
    ) {
        primitive_datetime.assume_utc()
    } else if let Ok(primitive_datetime) = PrimitiveDateTime::parse(
        &string,
        format_description!(
            "[year repr:full base:calendar]-[month repr:numerical]-[day] [hour]:[minute]:[second]"
        ),
    ) {
        primitive_datetime.assume_utc()
    } else if let Ok(date) = Date::parse(
        &string,
        format_description!("[year repr:full base:calendar]-[month repr:numerical]-[day]"),
    ) {
        PrimitiveDateTime::new(date, time!(00:00)).assume_utc()
    } else {
        return Err(ParseOffsetDateTimeError(string));
    };
    if datetime > OffsetDateTime::now_utc() {
        Ok(Some(datetime))
    } else {
        Err(ParseOffsetDateTimeError(string))
    }
}
