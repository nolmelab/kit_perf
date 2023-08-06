
struct Server; 

struct Client;


impl Server {

    pub fn new() -> Self {
        Server{}
    }

}

impl Client {

    pub fn new() -> Self {
        Client{}
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn understand_split() {



    }
}