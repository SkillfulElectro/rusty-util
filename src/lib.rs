pub fn is_valid_ipv4_port(input: &String) -> bool {
    // Dereference the String to &str and split into parts using ':'
    let parts: Vec<&str> = input.split(':').collect();
    
    // Check if there are exactly two parts (IPv4 address and port)
    if parts.len() != 2 {
        return false;
    }

    // Validate IPv4 address part
    let ip_addr = parts[0];
    if !is_valid_ipv4(ip_addr) {
        return false;
    }

    // Validate port number part
    let port = parts[1];
    match port.parse::<u16>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_valid_ipv4(ip: &str) -> bool {
    // Validate IPv4 address format
    if let Ok(addr) = ip.parse::<std::net::Ipv4Addr>() {
        // Check if parsed address matches the original string
        addr.to_string() == ip
    } else {
        false
    }
}

pub fn search_vec<T: PartialEq>(arr : &Vec<T> , key : &T) -> i32{
    for index in 0..arr.len(){
        if arr[index] == *key{
            return index.try_into().unwrap();
        }
    }
    
    return -1;
}