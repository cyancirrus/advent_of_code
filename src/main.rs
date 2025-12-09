#![allow(dead_code, unused)]
use rand::Rng;
use rand::distr::StandardUniform;
use rand_distr::StandardNormal;
use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::sync::Arc;
use std::{error::Error, fs};

const CARDINALITY: usize = 3;

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
        dot_product += x[i] * y[i];
    }
    dot_product
}

struct UnionFind {
    parents: Vec<usize>,
    updates: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parents: (0..n).collect(),
            updates:Vec::new(),
        }
    }
    fn finalize(&mut self) {
        while let Some(u) = self.updates.pop() {
            self.update(u);
        }
    }
    fn insert(&mut self, mut node: usize, parent: usize) {
        self.parents[node] = parent;
        self.updates.push(node);
        self.update(node);
    }
    fn update(&mut self, mut node: usize) {
        let mut ancestor = node;
        while ancestor != self.parents[ancestor] {
            ancestor = self.parents[ancestor];
        }
        while ancestor != self.parents[node] {
            self.parents[node] = ancestor;
            node = ancestor;
        }
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
            (self.pmap[i].entry(hash).or_default())
                .push((idx, arc_coord.clone()))
                .clone();
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
    fn one_nearest(&self, coord: &Coord) -> (usize, f32, Arc<Coord>) {
        let mut min_distance = f32::MAX;
        let mut closest = (usize::MAX, f32::MAX, Arc::new([f32::MAX; 3]));
        for i in 0..self.p_card {
            let hash = self.proj[i].project(coord);
            for (idx, candidate) in &self.pmap[i][&hash] {
                let distance = distance_coords(coord, &candidate);
                if distance < min_distance {
                    closest = (*idx, distance, candidate.clone());
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

// okay to do this i mean doesn't seem like bad idea what do we do
// we build out a hashmap which is a projection for a random based coord and bucket it by this
// then for each of the items we would like look at the buckets and check distances between
// elements

fn alpha_determine_circuit(
    knn: &mut LshKNearestNeighbors,
    unions: &mut UnionFind,
    coords: &[Coord],
    steps: usize,
) -> u64 {
    // // knn should already be initialized
    if knn.n_elem == 0 || coords.is_empty() { return 0; }
    let mut product = 1;
    let n = coords.len();
    for s in 0..steps {
        let mut min_distance = f32::MAX;
        let (mut base, mut target) = (usize::MAX, usize::MAX);
        for (base_idx, candidate) in coords.iter().enumerate() {
            let (target_idx, dist, partner) = knn.one_nearest(candidate);
            if dist < min_distance && unions.parents[base_idx] != unions.parents[target_idx] {
                min_distance = dist;
            }
        }
        unions.finalize()
    }
    let mut cluster_size = vec![0; n];
    for &n in &unions.parents {
        cluster_size[n] += 1;
    }
    cluster_size.sort_by(|a, b| b.cmp(a));
    for i in 0..3 {
        product *= cluster_size[i];
    }
    product
}

fn main() {
    // let tachyon = parser("./data/day_7.txt");
    // match tachyon {
    //     Ok((mut a, s)) => {
    //         println!("Beta result {}", beta_tachyon_many_worlds(a.clone(), &s));
    //     }
    //     _ => {
    //         println!("Error in parsing");
    //     }
    // }
}
