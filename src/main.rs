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
const EPSILON: f32 = 1e-6;

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
    fn union(&mut self, a:usize, b:usize) {
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
    fn one_nearest(&self, unions:&mut UnionFind, coord: &Coord, c_idx:usize) -> (usize, f32) {
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
    // for p in &knn.pmap {
    //     println!("Hashamp appears as {p:?}");
    // }
    let n = coords.len();
    for s in 0..steps - 1 {
        println!("--------------------------------------------------------------------------------");
        println!("                             CHOOSING {s}                                       ");
        println!("clusters {:?}", unions.parents);
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
        println!("dist: {min_distance}, base:{base}, target: {target}");
        println!("--------------------------------------------------------------------------------");
        let root_base = unions.find(base);
        let root_target = unions.find(target);
        unions.union(root_base, root_target);
        unions.finalize();
    }
    println!("-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    println!("-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    println!("END END END END END");

    // println!("Unions {:?}", unions.parents);
    let mut cluster_size = vec![0; n];
    for &n in &unions.parents {
        cluster_size[n] += 1;
    }
    cluster_size.sort_by(|a, b| b.cmp(a));
    println!("clusters {:?}", unions.parents);
    println!("cluster sizese {cluster_size:?}");
    for i in 0..3 {
        product *= cluster_size[i];
    }
    product
}

fn main() {
    let coords = parser("./data/day_8.txt");
    // println!("distance found {}", distance_coords(&[431.0, 825.0, 988.0],&[162.0, 817.0, 812.0]));
    // println!("distance example {}", distance_coords(&[906.0,360.0,560.0], &[805.0,96.0,715.0]));

    match coords {
        Ok(c) => {
            let n = c.len();
            let mut union = UnionFind::new(n);
            // let mut knn = LshKNearestNeighbors::new(200f32, 10_000);
            let mut knn = LshKNearestNeighbors::new(100_000.0, 12);
            for c_idx in 0..n {
                knn.insert(c_idx, c[c_idx])

            }
            let result = alpha_determine_circuit(
                &mut knn,
                &mut union,
                &c,
                10,
            );
            println!("Alpha result {}", result);
        }
        _ => {
            println!("Error in parsing");
        }
    }
}
