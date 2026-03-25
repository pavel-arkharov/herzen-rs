use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmbeddingError {
    #[error("model not loaded: {0}")]
    ModelNotLoaded(String),
    #[error("inference error: {0}")]
    InferenceError(String),
}

/// Trait for embedding providers. Implementations can use ONNX Runtime,
/// an HTTP embedding API, or any other backend.
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    /// Embed a single text into a vector.
    async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError>;

    /// Embed multiple texts (batch).
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, EmbeddingError>;

    /// Dimension of the embedding vectors.
    fn dimension(&self) -> usize;
}

/// Cosine similarity between two vectors.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "vectors must have same dimension");

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}

/// Find the best matching example for a given input embedding.
pub fn best_match(input: &[f32], examples: &[Vec<f32>]) -> (usize, f32) {
    examples
        .iter()
        .enumerate()
        .map(|(i, ex)| (i, cosine_similarity(input, ex)))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or((0, 0.0))
}

/// A mock embedding provider for testing. Maps text to a deterministic
/// vector based on character frequencies — not semantically meaningful,
/// but allows testing the matching pipeline without an actual model.
#[cfg(test)]
pub struct MockEmbeddingProvider {
    dim: usize,
}

#[cfg(test)]
impl MockEmbeddingProvider {
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }

    fn text_to_vector(&self, text: &str) -> Vec<f32> {
        let mut vec = vec![0.0f32; self.dim];
        for (i, byte) in text.bytes().enumerate() {
            vec[i % self.dim] += byte as f32 / 255.0;
        }
        // Normalize
        let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            vec.iter_mut().for_each(|x| *x /= norm);
        }
        vec
    }
}

#[cfg(test)]
#[async_trait]
impl EmbeddingProvider for MockEmbeddingProvider {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        Ok(self.text_to_vector(text))
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        Ok(texts.iter().map(|t| self.text_to_vector(t)).collect())
    }

    fn dimension(&self) -> usize {
        self.dim
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cosine_identical_vectors() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn cosine_orthogonal_vectors() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&a, &b)).abs() < 1e-6);
    }

    #[test]
    fn cosine_opposite_vectors() {
        let a = vec![1.0, 0.0];
        let b = vec![-1.0, 0.0];
        assert!((cosine_similarity(&a, &b) + 1.0).abs() < 1e-6);
    }

    #[test]
    fn best_match_finds_closest() {
        let input = vec![1.0, 0.0, 0.0];
        let examples = vec![
            vec![0.0, 1.0, 0.0], // orthogonal
            vec![0.9, 0.1, 0.0], // close
            vec![0.0, 0.0, 1.0], // orthogonal
        ];
        let (idx, score) = best_match(&input, &examples);
        assert_eq!(idx, 1);
        assert!(score > 0.9);
    }

    #[tokio::test]
    async fn mock_provider_produces_vectors() {
        let provider = MockEmbeddingProvider::new(64);
        let vec = provider.embed("hello world").await.unwrap();
        assert_eq!(vec.len(), 64);
        // Should be normalized
        let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-5);
    }

    #[tokio::test]
    async fn mock_provider_similar_texts_closer() {
        let provider = MockEmbeddingProvider::new(128);
        let a = provider.embed("turn off the lights").await.unwrap();
        let b = provider.embed("turn off the lamp").await.unwrap();
        let c = provider.embed("что делать с деревьями").await.unwrap();
        // a and b should be more similar than a and c
        let sim_ab = cosine_similarity(&a, &b);
        let sim_ac = cosine_similarity(&a, &c);
        assert!(sim_ab > sim_ac);
    }
}
