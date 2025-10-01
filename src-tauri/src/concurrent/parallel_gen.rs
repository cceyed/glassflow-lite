// T045: ParallelGenerator - use Tokio async runtime with semaphore for concurrent generation
use tokio::sync::Semaphore;
use std::sync::Arc;
use anyhow::Result;

pub struct ParallelGenerator {
    max_concurrent: usize,
}

impl ParallelGenerator {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            max_concurrent: max_concurrent.max(1).min(10), // Clamp between 1-10
        }
    }

    pub async fn generate_batch<T, R, F, Fut>(
        &self,
        items: Vec<T>,
        generator: F,
    ) -> Vec<Result<R>>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<R>> + Send,
    {
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));
        let generator = Arc::new(generator);
        let mut handles = Vec::new();
        
        for item in items {
            let sem = semaphore.clone();
            let gen = generator.clone();
            
            let handle = tokio::spawn(async move {
                // Acquire permit (blocks if max concurrent reached)
                let _permit = sem.acquire().await.unwrap();
                
                // Generate (permit held until this completes)
                gen(item).await
            });
            
            handles.push(handle);
        }
        
        // Wait for all to complete
        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => results.push(Err(anyhow::anyhow!("Task panicked: {}", e))),
            }
        }
        
        results
    }

    pub async fn generate_with_timeout<T, R, F, Fut>(
        &self,
        items: Vec<T>,
        generator: F,
        timeout: std::time::Duration,
    ) -> Vec<Result<R>>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<R>> + Send,
    {
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));
        let generator = Arc::new(generator);
        let mut handles = Vec::new();
        
        for item in items {
            let sem = semaphore.clone();
            let gen = generator.clone();
            
            let handle = tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                
                // Add timeout
                match tokio::time::timeout(timeout, gen(item)).await {
                    Ok(result) => result,
                    Err(_) => Err(anyhow::anyhow!("Generation timed out after {:?}", timeout)),
                }
            });
            
            handles.push(handle);
        }
        
        // Wait for all to complete
        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => results.push(Err(anyhow::anyhow!("Task panicked: {}", e))),
            }
        }
        
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_parallel_generation() {
        let generator = ParallelGenerator::new(3);
        
        let items = vec![1, 2, 3, 4, 5];
        let results = generator
            .generate_batch(items, |x| async move {
                tokio::time::sleep(Duration::from_millis(10)).await;
                Ok(x * 2)
            })
            .await;
        
        assert_eq!(results.len(), 5);
        assert!(results.iter().all(|r| r.is_ok()));
    }

    #[tokio::test]
    async fn test_timeout() {
        let generator = ParallelGenerator::new(2);
        
        let items = vec![1, 2];
        let results = generator
            .generate_with_timeout(
                items,
                |x| async move {
                    if x == 2 {
                        tokio::time::sleep(Duration::from_secs(10)).await;
                    }
                    Ok(x)
                },
                Duration::from_millis(100),
            )
            .await;
        
        assert_eq!(results.len(), 2);
        assert!(results[0].is_ok());
        assert!(results[1].is_err()); // Should timeout
    }
}
