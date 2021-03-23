use std::error::Error;
use std::iter::Map;

use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GitLab {
    url: String,
    api_token: String,
    proxy: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ProtectedBranches {
    branches: Vec<Branch>
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Branch {
    id: u16,
    name: String,
    #[serde(skip)]
    push_access_levels: Vec<Map<String, String>>,
    #[serde(skip)]
    merge_access_levels: Vec<Map<String, String>>,

//    allow_force_push: bool,
//    code_owner_approval_required: bool,
}

impl GitLab {
    pub fn build_client(&self) -> reqwest::Result<Client> {
        let client = reqwest::blocking::ClientBuilder::new()
            .proxy(reqwest::Proxy::http(&self.proxy)?)
            .proxy(reqwest::Proxy::https(&self.proxy)?)
            .build();

        return client;
    }

    pub fn get_protected_branches_project(&self, project_id: &str) -> Result<ProtectedBranches, Box<dyn Error>> {
        let client_res = self.build_client();
        if client_res.is_err() {
            let err: Box<dyn Error> = Box::from(client_res.unwrap_err());
            return Result::Err(err);
        }

        let client = client_res.unwrap();
        let endpoint_url = [&self.url,
            "/api/v4/",
            "/projects/",
            project_id,
            "/protected_branches"
        ]
            .join("");
        let res = client.get(endpoint_url).header("PRIVATE-TOKEN",
                                                  &self.api_token).send();

        if res.is_err() {
            println!("{:?}", res.unwrap_err());
            panic!("Could not query GitLab API, please check your proxy/network connectivity");
        }

        let response = res.unwrap();
        let brs: Vec<Branch> = response.json().unwrap();

        return Result::Ok(ProtectedBranches { branches: brs });
    }
}
