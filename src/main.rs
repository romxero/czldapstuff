
// chatgpt went with the asynchronous junk. Gonna have to rip that out.

use ldap3::{LdapConnAsync, Scope, SearchEntry};
use tokio;


fn main()-> ldap3::result::Result<()> {
    
    // testing the ldap stuff here 
    println!("Running a test for the ldap stuff on czbiohub sf");
    
    
    // LDAP server URL
    let ldap_url = "ldap://domcon02.czbiohub.org:389";
    // Bind DN and password
    let bind_dn =  "DC=czbiohub,DC=org";
    let password = "thankschatgpt?";

    // Connect to the LDAP server
    let (conn, mut ldap) = LdapConnAsync::new(ldap_url).await?;
    ldap3::drive!(conn);

    // Bind to the LDAP server
    ldap.simple_bind(bind_dn, password).await?.success()?;

    // make sure to look at the base for biohub
    let search_base = "OU=BiohubSecurityGroups,DC=czbiohub,DC=org";
    let search_filter = "(uid=randall.white)";

    // Perform the search
    let (rs, _) = ldap
        .search(
            search_base,
            Scope::Subtree,
            search_filter,
            vec!["dn", "cn", "mail"],
        )
        .await?
        .success()?;

    // Process the search results
    for entry in rs.into_iter().map(SearchEntry::construct) {
        println!("DN: {}", entry.dn);
        if let Some(cn) = entry.attrs.get("cn") {
            println!("CN: {:?}", cn);
        }
        if let Some(mail) = entry.attrs.get("mail") {
            println!("Email: {:?}", mail);
        }
    }

    // Unbind from the server
    ldap.unbind().await?;

    Ok(())


}

