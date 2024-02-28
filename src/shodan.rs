use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::Error as ReqwestError;

pub fn ip_lookup(ip: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Get the open ports, vulnerabilities and other information for an IP (https://internetdb.shodan.io/docs#/default/info__ip__get)
        Parameters:
            ip (str): The IP address for which the lookup is performed.
        Returns:
            Result<Response, reqwest::Error>: The result of the GET request, either containing the response or an error.
    */
    let client = Client::new();
    let url = format!("https://internetdb.shodan.io/{}", ip);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn ip_or_hostname_ping(ip_or_hostname: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Ping an IP or hostname (https://geonet.shodan.io/docs#/default/ping_api_ping__ip__get)
        Parameters:
            ip_or_hostname (str): The IP address or Hostname that's getting pinged.
        Returns:
            Result<Response, reqwest::Error>: The result of the GET request, either containing the response or an error.
    */
    let client = Client::new();
    let url = format!("https://geonet.shodan.io/api/ping/{}", ip_or_hostname);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn ip_or_hostname_geoping(ip_or_hostname: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            GeoPing an IP or hostname (https://geonet.shodan.io/docs#/default/geoping_api_geoping__ip__get)
        Parameters:
            ip_or_hostname (str): The IP address or Hostname that's getting geopinged.
        Returns:
            Result<Response, reqwest::Error>: The result of the GET request, either containing the response or an error.
    */
    let client = Client::new();
    let url = format!("https://geonet.shodan.io/api/geoping/{}", ip_or_hostname);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn dns_query(hostname: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Querying the hostname (https://geonet.shodan.io/docs#/default/dns_query_api_dns__hostname__get)
        Parameters:
            hostname (str): The hostname getting queried.
        Returns:
            Result<Response, reqwest::Error>: The result of the GET request, either containing the response or an error.
    */
    let client = Client::new();
    let url = format!("https://geonet.shodan.io/api/dns/{}", hostname);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn geo_dns_query(hostname: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Geo Querying the hostname (https://geonet.shodan.io/docs#/default/geo_dns_query_api_geodns__hostname__get)
        Parameters:
            hostname (str): The hostname getting queried.
        Returns:
            Result<Response, reqwest::Error>: The result of the GET request, either containing the response or an error.
    */
    let client = Client::new();
    let url = format!("https://geonet.shodan.io/api/geodns/{}", hostname);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn entity_id_lookup(entity_id: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Look up an entity with its associated ID (https://entitydb.shodan.io/)
        Parameters:
            entity_id (str): The entity id getting queried.
        Returns:
            Result<Response, reqwest::Error>: The result of the GET request, either containing the response or an error.
    */
    let client = Client::new();
    let url = format!("https://entitydb.shodan.io/api/entities/{}", entity_id);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn entity_symbol_lookup(entity_symbol: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Look up an entity with its associated symbol (https://entitydb.shodan.io/)
        Parameters:
            entity_symbol (str): The entity symbol getting queried.
        Returns:
            Result<Response, reqwest::Error>: The result of the GET request, either containing the response or an error.
    */
    let client = Client::new();
    let url = format!("https://entitydb.shodan.io/api/entities/symbol/{}", entity_symbol);
    let response = client.get(&url).send()?;
    Ok(response)
}