use std::collections::HashMap;

pub fn decode_query(query: Option<&str>) -> HashMap<String, String> {
    query.map(|query|
        query.split("&")
        .map(|pair| pair.splitn(2, "="))
        .filter_map(|mut pair| {
            let key: String = pair.next().unwrap().into();
            if key.is_empty() {
                None
            } else {
                let val: String = pair.next().unwrap_or("").into();
                Some((key, val))
            }
        })
        .collect::<HashMap<String, String>>()
    )
    .unwrap_or(HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_empty_map_on_empty_string() {
        let result = decode_query(Some("".into()));
        assert_eq!(HashMap::new(), result);
    }

    #[test]
    fn should_split_tokens() {
        let result = decode_query(Some("foo=bar&abc=123"));
        assert_eq!(
            hashmap!{
                "foo".to_string() => "bar".to_string(),
                "abc".to_string() => "123".to_string()
            },
            result
        )
    }

    #[test]
    fn should_map_only_keys_to_empty_string() {
        let result = decode_query(Some("foo&bar="));
        assert_eq!(
            hashmap!{
                "foo".to_string() => "".to_string(),
                "bar".to_string() => "".to_string()
            },
            result
        )
    }
}
