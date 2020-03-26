use crate::error;
use std::collections::HashMap;

pub fn parse_path(path: &str) -> error::Result<Vec<String>> {
    let paths: Vec<&str> = path.split(':').collect();
    if paths.len() != 6 {
        return Err(error::ErrorKind::InvalidSsrPath(path.into()).into());
    }
    // 密码解码
    match base64::decode_config(paths[5], base64::URL_SAFE_NO_PAD) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(password) => {
                let mut config: Vec<String> = paths[..=4].iter().map(|v| v.to_string()).collect();
                config.push(password);
                Ok(config)
            }
            Err(_) => Err(error::ErrorKind::InvalidSsrPath(path.into()).into()),
        },
        Err(_) => Err(error::ErrorKind::InvalidSsrPath(path.into()).into()),
    }
}

pub fn parse_params(query: &str) -> error::Result<HashMap<String, String>> {
    let params_group: Vec<&str> = query.split('&').collect();
    params_group
        .iter()
        .map(|param| {
            let group: Vec<_> = param.split('=').filter(|item| item.is_empty()).collect();
            if group.len() != 2 {
                return Err(error::ErrorKind::InvalidSsrParam(query.into()).into());
            }
            match base64::decode_config(&group[1], base64::URL_SAFE_NO_PAD) {
                Ok(result) => match String::from_utf8(result) {
                    Ok(_) => Ok((group[0].to_string(), group[1].to_string())),
                    Err(_) => Err(error::ErrorKind::InvalidSsrParam(query.into()).into()),
                },
                Err(_) => Err(error::ErrorKind::InvalidSsrParam(query.into()).into()),
            }
        })
        .collect()
}
