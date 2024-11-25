use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::Duration;

/// Represents a reader-writer lock allowing multiple readers or a single writer.
struct ReadWriteLock {
    readers: Arc<Mutex<u32>>,          // Count of current readers
    writer: Arc<Semaphore>,           // Semaphore for writers
    read_lock: Arc<Semaphore>,        // Semaphore for reader access control
}

impl ReadWriteLock {
    /// Creates a new `ReadWriteLock`.
    fn new() -> Self {
        ReadWriteLock {
            readers: Arc::new(Mutex::new(0)),
            writer: Arc::new(Semaphore::new(1)),  // Only one writer allowed
            read_lock: Arc::new(Semaphore::new(1)),  // Control access for readers
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
                println!("Reader {} is the first reader, blocking writers.", id);
                let _ = self.writer.acquire().await.unwrap();
            }
        }

        // Release the read lock for others
        self.read_lock.add_permits(1);

        // Simulate reading
        println!("Reader {} is reading.", id);
        println!("Reader {} has finished reading.", id);
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Decrement the count of current readers
        {
            let mut count = self.readers.lock().await;
            *count -= 1;
            if *count == 0 {
                // Last reader releases writer lock
                println!("Reader {} is the last reader, releasing writer lock.", id);
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
        println!("Writer {} has finished writing.", id);
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
    for i in 1..=3 {
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
The `ReadWriteLock` struct contains three fields: `readers`, `writer`, and `read_lock`.
The `readers` field is a `Mutex<u32>` that stores the count of current readers.
The `writer` field is a `Semaphore` with a permit count of 1, which controls writer access.
The `read_lock` field is another `Semaphore` with a permit count of 1, which controls reader access.

The `new` method creates a new `ReadWriteLock` instance.

The `read` method simulates a reader trying to read. It first acquires the `read_lock` semaphore
to enter the reading section. It then increments the count of current readers and blocks writers
if it's the first reader. After releasing the `read_lock` semaphore, it simulates reading for 2 seconds.
Finally, it decrements the count of current readers and releases the writer lock if it's the last reader.

The `write` method simulates a writer trying to write. It acquires the `writer` semaphore to
enter the writing section, simulates writing for 2 seconds, and then releases the writer lock.

The `main` function creates a new `ReadWriteLock` instance and spawns tasks for readers and writers.
The readers and writers are created in separate loops, and each task is spawned using `tokio::spawn`.
The `main` function waits for all tasks to finish using `task.await.unwrap()`.

To run the program, execute the following command:
cargo run
*/
