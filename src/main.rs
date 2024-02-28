mod shodan;
mod helpers;

fn main() {
    // InternetDB API - Fast IP Lookups (https://internetdb.shodan.io/)
    let ip_to_query = "1.1.1.1"; 


    println!("InternetDB API\n\n[+] Function currently being runned: ip_lookup()");
    helpers::handle_error_shodan(shodan::ip_lookup(ip_to_query)); 

    println!("\n--------------------------------------------------\n");


    // GeoNet - Geographic Network Tools (https://geonet.shodan.io/)
    let hostname_to_query = "google.com";

    println!("GeoNet API\n\n[+] Function currently being runned: ip_or_hostname_ping()");
    helpers::handle_error_shodan(shodan::ip_or_hostname_ping(hostname_to_query)); 
    
    println!("\n[+] Function currently being runned: ip_or_hostname_geoping()");
    helpers::handle_error_shodan(shodan::ip_or_hostname_geoping(hostname_to_query));
    
    println!("\n[+] Function currently being runned: dns_query()");
    helpers::handle_error_shodan(shodan::dns_query(hostname_to_query)); 
    
    println!("\n[+] Function currently being runned: geo_dns_query()");
    helpers::handle_error_shodan(shodan::geo_dns_query(hostname_to_query)); 

    println!("\n--------------------------------------------------\n");


    // EntityDB API - Fast Business Entities Lookups for Financial and Executives Overview
    let entity_id = "5";

    println!("\n[+] Function currently being runned: entity_id_lookup()");
    helpers::handle_error_shodan(shodan::entity_id_lookup(entity_id));

    let entity_symbol = "NVDA";

    println!("\n[+] Function currently being runned: entity_symbol_lookup()");
    helpers::handle_error_shodan(shodan::entity_symbol_lookup(entity_symbol));

    println!("\n--------------------------------------------------\n");
}