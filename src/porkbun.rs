use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Auth {
    apikey: String,
    secretapikey: String,
}

#[derive(Serialize, Deserialize)]
struct AuthCreateContent {
    apikey: String,
    secretapikey: String,
    name: String,
    r#type: String,
    content: String,
    ttl: usize,
}

#[derive(Serialize, Deserialize)]
struct AuthContent {
    apikey: String,
    secretapikey: String,
    content: String,
    ttl: usize,
}

pub struct Porkbun {
    base_url: String,
    client: Client,
    domain: String,
    auth: Auth,
}

#[allow(dead_code)]
impl Porkbun {
    pub fn new(api_key: String, secret_key: String, domain: String) -> Self {
        let client = Client::builder().build().unwrap();
        let auth = Auth {
            apikey: api_key,
            secretapikey: secret_key,
        };
        Porkbun {
            base_url: String::from("https://porkbun.com/api/json/v3"),
            client,
            domain,
            auth,
        }
    }

    pub async fn create_record(
        &self,
        subdomain: &str,
        record_type: &str,
        content: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/dns/create/{}", self.base_url, self.domain);
        let auth_content = serde_json::to_string(&AuthCreateContent {
            apikey: self.auth.apikey.clone(),
            secretapikey: self.auth.secretapikey.clone(),
            name: subdomain.to_string(),
            r#type: record_type.to_string(),
            content: content.to_string(),
            ttl: 300,
        })?;
        let response = self.client.post(url).body(auth_content).send().await?;
        let text = response.text().await?;
        Ok(text)
    }

    pub async fn get_record(
        &self,
        subdomain: &str,
        record_type: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/dns/retrieveByNameType/{}/{}/{}",
            self.base_url, self.domain, record_type, subdomain
        );
        let auth = serde_json::to_string(&self.auth)?;
        let response = self.client.post(url).body(auth).send().await?;
        let text = response.text().await?;
        Ok(text)
    }

    pub async fn update_record(
        &self,
        subdomain: &str,
        record_type: &str,
        content: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/dns/editByNameType/{}/{}/{}",
            self.base_url, self.domain, record_type, subdomain
        );
        let auth_content = serde_json::to_string(&AuthContent {
            apikey: self.auth.apikey.clone(),
            secretapikey: self.auth.secretapikey.clone(),
            content: content.to_string(),
            ttl: 300,
        })?;
        let response = self.client.post(url).body(auth_content).send().await?;
        let text = response.text().await?;
        Ok(text)
    }

    pub async fn delete_record(
        &self,
        subdomain: &str,
        record_type: &str,
        content: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/dns/deleteByNameType/{}/{}/{}",
            self.base_url, self.domain, record_type, subdomain
        );
        let auth_content = serde_json::to_string(&AuthContent {
            apikey: self.auth.apikey.clone(),
            secretapikey: self.auth.secretapikey.clone(),
            content: content.to_string(),
            ttl: 300,
        })?;
        let response = self.client.post(url).body(auth_content).send().await?;
        let text = response.text().await?;
        Ok(text)
    }
}
