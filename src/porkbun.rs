use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
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

#[allow(dead_code)]
pub trait PorkbunAPI {
    async fn create_record(
        &self,
        subdomain: &str,
        record_type: &str,
        content: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_record(
        &self,
        subdomain: &str,
        record_type: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;
    async fn update_record(
        &self,
        subdomain: &str,
        record_type: &str,
        content: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_record(
        &self,
        subdomain: &str,
        record_type: &str,
        content: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

#[derive(Clone)]
pub struct Porkbun {
    base_url: String,
    client: Client,
    domain: String,
    auth: Auth,
}

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
}

impl PorkbunAPI for Porkbun {
    async fn create_record(
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

    async fn get_record(
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

    async fn update_record(
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

    async fn delete_record(
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

#[derive(Clone)]
#[cfg(test)]
pub struct MockPorkbun {
    domain: String,
}

#[cfg(test)]
impl MockPorkbun {
    pub fn new(domain: String) -> Self {
        MockPorkbun { domain }
    }

    fn fqdn(&self, subdomain: &str) -> String {
        format!("{}.{}", subdomain, self.domain)
    }
}

#[cfg(test)]
impl PorkbunAPI for MockPorkbun {
    async fn create_record(
        &self,
        subdomain: &str,
        _record_type: &str,
        _content: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let fqdn = self.fqdn(subdomain);
        Ok(format!("create_record: {}", fqdn))
    }

    async fn get_record(
        &self,
        subdomain: &str,
        _record_type: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let fqdn = self.fqdn(subdomain);
        Ok(format!("get_record: {}", fqdn))
    }

    async fn update_record(
        &self,
        subdomain: &str,
        _record_type: &str,
        _content: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let fqdn = self.fqdn(subdomain);
        Ok(format!("update_record: {}", fqdn))
    }

    async fn delete_record(
        &self,
        subdomain: &str,
        _record_type: &str,
        _content: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let fqdn = self.fqdn(subdomain);
        Ok(format!("delete_record: {}", fqdn))
    }
}
