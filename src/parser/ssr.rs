use crate::error;
use super::{parse_params, parse_path};
use std::collections::HashMap;
use std::convert::TryFrom;

/// Ssr 链接
/// ssr://server:port:protocol:method:obfs:password_base64/?params_base64
// obfsparam=obfsparam_base64&protoparam=protoparam_base64&remarks=remarks_base64&group=group_base64
//
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SsrUrl {
    config: SsrUrlConfig,
    params: SsrUrlParams,
}

impl TryFrom<&str> for SsrUrl {
    type Error = error::Error;

    fn try_from(value: &str) -> error::Result<SsrUrl> {
        if !value.starts_with("ssr://") {
            return Err(error::ErrorKind::InvaildSsrUrl(value.into()).into());
        }
        let (config, params) = match &value
            .trim_start_matches("ssr://")
            .split("/?")
            .collect::<Vec<_>>()[..]
        {
            &[v1, v2, ..] => (v1, v2),
            _ => return Err(error::ErrorKind::InvaildSsrUrl(value.into()).into()),
        };
        Ok(Self {
            config: SsrUrlConfig::try_from(config).map_err(|_e| {
                error::Error::from_kind(error::ErrorKind::InvaildSsrUrl(value.into()))
            })?,
            params: SsrUrlParams::try_from(params).map_err(|_e| {
                error::Error::from_kind(error::ErrorKind::InvaildSsrUrl(value.into()))
            })?,
        })
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SsrUrlConfig {
    pub server: String,
    pub port: u16,
    pub protocol: String,
    pub method: String,
    pub obfs: String,
    pub password: String,
}

impl<T: AsRef<str>> TryFrom<&[T]> for SsrUrlConfig {
    type Error = error::Error;

    fn try_from(value: &[T]) -> error::Result<Self> {
        let err = error::ErrorKind::Msg("无效的SSR路径".into());
        if value.len() != 6 {
            return Err(err.into());
        }
        Ok(Self {
            password: value[5].as_ref().to_string(),
            obfs: value[4].as_ref().to_string(),
            method: value[3].as_ref().to_string(),
            protocol: value[2].as_ref().to_string(),
            port: value[1]
                .as_ref()
                .parse()
                .map_err(|_e| error::Error::from_kind(err))?,
            server: value[0].as_ref().to_string(),
        })
    }
}

impl TryFrom<&str> for SsrUrlConfig {
    type Error = error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let vec = parse_path(value)?;
        Self::try_from(&vec)
    }
}

impl From<SsrUrlConfig> for String {
    fn from(value: SsrUrlConfig) -> Self {
        format!(
            "{}:{}:{}:{}:{}:{}",
            value.server,
            value.port,
            value.method,
            value.protocol,
            value.obfs,
            base64::encode_config(&value.password, base64::URL_SAFE_NO_PAD)
        )
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SsrUrlParams {
    pub obfs_param: Option<String>,
    pub proto_param: Option<String>,
    pub remarks: Option<String>,
    pub group: Option<String>,
}

impl<T: AsRef<str>> TryFrom<&HashMap<String, T>> for SsrUrlParams {
    type Error = error::Error;
    fn try_from(value: &HashMap<String, T>) -> error::Result<Self> {
        Ok(Self {
            obfs_param: value.get("obfsparam").map(|v| v.as_ref().to_string()),
            proto_param: value.get("protoparam").map(|v| v.as_ref().to_string()),
            remarks: value.get("remarks").map(|v| v.as_ref().to_string()),
            group: value.get("group").map(|v| v.as_ref().to_string()),
        })
    }
}

impl TryFrom<&str> for SsrUrlParams {
    type Error = error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let params = parse_params(value)?;
        Self::try_from(&params)
    }
}

impl From<SsrUrlParams> for String {
    fn from(params: SsrUrlParams) -> Self {
        format!(
            "bofsparam={}&protoparam={}&remarks={}&group={}",
            params.obfs_param.unwrap_or("".into()),
            params.proto_param.unwrap_or("".into()),
            params.remarks.unwrap_or("".into()),
            params.group.unwrap_or("".into())
        )
    }
}
