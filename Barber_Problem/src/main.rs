use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::Duration;

struct BarberShop {
    semaphore: Arc<Semaphore>,                // Manejo de sillas disponibles
    max_chairs: u32,                          // Máximo de sillas
    customers_waiting: Arc<Mutex<u32>>,       // Clientes esperando
    remaining_customers: Arc<Mutex<u32>>,     // Clientes totales restantes
}

impl BarberShop {
    fn new(max_chairs: u32, total_customers: u32) -> Self {
        BarberShop {
            semaphore: Arc::new(Semaphore::new(max_chairs as usize)),
            max_chairs,
            customers_waiting: Arc::new(Mutex::new(0)),
            remaining_customers: Arc::new(Mutex::new(total_customers)),
        }
    }

    async fn arrive_customer(&self, id: u32) {
        let mut customers_waiting = self.customers_waiting.lock().await;

        if *customers_waiting < self.max_chairs {
            self.semaphore.acquire().await.unwrap();
            *customers_waiting += 1;
            println!(
                "Cliente {} está esperando. Total clientes esperando: {}",
                id, *customers_waiting
            );

            if *customers_waiting == 1 {
                println!("Cliente {} despierta al barbero.", id);
            }
        } else {
            println!("No hay espacio para el cliente {}. Se va de la barbería.", id);
        }
    }

    async fn cut_hair(&self) {
        loop {
            let mut customers_waiting = self.customers_waiting.lock().await;

            if *customers_waiting == 0 {
                println!("El barbero está durmiendo, esperando clientes...");
                drop(customers_waiting);
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }

            *customers_waiting -= 1;
            println!(
                "El barbero está cortando el cabello. Clientes esperando: {}",
                *customers_waiting
            );
            drop(customers_waiting);

            tokio::time::sleep(Duration::from_secs(2)).await;
            println!("El barbero ha terminado de cortar el cabello.");

            self.semaphore.add_permits(1);

            let mut remaining_customers = self.remaining_customers.lock().await;
            *remaining_customers -= 1;

            if *remaining_customers == 0 {
                println!("El barbero ha terminado de atender a todos los clientes.");
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let total_customers = 8; // Simulación con más clientes para forzar acumulación
    let shop = Arc::new(BarberShop::new(3, total_customers)); // 3 sillas

    let barber_shop = Arc::clone(&shop);
    let barber_thread = tokio::spawn(async move {
        barber_shop.cut_hair().await;
    });

    let mut client_threads = vec![];

    for i in 1..=total_customers {
        let shop = Arc::clone(&shop);
        let client_thread = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(i as u64)).await; // Llegadas escalonadas
            println!("Cliente {} ha llegado.", i);
            shop.arrive_customer(i).await;
        });
        client_threads.push(client_thread);
    }

    for client in client_threads {
        client.await.unwrap();
    }

    barber_thread.await.unwrap();
}
