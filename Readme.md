**triangulation** is a utily for converting image using [delaunay triangulation](https://en.wikipedia.org/wiki/Delaunay_triangulation).
The generated image is composed of title triangle.

![Sample image](samples/perroquet_colorfull.png)

### How does it work ?

- First, the original image is blurred, then a [sobel filter](https://en.wikipedia.org/wiki/Sobel_operator) operator is apply to detect the edge off the image

- A certain amount of points is randomly selected on the edges of the original image.

- After that, the [delaunay triangulation](https://en.wikipedia.org/wiki/Delaunay_triangulation) algorithm is applied using the chosen point as
  vertex.

- Finally, the result image is formed using the triangles generated from the previous step.
  The color chosen to fill the triangle corresponds to the color of pixel of the original image placed at the center of the triangle.

### Install & usage

This program use [rust](https://www.rust-lang.org). build the release binary.

```bash
cargo build --release
```

```bash
./target/release/triangulation -in input.jpg --out output.png
```

### Api

```rust

// open the input file using crate image.
let input_image = ImageReader::open("input.jpg")?.decode()?;

let triangulation = Triangulation::new{..Default::default()};
let drawer = Drawer{..Default::default()};

// generate the triangles
let (triangles, source_image) = triangulation.generate_triangle(input_image);

// generate the result image
let result_image = drawer.draw(source_image, triangles)?;

// save as png
result_image.save_as_png("output.png")?;

```

### Options

```bash
$ cargo run  -- --help

Usage: triangulation [OPTIONS] --in <INPUT> --out <OUTPUT>
```

The following options are supported:

| options | description                                                                          | default |
| ------- | ------------------------------------------------------------------------------------ | ------- |
| `in`    | source image                                                                         | n/a     |
| `out`   | destination image                                                                    | n/a     |
| `bf`    | blur filter factor                                                                   | 1       |
| `mp`    | max number of points in the generated image                                          | 2500    |
| `pt`    | point threshold (control the amount of point detected by the sobel filter operation) | 10      |
| `pr`    | point rate (control the number of point use by delaunay triangulation)               | 0.075   |
| `gr`    | convert the result image into grayscale                                              | false   |
| `ow`    | wireframe only (do not fill the triangle in the generated image)                     | false   |
| `sw`    | stroke width in the generated image (0 mean no stroke)                               | 0.1     |
| `wb`    | with background (whenever the generated image should have a background)              |  true   |
| `bc`    | background color (in hex format)                                                     |  white  |
| `sc`    | stroke color (in hex format)                                                         |  black  |

### Examples
