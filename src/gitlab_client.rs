use std::error::Error;
use std::iter::Map;

use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug)]
struct GitLab {
    base_url: String,
    api_token: String,
    proxy: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProtectedBranches {
    branches: Vec<Branch>
}

#[derive(Serialize, Deserialize, Debug)]
struct Branch {
    id: u16,
    name: String,
    #[serde(skip)]
    push_access_levels: Vec<Map<String, String>>,
    #[serde(skip)]
    merge_access_levels: Vec<Map<String, String>>,
    allow_force_push: bool,
    code_owner_approval_required: bool,
}

impl GitLab {
    fn build_client(&self) -> reqwest::Result<Client> {
        let client = reqwest::ClientBuilder::new()
            .proxy(reqwest::Proxy::https(&self.proxy)?)
            .build();

        return client;
    }

    fn get_protected_branches_project(&self, project_id: &str) -> Result<ProtectedBranches, Box<dyn Error>> {
        let client_res = self.build_client();
        if client_res.is_err() {
            let err: Box<dyn Error> = Box::from(client_res.unwrap_err());
            return Result::Err(err);
        }

        let client = client_res.unwrap();
        let endpoint_url = concat!(self.base_url, "/projects/", project_id, "/protected_branches" );

        async {
            let res = client.get(endpoint_url).send().await?;
        }


        return Result::Ok(ProtectedBranches { branches: Vec::new() });
    }
}
