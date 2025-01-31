use std::sync::Arc;
use tokio::sync::{Semaphore, SemaphorePermit};

pub struct ConnectionPool {
    semaphore: Arc<Semaphore>,
    max_connections: usize,
}

impl ConnectionPool {
    pub fn new(max_connections: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_connections)),
            max_connections,
        }
    }

    pub async fn acquire(&self) -> PooledConnection {
        let permit = self.semaphore.acquire().await.unwrap();
        PooledConnection {
            permit: Some(permit),
            pool: self,
        }
    }
}

pub struct PooledConnection<'a> {
    permit: Option<SemaphorePermit<'a>>,
    pool: &'a ConnectionPool,
}

impl<'a> PooledConnection<'a> {
    pub fn release(self) {
        // Permit is automatically released when dropped
        drop(self.permit);
    }
}
