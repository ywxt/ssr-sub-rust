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
            let group: Vec<_> = param.split('=').filter(|item| !item.is_empty()).collect();
            if group.len() != 2 {
                return Err(error::ErrorKind::InvalidSsrParam(query.into()).into());
            }
            match base64::decode_config(&group[1], base64::URL_SAFE_NO_PAD) {
                Ok(result) => match String::from_utf8(result) {
                    Ok(result) => Ok((group[0].to_string(), result)),
                    Err(_) => Err(error::ErrorKind::InvalidSsrParam(query.into()).into()),
                },
                Err(_) => Err(error::ErrorKind::InvalidSsrParam(query.into()).into()),
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error;

    #[test]
    fn test_get_ssr_path() -> error::Result<()> {
        let password = base64::encode_config("Hello", base64::URL_SAFE_NO_PAD);
        let base =
            String::from("127.0.0.1:8080:default_proto:default_method:default_obfs:") + &password;
        let vecs = parse_path(&base)?;
        assert_eq!(vecs.len(), 6);
        assert_eq!(vecs[0], "127.0.0.1");
        assert_eq!(vecs[1], "8080");
        assert_eq!(vecs[2], "default_proto");
        assert_eq!(vecs[3], "default_method");
        assert_eq!(vecs[4], "default_obfs");
        assert_eq!(vecs[5], "Hello");
        Ok(())
    }

    #[test]
    fn test_get_ssr_params() -> error::Result<()> {
        let group = "Test Group";
        let remark = "Test remark";
        let param = format!(
            "{}={}&{}={}",
            "group",
            base64::encode_config(group, base64::URL_SAFE_NO_PAD),
            "remark",
            base64::encode_config(remark, base64::URL_SAFE_NO_PAD)
        );
        let map = parse_params(&param)?;
        assert_eq!(map.len(), 2);
        assert_eq!(map["group"], "Test Group");
        assert_eq!(map["remark"], "Test remark");
        Ok(())
    }
}
