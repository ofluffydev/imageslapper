use std::path::PathBuf;

pub struct ImageBatchBuilder {
    pub images: Vec<String>,
    pub batch_size: usize,
}
impl ImageBatchBuilder {
    pub fn new(images: Vec<String>, batch_size: usize) -> Self {
        Self { images, batch_size }
    }

    pub fn build(&self) -> Vec<(String, String)> {
        let mut batches = Vec::new();
        for chunk in self.images.chunks(self.batch_size) {
            let batch = chunk
                .iter()
                .map(|image| (image.clone(), image.clone()))
                .collect::<Vec<_>>();
            batches.push(batch);
        }
        batches.into_iter().flatten().collect()
    }
}

pub struct ImagePipeline {
    pub cache_dir: PathBuf,
    pub output_dir: PathBuf,
}

impl ImagePipeline {
    pub fn new(cache_dir: Option<PathBuf>, output_dir: Option<PathBuf>) -> Self {
        let cache_dir = cache_dir.unwrap_or_else(|| {
            let default_cache = PathBuf::from("./.cache");
            std::fs::create_dir_all(&default_cache).expect("Failed to create cache directory");
            default_cache
        });

        let output_dir = output_dir.unwrap_or_else(|| {
            let default_output = PathBuf::from("./out");
            std::fs::create_dir_all(&default_output).expect("Failed to create output directory");
            default_output
        });

        Self {
            cache_dir,
            output_dir,
        }
    }

    pub fn build(&self) -> Vec<(PathBuf, PathBuf)> {
        vec![
            (self.cache_dir.clone(), self.cache_dir.clone()),
            (self.output_dir.clone(), self.output_dir.clone()),
        ]
    }
}
