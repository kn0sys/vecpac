#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexNode {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl HexNode {
    pub fn new(q: i32, r: i32) -> Self {
        HexNode { q, r, s: -q - r }
    }

    /// Snap a continuous 2D coordinate (from a vector embedding)
    /// to the nearest HexNode for indexing.
    pub fn from_fractional(q: f64, r: f64) -> Self {
        let s = -q - r;

        let mut rq = q.round();
        let mut rr = r.round();
        let mut rs = s.round();

        let q_diff = (rq - q).abs();
        let r_diff = (rr - r).abs();
        let s_diff = (rs - s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            rq = -rr - rs;
        } else if r_diff > s_diff {
            rr = -rq - rs;
        } else {
            rs = -rq - rr;
        }

        HexNode {
            q: rq as i32,
            r: rr as i32,
            s: rs as i32,
        }
    }

    /// Returns the 6 neighbors (The Seed of Life multiplicity unit)
    pub fn neighbors(&self) -> [HexNode; 6] {
        let directions = [(1, -1), (1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1)];

        let mut neighbors = [HexNode::new(0, 0); 6];
        for (i, (dq, dr)) in directions.iter().enumerate() {
            neighbors[i] = HexNode::new(self.q + dq, self.r + dr);
        }
        neighbors
    }

    /// Calculate the geometric distance between two vectors on the hex grid.
    pub fn distance_to(&self, other: &HexNode) -> i32 {
        ((self.q - other.q).abs() + (self.r - other.r).abs() + (self.s - other.s).abs()) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_point_balance() {
        let node = HexNode::new(3, -5);
        assert_eq!(node.s, 2);
        // Prove q + r + s = 0
        assert_eq!(node.q + node.r + node.s, 0);
    }

    #[test]
    fn test_quantization() {
        // A raw floating point vector from the database
        let raw_q = 1.2;
        let raw_r = 2.1;

        let quantized = HexNode::from_fractional(raw_q, raw_r);

        assert_eq!(quantized.q, 1);
        assert_eq!(quantized.r, 2);
        assert_eq!(quantized.s, -3);
    }

    #[test]
    fn test_seed_of_life_neighbors() {
        let center = HexNode::new(0, 0);
        let neighbors = center.neighbors();

        assert_eq!(neighbors.len(), 6);
        // Every neighbor must be exactly 1 unit of distance away
        for neighbor in neighbors.iter() {
            assert_eq!(center.distance_to(neighbor), 1);
        }
    }

    #[test]
    fn test_distance() {
        let origin = HexNode::new(0, 0);
        let target = HexNode::new(3, -5);

        // Path should be exactly 5 steps across the grid
        assert_eq!(origin.distance_to(&target), 5);
    }
}
