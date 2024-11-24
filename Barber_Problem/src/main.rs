use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::Duration;

/// Represents a barber shop with a limited number of chairs and customers.
struct BarberShop {
    semaphore: Arc<Semaphore>,                // Semaphore to manage available chairs
    max_chairs: u32,                          // Maximum number of chairs
    customers_waiting: Arc<Mutex<u32>>,       // Number of customers waiting
    remaining_customers: Arc<Mutex<u32>>,     // Total remaining customers
}

impl BarberShop {
    /// Creates a new `BarberShop` with the specified number of chairs and total customers.
    ///
    /// # Arguments
    ///
    /// * `max_chairs` - The maximum number of chairs available in the barber shop.
    /// * `total_customers` - The total number of customers expected.
    fn new(max_chairs: u32, total_customers: u32) -> Self {
        BarberShop {
            semaphore: Arc::new(Semaphore::new(max_chairs as usize)),
            max_chairs,
            customers_waiting: Arc::new(Mutex::new(0)),
            remaining_customers: Arc::new(Mutex::new(total_customers)),
        }
    }

    /// Handles the arrival of a customer.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the arriving customer.
    async fn arrive_customer(&self, id: u32) {
        let mut customers_waiting = self.customers_waiting.lock().await;

        if *customers_waiting < self.max_chairs {
            let _ = self.semaphore.acquire().await.unwrap();
            *customers_waiting += 1;
            println!(
                "Customer {} is waiting. Total customers waiting: {}",
                id, *customers_waiting
            );

            if *customers_waiting == 1 {
                println!("Customer {} wakes up the barber.", id);
            }
        } else {
            println!("No space for customer {}. Leaving the barber shop.", id);
        }
    }

    /// Simulates the barber cutting hair.
    async fn cut_hair(&self) {
        loop {
            let mut customers_waiting = self.customers_waiting.lock().await;

            if *customers_waiting == 0 {
                println!("The barber is sleeping, waiting for customers...");
                drop(customers_waiting);
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }

            *customers_waiting -= 1;
            println!(
                "The barber is cutting hair. Customers waiting: {}",
                *customers_waiting
            );
            drop(customers_waiting);

            tokio::time::sleep(Duration::from_secs(2)).await;
            println!("The barber has finished cutting hair.");

            let mut remaining_customers = self.remaining_customers.lock().await;
            *remaining_customers -= 1;

            if *remaining_customers == 2 {
                println!("The barber has finished cutting hair for all customers.");
                break;
            }

            self.semaphore.add_permits(1);
        }
    }
}

#[tokio::main]
async fn main() {
    let total_customers = 8; // Simulate with more customers to force accumulation
    let shop = Arc::new(BarberShop::new(3, total_customers)); // 3 chairs

    let barber_shop = Arc::clone(&shop);
    let barber_thread = tokio::spawn(async move {
        barber_shop.cut_hair().await;
    });

    let mut client_threads = vec![];

    for i in 1..=total_customers {
        let shop = Arc::clone(&shop);
        let client_thread = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(i as u64)).await; // Staggered arrivals
            println!("Customer {} has arrived.", i);
            shop.arrive_customer(i).await;
        });
        client_threads.push(client_thread);
    }

    for client in client_threads {
        client.await.unwrap();
    }

    barber_thread.await.unwrap();
}

/* The code above creates a  BarberShop  struct that represents a barber shop with a limited number of chairs and customers. The  arrive_customer  method is used to handle the arrival of a customer, while the  cut_hair  method simulates the barber cutting hair.
 The  main  function creates a new  BarberShop  instance with 3 chairs and 8 total customers. It then spawns a barber thread to cut hair and multiple client threads to simulate customers arriving at the barber shop.
 To run the code, execute the following command:
 cargo run */
