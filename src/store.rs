
#[derive(Serialize, Deserialize, Debug)]
struct Credentials {
    user: String,
    password: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PushData {
    uaid: String,
    device_id: String,
    service: String,
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Storage {
    filename: String,           // Storage location identifier (s3 name, etc.)
    content_type: String,       // Type of data storage (text/plain, application/json, etc.)
    ttl: u64,                   // UTC of expiration
    _io: None,                  // TODO: Placeholder for IO handle.
}


impl Storage {
    fn new(credentials:Credentials) -> Storage {
        // TODO: use credential info to initialize IO Handle.
        return Storage {
            filename: "",
            content_type: "",
            ttl: 0,
            _io: None
        }
    }

    fn read() -> Result<String, Error>{
        // TODO Return the content of the IO Handle
    }

    fn write(data: String) -> Result<Bool, Error>{
        // TODO: Write the data (should be uBuffer) to IO Handle
    }

    fn delete() -> Result<Bool, Error>{
        // TODO: Delete the buffer
    }
}
