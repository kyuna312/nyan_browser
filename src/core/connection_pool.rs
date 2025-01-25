use std::error::Error;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct ConnectionPool {
    semaphore: Arc<Semaphore>,
}

impl ConnectionPool {
    pub fn new(size: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(size)),
        }
    }

    pub async fn acquire(&self) -> Result<PooledConnection<'_>, Box<dyn Error>> {
        let permit = self.semaphore.acquire().await?;
        Ok(PooledConnection { _permit: permit })
    }
}

pub struct PooledConnection<'a> {
    _permit: tokio::sync::SemaphorePermit<'a>,
}
