use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::Duration;

struct BarberShop {
    semaphore: Arc<Semaphore>,        // Semáforo que limita el número de clientes esperando
    max_chairs: u32,                  // Número máximo de sillas disponibles
    customers_in_waiting: Arc<Mutex<u32>>, // Número de clientes esperando
    remaining_customers: Arc<Mutex<u32>>,  // Contador de clientes restantes por atender
}

impl BarberShop {
    fn new(max_chairs: u32, total_customers: u32) -> Self {
        BarberShop {
            semaphore: Arc::new(Semaphore::new(max_chairs as usize)),
            max_chairs,
            customers_in_waiting: Arc::new(Mutex::new(0)),
            remaining_customers: Arc::new(Mutex::new(total_customers)), // Inicializamos con el total de clientes
        }
    }

    async fn arrive_customer(&self, id: u32) {
        // Bloqueamos el mutex para acceder al número de clientes esperando
        let mut customers = self.customers_in_waiting.lock().await;
        // Si hay espacio, el cliente ocupa una silla
        if *customers < self.max_chairs {
            self.semaphore.acquire().await.unwrap(); // Intentamos obtener una silla (semáforo)
            *customers += 1;
            println!("Cliente {} ha llegado. Hay {} clientes esperando.", id, *customers);
        } else {
            // Si no hay espacio, el cliente se va
            println!("Cliente {} no encontró espacio. Se va.", id);
        }

        // Si el barbero estaba durmiendo y un cliente llega, lo despierta
        if *customers == 1 { // El primer cliente despierta al barbero
            println!("Cliente {} ha despertado al barbero.", id);
        }
    }

    async fn cut_hair(&self) {
        loop {
            // Bloqueamos el mutex para acceder al número de clientes esperando
            let mut customers = self.customers_in_waiting.lock().await;

            if *customers == 0 {
                // El barbero duerme si no hay clientes
                println!("El barbero está durmiendo, esperando clientes...");
                // Liberamos el mutex antes de dormir
                drop(customers);
                tokio::time::sleep(Duration::from_secs(1)).await; // El barbero "duerme"
                continue; // Volvemos al inicio para intentar otra vez
            }

            // Si hay un cliente esperando, el barbero lo atiende
            *customers -= 1; // Disminuimos el número de clientes esperando
            println!("El barbero está cortando el cabello de un cliente. Quedan {} clientes.", *customers);

            // El semáforo se libera cuando el barbero empieza a cortar
            drop(customers); // Liberamos el mutex

            // Simulamos el tiempo que tarda el corte de cabello
            tokio::time::sleep(Duration::from_secs(2)).await; // Corte de cabello

            // El barbero ha terminado de atender al cliente
            println!("El barbero ha terminado de cortar el cabello de un cliente.");

            // Reducimos el número de clientes restantes
            let mut remaining = self.remaining_customers.lock().await;
            *remaining -= 1;

            // Si no hay más clientes, terminamos
            if *remaining == 0 {
                println!("El barbero ha terminado de atender a todos los clientes.");
                break; // Salimos del ciclo infinito
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let total_customers = 5; // Total de clientes a atender
    let shop = Arc::new(BarberShop::new(3, total_customers)); // Barbería con 3 sillas

    // Hilo del barbero
    let barber_shop = Arc::clone(&shop);
    let barber_thread = tokio::spawn(async move {
        barber_shop.cut_hair().await;
    });

    // Hilos de los clientes
    let mut client_threads = vec![];

    for i in 1..=total_customers { // Iniciamos los clientes desde 1 hasta el total de clientes
        let shop = Arc::clone(&shop);
        let client_thread = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(((i - 1) * 2) as u64)).await; // Asegura que los clientes lleguen escalonadamente
            println!("El cliente {} ha llegado.", i);
            shop.arrive_customer(i).await;
            tokio::time::sleep(Duration::from_secs(2)).await; // El cliente espera mientras el barbero trabaja
        });
        client_threads.push(client_thread);
    }

    // Esperamos a que todos los clientes terminen
    for client in client_threads {
        client.await.unwrap();
    }

    // Esperamos a que el barbero termine de cortar el cabello
    barber_thread.await.unwrap();
}
