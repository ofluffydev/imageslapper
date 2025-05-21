# Image Slapper

This crate makes use of layering and patch editing to render only what is needed, while making full use of your systems resources.

## Features

### Optimized

- Caching: Utilizes QOI images in a cache for all images
- Full concurrency: Where possible, the library will use concurrency to speed up rendering
- Layering: A full layering system for creating layered changes, combining them, and rendering the final layer
- Sparse Pixel Update System: For things that do not fill their bounds, the library will utilize delta pixel buffers in order to only iterate over the pixels that were changed
- GPU Support: GPU accelerated rendering when enabled and supported

### Complex features, simple API

- Under the hood the "magic" is taken care of for you
- Fully configurable
- Gives you the same high level API's as the original [imagesmacker](https://github.com/whinee/imagesmacker)

### Editing options

- Text: Render text on images
  - Alignment
  - Any font
  - Any color
  - Any size
  - Automatic and configurable line wrapping and bounds filling
- Shapes: Complex geometry options
  - Rectangles
  - Circles
  - Ellipses
  - Polygons
  - Lines
- Anchoring
  - All shapes and text can be anchored to a specific point on the image, typically used with enums:
    - TopLeft
    - TopRight
    - BottomLeft
    - BottomRight
    - Center
    - CenterLeft
    - CenterRight
    - CenterTop
    - CenterBottom
