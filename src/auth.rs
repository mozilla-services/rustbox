use std::collections::HashMap;
use serde::json;

struct Validate {
    fxa_url: String,    // FxA Verifier URL
    token: String,      // Authorization Header token
}

impl Validate {
    fn new(auth_header:String, config: HashMap) -> Validate {
        Validate {
            fxa_url: "",
            token: ""
        }
    }

    fn verify(auth_header:String) -> bool {
        /// Verify a given action ("read", "write")
        let mut splitter = auth_header.splitn(1, " ");
        let schema = splitter.next().expect("Missing schema");
        let token = splitter.next().expect("Missing token");
        // Get the scopes from the verify server.

        return false
    }
}
