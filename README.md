# vecpac

**A high-performance, low-compute hexagonal spatial indexer for Rust.**

`vecpac` is a lightweight utility designed to map spatial data and high-dimensional vectors onto a Hexagonal Close Packed (HCP) grid. By utilizing **Hexagonal Cube Coordinates** ($q, r, s$), it eliminates the diagonal distortion inherent in standard Cartesian $(x, y)$ grids. 

Every node in `vecpac` has exactly 6 equidistant neighbors, allowing for lightning-fast spatial queries, clustering, and nearest-neighbor searches using simple integer arithmetic instead of expensive floating-point math.

Ideal for **Vector Databases**, Procedural Terrain Generation, and high-efficiency Spatial Hashing.

## Features

* **Zero-Point Balance:** Enforces the strict $q + r + s = 0$ geometric constraint for absolute structural integrity.
* **Fractional Quantization:** Instantly "snaps" continuous 2D floating-point coordinates (e.g., projected embeddings) to the nearest discrete hex node.
* **Low-Compute Distance:** Calculates geometric grid distance using ultra-fast integer math (Manhattan distance / 2) rather than heavy Euclidean formulas.
* **The "Seed" Multiplicity:** Generates the 6 perfectly equidistant neighbor nodes instantly.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vecpac = "0.1.0"
```

Or run: 

```bash
cargo add vecpac
```

## Quick Start
1. Creating Nodes and Calculating Distance

Because the grid is perfectly balanced, finding the distance between two points avoids costly square root operations.

```rust
use vecpac::HexNode;

fn main() {
    let origin = HexNode::new(0, 0);
    let target = HexNode::new(3, -5);

    // Calculates grid steps using integer math
    let distance = origin.distance_to(&target);
    
    println!("Distance to target: {}", distance); // Output: 5
}
```

2. Quantizing Fractional Vectors

If you are building a vector database or working with floating-point coordinate spaces, you can snap those values directly into the hexagonal bucket system.

```rust
use vecpac::HexNode;

fn main() {
    let raw_x = 1.2;
    let raw_y = 2.1;
    
    // Snaps the floating point coordinate to the nearest geometric hex bucket
    let hex_bucket = HexNode::from_fractional(raw_x, raw_y);
    
    println!("Quantized to: {:?}", hex_bucket); 
    // Output: HexNode { q: 1, r: 2, s: -3 }
}
```

3. Finding Equidistant Neighbors

Expand your search radius by finding the 6 immediate neighbors of any node.

```rust
use vecpac::HexNode;

fn main() {
    let center = HexNode::new(0, 0);
    
    // Returns an array of the 6 surrounding HexNodes
    let neighbors = center.neighbors();
    
    for neighbor in neighbors.iter() {
        println!("{:?}", neighbor);
    }
}
```

### Why Hexagonal Packing for Vector Databases?

Most spatial indices (like KD-Trees or R-Trees) rely on square grids. However, in a square grid, the diagonal neighbors are further away than the orthogonal neighbors, requiring complex floating-point math to calculate "true" distance.

vecpac solves this. By projecting high-dimensional vectors down to a 2D plane and quantizing them into a hexagonal grid, you can group similar data into "buckets." Finding the nearest neighbor becomes as simple as checking the current hex bucket, and if more results are needed, expanding to the 6 perfectly equidistant adjacent buckets.
