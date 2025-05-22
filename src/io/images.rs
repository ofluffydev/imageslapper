use tracing::info;

/// Loads an image from the specified path and returns it along with the elapsed time.
pub fn load_image(image_path: &str) -> (image::DynamicImage, std::time::Duration) {
    let start = std::time::Instant::now();
    let image = image::open(image_path).expect("Failed to load image");
    (image, start.elapsed())
}

/// Loads an image, using `qoi` if the extension is `.qoi`, or falls back to the default loader.
pub fn load_image_with_qoi(image_path: &str) -> (image::DynamicImage, std::time::Duration) {
    let start = std::time::Instant::now();
    let extension = std::path::Path::new(image_path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");

    info!("Loading image: {}", image_path);
    info!("Detected extension: {}", extension);

    let image = if extension.eq_ignore_ascii_case("qoi") {
        info!("Using QOI loader for image: {}", image_path);
        image::open(image_path).expect("Failed to load QOI image")
    } else {
        let cache_path = format!("./.cache/{image_path}.qoi");
        if std::path::Path::new(&cache_path).exists() {
            info!("Loading image from cache: {}", cache_path);
            image::open(&cache_path).expect("Failed to load cached image")
        } else {
            info!("Loading image directly: {}", image_path);
            let image = image::open(image_path).expect("Failed to load image");
            let cache_dir = std::path::Path::new(&cache_path)
                .parent()
                .expect("Failed to get cache directory");
            std::fs::create_dir_all(cache_dir).expect("Failed to create cache directory");
            image
                .save(&cache_path)
                .expect("Failed to save image to cache");
            info!("Image saved to cache: {}", cache_path);
            image
        }
    };

    (image, start.elapsed())
}
