use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::CrosstownBus;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

#[tokio::main]
async fn main() {
    let mut publisher = CrosstownBus::new_publisher("amqp://guest:guest@localhost:5672".to_owned()).unwrap();
    
    _ = publisher.send("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "1".to_owned(), 
        user_name: "2406495703-Amir".to_owned() 
    });
    _ = publisher.send("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "2".to_owned(), 
        user_name: "2406495703-Budi".to_owned() 
    });
    _ = publisher.send("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "3".to_owned(), 
        user_name: "2406495703-Cica".to_owned() 
    });
    _ = publisher.send("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "4".to_owned(), 
        user_name: "2406495703-Dira".to_owned() 
    });
    _ = publisher.send("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "5".to_owned(), 
        user_name: "2406495703-Emir".to_owned() 
    });
}
