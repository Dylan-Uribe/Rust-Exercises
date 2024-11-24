use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::Duration;

/// Represents a reader-writer lock allowing multiple readers or a single writer.
struct ReadWriteLock {
    readers: Arc<Mutex<u32>>,          // Count of current readers
    writer: Arc<Semaphore>,             // Semaphore for writers
    read_lock: Arc<Semaphore>,          // Semaphore for reader access control
}

impl ReadWriteLock {
    /// Creates a new `ReadWriteLock`.
    fn new() -> Self {
        ReadWriteLock {
            readers: Arc::new(Mutex::new(0)),
            writer: Arc::new(Semaphore::new(1)), // Only one writer allowed
            read_lock: Arc::new(Semaphore::new(1)), // Control access for readers
        }
    }

    /// Handles a reader trying to read.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the reader.
    async fn read(&self, id: u32) {
        // Wait to enter the reading section
        let _ = self.read_lock.acquire().await.unwrap();

        // Increment the count of current readers
        {
            let mut count = self.readers.lock().await;
            *count += 1;
            if *count == 1 {
                // First reader blocks writers
                let _ = self.writer.acquire().await.unwrap();
            }
        }

        // Release the read lock for others
        self.read_lock.add_permits(1);

        // Simulate reading
        println!("Reader {} is reading.", id);
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Decrement the count of current readers
        {
            let mut count = self.readers.lock().await;
            *count -= 1;
            if *count == 0 {
                // Last reader releases writer lock
                self.writer.add_permits(1);
            }
        }
    }

    /// Handles a writer trying to write.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the writer.
    async fn write(&self, id: u32) {
        // Wait to enter the writing section
        let _ = self.writer.acquire().await.unwrap();

        // Simulate writing
        println!("Writer {} is writing.", id);
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Release the writer lock
        self.writer.add_permits(1);
    }
}

#[tokio::main]
async fn main() {
    let rw_lock = Arc::new(ReadWriteLock::new());

    let mut tasks = vec![];

    // Create readers
    for i in 1..=5 {
        let rw_lock_clone = Arc::clone(&rw_lock);
        tasks.push(tokio::spawn(async move {
            rw_lock_clone.read(i).await;
        }));
    }

    // Create writers
    for i in 1..=2 {
        let rw_lock_clone = Arc::clone(&rw_lock);
        tasks.push(tokio::spawn(async move {
            rw_lock_clone.write(i).await;
        }));
    }

    // Wait for all tasks to finish
    for task in tasks {
        task.await.unwrap();
    }
}

/*
The code above creates a `ReadWriteLock` struct that allows multiple readers to read simultaneously or a single writer to write exclusively. The `read` method manages the arrival of readers, while the `write` method handles writers.

The `main` function creates a new `ReadWriteLock` instance, spawns multiple reader and writer tasks, and waits for all tasks to complete.

To run the code, execute the following command:
cargo run
*/
