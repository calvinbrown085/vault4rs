use reqwest::{Client, Url, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct VaultSecret<T> {
    data: T,
    lease_duration: i64,
    lease_id: String,
    renewable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct VaultToken {
    client_token: String,
    lease_duration: i64,
    renewable: bool,
}

//            /**
//               * https://www.vaultproject.io/api/auth/approle/index.html#login-with-approle
//               */
//            def login[F[_]](client: Client[F], vaultUri: Uri)(roleId: String)(implicit F: Sync[F]): F[VaultToken] = {
//            val request = Request[F](
//            method = Method.POST,
//    uri = vaultUri / "v1" / "auth" / "approle" / "login"
//    ).withEntity(Json.obj(("role_id", Json.fromString(roleId))))
//    for {
//    json <- F.handleErrorWith(client.expect[Json](request)
//    ) { e =>
//    F.raiseError(VaultRequestError(request, e.some, s"roleId=$roleId".some))
//    }
//    token <- raiseKnownError(json.hcursor.get[VaultToken]("auth"))(decoderError)
//    } yield token
//            }
fn login(client: Client, base_vault_url: Url) -> VaultToken {

    let url: Url = base_vault_url.join(&format!("/v1/{}", secret_path)).unwrap();
    let request: RequestBuilder = client.get(url).header("X-Vault-Token", token);
    let resp: String = request.send().unwrap().json().unwrap();

    serde_json::from_str(&resp).unwrap()

}

fn read_secret(client: Client, base_vault_url: Url, token: &str, secret_path: &str) -> VaultSecret<String> {
    let url: Url = base_vault_url.join(&format!("/v1/{}", secret_path)).unwrap();
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
