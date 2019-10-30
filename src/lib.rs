use reqwest::{Client, RequestBuilder, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct VaultSecret<T> {
    data: T,
    lease_duration: i64,
    lease_id: String,
    renewable: bool,
}

fn read_secret(client: Client, mut url: Url, token: &str, secret_path: &str) -> VaultSecret<String> {
    url.set_path(&format!("/v1/{}", secret_path));
    let request: RequestBuilder = client.get(url).header("X-Vault-Token", token);
    let resp: String = request.send().unwrap().json().unwrap();

    serde_json::from_str(&resp).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
