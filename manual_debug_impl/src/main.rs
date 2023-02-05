#![allow(unused_variables)]
#![allow(dead_code)]

// Debug again
use std::fmt;
use client::InternetClient;

// External code(외부 Crate)
mod client {
    pub struct InternetClient {
        pub client_id: u32, // Other stuff
    }
}

struct Customer<'a> {
    money: u32,
    name: &'a str,
    client: &'a InternetClient
}

// 외부 Crate를 사용하는 중 Debug가 구현되어 있지 않을 경우 다음과 같이 구현할 수 있다.
impl fmt::Debug for Customer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Customer")
         .field("money", &self.money)
         .field("name", &self.name)
         .field("Client", &"Client")
         .finish()
    }
}

fn main() {
    let client = client::InternetClient {
        client_id: 0
    };

    let customer1 = Customer {
        money: 6876,
        name: "Billy",
        client: &client
    };

    println!("{customer1:?}");
}
