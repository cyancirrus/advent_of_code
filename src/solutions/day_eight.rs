#![allow(dead_code, unused)]
use rand::Rng;
use rand::distr::StandardUniform;
use std::collections::HashMap;
use std::mem;
use std::sync::Arc;
use std::{error::Error, fs};

const CARDINALITY: usize = 3;
// const EPSILON: f32 = 1e-6;
const CLUSTER_PRODUCT_THRESHOLD: usize = 3;

type Coord = [f32; CARDINALITY];
type ProjHash = HashMap<u32, Vec<(usize, Arc<Coord>)>>;

struct Projection {
    p_vec: Coord,
    p_cons: f32,
    b_width: f32,
}

impl Projection {
    fn new(b_width: f32) -> Projection {
        let mut rng = rand::rng();
        let mut p_vec = [0f32; CARDINALITY];
        for i in 0..CARDINALITY {
            p_vec[i] += rng.sample::<f32, _>(StandardUniform);
        }
        let p_cons = rng.sample::<f32, _>(StandardUniform);
        Projection {
            p_vec,
            p_cons,
            b_width,
        }
    }
    fn project(&self, coord: &Coord) -> u32 {
        let mut projection = self.p_cons;
        for i in 0..CARDINALITY {
            projection += self.p_vec[i] * coord[i];
        }
        (projection / self.b_width).floor() as u32
    }
}

struct LshKNearestNeighbors {
    n_elem: usize,
    p_card: usize,
    b_width: f32,
    proj: Vec<Projection>,
    pmap: Vec<ProjHash>,
}

fn distance_coords(x: &Coord, y: &Coord) -> f32 {
    let mut dot_product = 0f32;
    for i in 0..CARDINALITY {
        dot_product += (x[i] - y[i]) * (x[i] - y[i]);
    }
    dot_product.sqrt()
}

struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parents: (0..n).collect(),
        }
    }
    fn finalize(&mut self) {
        for n in 0..self.parents.len() {
            self.find(n);
        }
    }
    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a != root_b {
            self.parents[root_a] = root_b;
        }
    }
    fn find(&mut self, mut node: usize) -> usize {
        // println!("node {node}");
        let mut ancestor = node;
        while ancestor != self.parents[ancestor] {
            ancestor = self.parents[ancestor];
        }
        while ancestor != self.parents[node] {
            let next = self.parents[node];
            self.parents[node] = ancestor;
            node = next;
        }
        self.parents[node]
    }
}

impl LshKNearestNeighbors {
    fn new(b_width: f32, p_card: usize) -> LshKNearestNeighbors {
        let mut proj = vec![];
        let mut pmap = vec![HashMap::new(); p_card];
        for i in 0..p_card {
            proj.push(Projection::new(b_width));
        }
        LshKNearestNeighbors {
            n_elem: 0,
            p_card,
            b_width,
            proj,
            pmap,
        }
    }
    fn insert(&mut self, idx: usize, coord: Coord) {
        let arc_coord = Arc::new(coord);
        for i in 0..self.p_card {
            let hash = self.proj[i].project(&arc_coord);
            (self.pmap[i].entry(hash).or_default()).push((idx, arc_coord.clone()));
        }
        self.n_elem += 1;
    }
    fn knn(&self, k: usize, coord: &Coord) -> Vec<(usize, Arc<Coord>)> {
        let mut similar = vec![];
        // Get similar candidates
        for i in 0..self.p_card {
            similar.extend(self.pmap[i][&self.proj[i].project(&coord)].clone());
        }
        similar.sort_by(|(idx_a, candidate_a), (idx_b, candidate_b)| {
            let dist_a = distance_coords(coord, candidate_a);
            let dist_b = distance_coords(coord, candidate_b);
            dist_a.total_cmp(&dist_b)
        });
        similar.dedup();
        similar.truncate(k);
        similar
    }

    fn one_nearest(&self, unions: &mut UnionFind, coord: &Coord, c_idx: usize) -> (usize, f32) {
        let mut closest = (usize::MAX, f32::MAX);
        for i in 0..self.p_card {
            let hash = self.proj[i].project(coord);
            for (idx, candidate) in &self.pmap[i][&hash] {
                let distance = distance_coords(coord, &candidate);
                if unions.find(c_idx) != unions.find(*idx) && distance < closest.1 {
                    closest = (*idx, distance);
                }
            }
        }
        closest
    }
}

pub fn parser(path: &str) -> Result<Vec<Coord>, Box<dyn Error>> {
    let mut coords = vec![];
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(format!("Path does not exist {path:?}").into()),
    };

    for line in content.lines() {
        let nums: Vec<&str> = line.splitn(CARDINALITY, ",").collect();
        match (nums[0].parse(), nums[1].parse(), nums[2].parse()) {
            (Ok(one), Ok(two), Ok(three)) => coords.push([one, two, three]),
            _ => return Err(format!("Oddity occurred in the input data").into()),
        }
    }
    Ok(coords)
}

