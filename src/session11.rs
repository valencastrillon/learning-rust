/*
use tokio:: time ::{sleep, Duration};

async fn get_balance(){
    println!("Getting balance...");
    sleep(Duration::from_secs(2)).await;
    println!("Balance: $1000");
}

async fn get_transaction() {
    sleep(Duration::from_secs(1)).await;
    println!("Transaction: $500");
}

#[tokio::main]
async fn main() {
    let balance_future = get_balance();
    let transaction_future = get_transaction();

    tokio::join!(balance_future, transaction_future);
}
*/

//Coingecko API
use request :: Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let response = client.get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd")
        .header("User-Agent", "RustAsyncDemo/1.0 (contact: example@gmail.com)") //Aditional information
        .send()
        .await
        .unwrap();
        
    println!("Status: {}", response.status());
    let body = response.text().await.unwrap();
    println!("Response body: {}", body);
}
