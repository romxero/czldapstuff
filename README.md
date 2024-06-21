# CZBiohub Rust Juhnk
* This is just some rust work for ldap3 parsing @ czbiohubsf
* seed started with chatgpt 

```
Explanation:

    Dependencies: The ldap3 crate is used for LDAP operations, and tokio is used for asynchronous programming.
    LDAP Connection: The code connects to the LDAP server using the provided URL.
    Binding: It binds to the LDAP server using the bind DN and password.
    Search: It performs a search with the specified base and filter. The search retrieves the dn, cn, and mail attributes of the matching entries.
    Result Processing: The results are processed and printed.
    Unbind: Finally, the connection is unbound from the LDAP server.

Make sure to replace ldap://example.com:389, cn=admin,dc=example,dc=com, and adminpassword with the actual values for your LDAP server. Similarly, adjust the search base and filter as needed for your LDAP directory.

```
