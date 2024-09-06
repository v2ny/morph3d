# morph3d

`morph3d` is a Rust crate for loading and processing 3D model files in various formats, including OBJ, GLTF, DAE, USDZ, and STL. This crate provides functionality to extract and manipulate mesh data such as vertices, indices, texture coordinates, and normals, and apply transformations to the loaded meshes.

## Features

- **OBJ Loader**: Load and process OBJ files, including vertices, normals, texture coordinates, and transformation matrices.
- **GLTF Loader**: Load GLTF & GLB files with support for vertices, indices, texture coordinates, normals, and node matrices.

### Upcomming
- **DAE Loader**: Load Collada (DAE) files and extract vertices, indices, texture coordinates, and normals.
- **USDZ and STL Support**: Soon.

## Getting Started

### Installation

Add `morph3d` to your `Cargo.toml`:

```toml
[dependencies]
morph3d = "0.1"  # Replace with the latest version
```

Or use 
```bash
$ cargo add morph3d
```

### Usage

```rust
use morph3d::{Loader, ModelFormat, Collection};

fn main() {
    let mut loader = Loader::new("path/to/your/model.obj", ModelFormat::OBJ);

    // Load OBJ file
    let obj_collection: Vec<Collection> = loader.load();
    // Process obj collections...
}
```

### API

#### `Collection`

- `vertices: Vec<[f32; 3]>`: List of vertices in the mesh.
- `indices: Vec<u32>`: List of indices for the mesh.
- `texture_coordinates: Vec<[f32; 2]>`: List of texture coordinates.
- `normals: Vec<[f32; 3]>`: List of normals.
- `matrix: Matrix4<f32>`: Transformation matrix applied to the mesh.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have suggestions or improvements.

## License

This crate is licensed under the MIT License OR Apache 2.0 License. Feel free to choose one of them.

## Acknowledgements

- Thanks to the Rust community and the maintainers of the `dae-parser`, `tobj`, and other libraries used in this project.