fn alpha_determine_circuit(
    knn: &mut LshKNearestNeighbors,
    unions: &mut UnionFind,
    coords: &[Coord],
    steps: usize,
) -> u64 {
    if knn.n_elem == 0 || coords.is_empty() {
        return 0;
    }

    let n = coords.len();
    let mut used_pairs = std::collections::HashSet::new();

    for s in 0..steps {
        let mut min_distance = f32::MAX;
        let (mut base, mut target) = (usize::MAX, usize::MAX);

        for base_idx in 0..n {
            for target_idx in (base_idx + 1)..n {
                let pair = if base_idx < target_idx {
                    (base_idx, target_idx)
                } else {
                    (target_idx, base_idx)
                };

                if used_pairs.contains(&pair) {
                    continue;
                }

                let dist = distance_coords(&coords[base_idx], &coords[target_idx]);
                if dist < min_distance {
                    min_distance = dist;
                    base = base_idx;
                    target = target_idx;
                }
            }
        }

        if base == usize::MAX {
            break;
        }

        let pair = if base < target {
            (base, target)
        } else {
            (target, base)
        };
        used_pairs.insert(pair);

        // Union them (even if already in same circuit, this counts as a step)
        let root_base = unions.find(base);
        let root_target = unions.find(target);
        if root_base != root_target {
            unions.union(root_base, root_target);
        }
    }

    unions.finalize();
    let mut cluster_size = vec![0; n];
    for &n in &unions.parents {
        cluster_size[n] += 1;
    }
    cluster_size.sort_by(|a, b| b.cmp(a));

    let mut product = 1;
    for i in 0..CLUSTER_PRODUCT_THRESHOLD {
        product *= cluster_size[i];
    }
    product
}

fn alpha_determine_circuit_unique(
    knn: &mut LshKNearestNeighbors,
    unions: &mut UnionFind,
    coords: &[Coord],
    steps: usize,
) -> u64 {
    // // knn should already be initialized
    if knn.n_elem == 0 || coords.is_empty() {
        return 0;
    }
    let mut product = 1;
    let n = coords.len();
    for s in 0..steps {
        let mut min_distance = f32::MAX;
        let (mut base, mut target) = (usize::MAX, usize::MAX);
        for (base_idx, candidate) in coords.iter().enumerate() {
            let (target_idx, dist) = knn.one_nearest(unions, candidate, base_idx);
            if dist < min_distance {
                min_distance = dist;
                base = base_idx;
                target = target_idx;
            }
        }
        let root_base = unions.find(base);
        let root_target = unions.find(target);
        unions.union(root_base, root_target);
    }
    unions.finalize();
    let mut cluster_size = vec![0; n];
    for &n in &unions.parents {
        cluster_size[n] += 1;
    }
    cluster_size.sort_by(|a, b| b.cmp(a));
    for i in 0..CLUSTER_PRODUCT_THRESHOLD {
        product *= cluster_size[i];
    }
    product
}

fn beta_determine_circuit(coords: &[Coord]) -> u64 {
    if coords.is_empty() {
        return 0;
    }

    let n = coords.len();
    let mut unions = UnionFind::new(n);
    let mut used_pairs = std::collections::HashSet::new();

    loop {
        let mut min_distance = f32::MAX;
        let (mut base, mut target) = (usize::MAX, usize::MAX);

        for base_idx in 0..n {
            for target_idx in (base_idx + 1)..n {
                let pair = (base_idx.min(target_idx), base_idx.max(target_idx));

                if used_pairs.contains(&pair) {
                    continue;
                }

                let dist = distance_coords(&coords[base_idx], &coords[target_idx]);
                if dist < min_distance {
                    min_distance = dist;
                    base = base_idx;
                    target = target_idx;
                }
            }
        }

        if base == usize::MAX {
            break;
        }

        let pair = (base.min(target), base.max(target));
        used_pairs.insert(pair);

        let root_base = unions.find(base);
        let root_target = unions.find(target);

        if root_base != root_target {
            unions.union(root_base, root_target);

            // Check if we now have only one component
            unions.finalize();
            let unique_roots: std::collections::HashSet<_> =
                (0..n).map(|i| unions.find(i)).collect();

            if unique_roots.len() == 1 {
                // This connection completed the circuit!
                println!(
                    "Final connection: {:?} and {:?}",
                    coords[base], coords[target]
                );
                return (coords[base][0] * coords[target][0]).floor() as u64;
            }
        }
    }

    0
}

// fn main() {
//     let coords = parser("./data/day_8.txt");

//     match coords {
//         Ok(c) => {
//             let n = c.len();
//             let mut union = UnionFind::new(n);
//             // let mut knn = LshKNearestNeighbors::new(200f32, 10_000);
//             let mut knn = LshKNearestNeighbors::new(10_000.0, 12);
//             for c_idx in 0..n {
//                 knn.insert(c_idx, c[c_idx])
//             }
//             let result = beta_determine_circuit( &c);
//             println!("Alpha result {}", result);
//         }
//         _ => {
//             println!("Error in parsing");
//         }
//     }
// }
