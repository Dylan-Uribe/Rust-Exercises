use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::Duration;

struct BarberShop{
    semaphore: Arc<Semaphore>,
    max_chairs: u32,
    customers_in_waiting: Arc<Mutex<u32>>,
    remaining_customers: Arc<Mutex<u32>>
}

impl BarberShop{
    fn new(max_chairs: u32, total_customers: u32) -> Self{
        BarberShop{
            semaphore: Arc::new(Semaphore::new(max_chairs as usize)),
            max_chairs,
            customers_in_waiting: Arc::new(Mutex::new(0)),
            remaining_customers: Arc::new(Mutex::new(total_customers)),
        }
    }

    async fn arrive_customers(&self, id: u32){

        let mut customers = self.customers_in_waiting.lock().await;

        if *customers < self.max_chairs {
            self.semaphore.acquire().await.unwrap();
            *customers += 1;
            println!("Customers waiting #{}", *customers);
        }
        else{
            println!("No free space for customer #{}", id);
        }

        if *customers == 1 && self.max_chairs > 0{
            println!("Customer {} woke up the barber", id);
        }

    }

    async fn cut_hair(&self){
        loop {
            let mut customers = self.customers_in_waiting.lock().await;

            if *customers == 0{
                println!("the barber is sleeping, waiting for customers.");
                drop(customers);
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }

            *customers -= 1;
            println!("The barber is cutting hair. {} customers are waiting.", *customers);
            drop(customers);
            tokio::time::sleep(Duration::from_secs(2)).await;

            println!("The barber has finished cutting hair.");

            let mut remaining_customers = self.remaining_customers.lock().await;
            *remaining_customers -= 1;

            if *remaining_customers == 0{
                println!("The barber has finished cutting hair for all customers.");
                break;
            }
        }
    }
}


#[tokio::main]
async fn main() {

    let total_customers = 5;
    let shop = Arc::new(BarberShop::new(3, total_customers));

    let barber_shop = Arc::clone(&shop);
    let barber_thread = tokio::spawn(async move {
        barber_shop.cut_hair().await;
    });

    let mut client_threads = vec![];

    for i in 1..=total_customers{
        let shop = Arc::clone(&shop);
        let client_thread = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(((i - 1) * 2) as u64)).await;
            println!("The customer {} has arrived.", i);
            shop.arrive_customers(i).await;
            tokio::time::sleep(Duration::from_secs(2)).await;
        });
        client_threads.push(client_thread);
    }

    for client in client_threads{
        client.await.unwrap();
    }

    barber_thread.await.unwrap();
}
