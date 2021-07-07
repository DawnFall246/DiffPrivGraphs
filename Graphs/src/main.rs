/*
    Author: Diya Krishnan
    Purpose: Compute local differential private network stats.
 */
use rand;
use rand::Rng;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;


type Vertex = i32;
type Edge = (Vertex, Vertex);

struct Graph {
    V: HashSet<Vertex>,
    E: HashMap<Vertex, HashSet<Vertex>>,
}

fn add_vertex(g: &mut Graph, v: Vertex) -> () {
    if !g.V.contains(&v) {
        g.V.insert(v);
        g.E.insert(v, HashSet::new());
    }
}

fn edge_exists(g: &mut Graph, u: Vertex, v: Vertex) -> bool {
    g.E[&u].contains(&v)
}

fn add_edge(g: &mut Graph, e: Edge) -> () {
    let (u, v) = e;
    add_vertex(g, u);
    add_vertex(g, v);
    if !edge_exists(g, u, v) {
        g.E.entry(u).or_insert(HashSet::new()).insert(v);
    }
    if !edge_exists(g, v, u) {
        g.E.entry(v).or_insert(HashSet::new()).insert(u);
    }
}

fn delete_edge(g: &mut Graph, e: Edge) -> () {
    let (u, v) = e;
    add_vertex(g, u);
    add_vertex(g, v);
    if edge_exists(g, u, v) {
        g.E.entry(u).or_insert(HashSet::new()).remove(&v);
    }
    if edge_exists(g, v, u) {
        g.E.entry(v).or_insert(HashSet::new()).remove(&u);
    }
}

fn get_neighbors(g: &mut Graph, v: Vertex) -> &mut HashSet<Vertex> {
    add_vertex(g, v);
    return g.E.entry(v).or_insert(HashSet::new());
}

fn make_regular(g: &mut Graph, k: i32) -> () {
    let n = g.V.len() as i32;
    for i in 0..n {
        for j in 1..(k + 1) {
            add_edge(g, (i, (i + j) % n));
        }
    }
}

fn rewire(g: &mut Graph, p: f64) -> () {
    let mut rng = rand::thread_rng();
    let n = g.V.len() as i32;
    for i in 0..n {
        let neighbors = get_neighbors(g, i);
        let mut to_remove = vec![0i32; 0];
        let mut to_add = vec![0i32; 0];
        for j in neighbors.iter() {
            let r: f64 = rng.gen();
            if r < p {
                let mut target = rng.gen_range(0..n);
                while target == i || target == *j {
                    target = rng.gen_range(0..n);
                }
                to_remove.push(*j);
                to_add.push(target);
            }
        }
        for j in to_remove {
            delete_edge(g, (i, j));
        }
        for j in to_add {
            add_edge(g, (i, j));
        }
    }
}

fn avg_shortest_from_vertex(g: &mut Graph, s: Vertex) -> f64 {
    let mut reached = vec![false; g.V.len()];
    let mut dist = vec![i32::MAX; g.V.len()];
    let mut queue = VecDeque::new();
    dist[s as usize] = 0;
    queue.push_back(s);
    reached[s as usize] = true;

    while !queue.is_empty() {
        let temp = queue.pop_front();
        match temp {
            None => {
                break;
            }
            Some(u) => {
                let du = dist[u as usize];
                for v in get_neighbors(g, u).iter() {
                    if !reached[*v as usize] {
                        dist[*v as usize] = du + 1;
                        queue.push_back(*v);
                        reached[*v as usize] = true;
                    }
                }
            }
        }
    }

    let mut m: i64 = 0;

    for d in dist {
        m += d as i64;
    }
    return (m as f64) / (g.V.len() as f64);
}

fn dummy(){
    print!("Hello world!");
}

fn avg_shortest_path(g: &mut Graph) -> f64 {
    let mut total = 0.;
    for s in g.V.clone().iter() {
        total += avg_shortest_from_vertex(g, *s);
    }
    return total / g.V.len() as f64;
}

fn num_triangles(g: &mut Graph) -> i32 {
    let mut count = 0;
    for i in g.V.clone().iter() {
        for j in g.V.clone().iter() {
            if i < j && edge_exists(g, *i, *j) {
                for k in g.V.clone().iter() {
                    if j < k && edge_exists(g, *i, *k) && edge_exists(g, *j, *k){
                            count = count + 1;
                    }
                }
            }
        }
    }
    return count;
}

fn num_triangles_1(g:  & mut Graph) -> usize {
    let mut count = 0;
    for i in g.V.clone().iter() {
        let neigh_i = get_neighbors(g, *i).clone();  // Get neighbors of i
        for j in neigh_i.clone().iter() {
            let neigh_j = get_neighbors(g, *j);

            let common_neighbors:HashSet<_> = neigh_i.intersection(neigh_j).collect();
            count = count + common_neighbors.len();
        }
    }

    return count;
}

fn num_4_cliques_err(g:  & mut Graph) -> usize {
    let mut count = 0;
    for i in g.V.clone().iter() {
        let neigh_i = get_neighbors(g, *i).clone();  // Get neighbors of i
        for j in neigh_i.iter() {
            let neigh_j = get_neighbors(g, *j).clone();
            for k in neigh_j.iter() {
                let neigh_k = get_neighbors(g, *j).clone();
                for l in neigh_k.iter() {
                    if i < j && j < k && k < l && neigh_i.contains(l) && neigh_k.contains(i) && neigh_j.contains(l) {
                        count = count + 1;
                    }
                }
            }
        }
    }

    return count;
}

fn num_near_4_cliques(g:  & mut Graph) -> usize {
    let mut count = 0;
    for i in g.V.clone().iter() {
        let neigh_i = get_neighbors(g, *i).clone();  // Get neighbors of i
        for j in neigh_i.iter() {
            let neigh_j = get_neighbors(g, *j).clone();
            for k in neigh_j.iter() {
                if ! edge_exists(g, *i, *k){
                    continue;
                }
                let neigh_k = get_neighbors(g, *k).clone();
                for l in neigh_k.iter() {
                    if l == i || l == j {
                        continue;
                    }
                    if edge_exists(g, *l, *j) && ! edge_exists(g, *i,*l){
                        count = count + 1;
                        //println!("{}{}{}{}",i,j,k,l);
                    }
                    /*if i < j && j < k && neigh_k.contains(i){
                        if !neigh_i.contains(l) {
                            num = num + 1;
                        }
                        if !neigh_j.contains(l){
                            num = num + 1;
                        }
                        if!neigh_k.contains(l)){
                            num = num + 1;
                        }
                        if num = 1{
                            count = count + 1;
                        }
                    } */
                }
            }
        }
    }

    return count/4;
}


fn num_4_cliques(g:  & mut Graph) -> usize {
    let mut count = 0;
    let num = 0;
    for i in g.V.clone().iter() {
        let neigh_i = get_neighbors(g, *i).clone();  // Get neighbors of i
        for j in neigh_i.iter() {
            if j < i {
                continue;
            }
            let neigh_j = get_neighbors(g, *j).clone();
            for k in neigh_j.iter() {
                if k < j || k < i || ! edge_exists(g, *i, *k){
                    continue;
                }
                let neigh_k = get_neighbors(g, *k).clone();
                for l in neigh_k.iter() {
                    if l < k || l < j || l < i || l == i || l == j {
                        continue;
                    }
                    if l == k {
                        println!("What?!! l = {} and k = {}", l,k);
                        println!("neighbors of k are: {:?}", neigh_k);
                        for t in 0.. 4 {
                            println!("{}: {:?}", t, get_neighbors(g, t));
                        }
                    }
                    if edge_exists(g, *l, *j) && edge_exists(g, *i,*l){
                        count = count + 1;
                        //println!("{}{}{}{}",i,j,k,l);
                    }
                    /*if i < j && j < k && neigh_k.contains(i){
                        if !neigh_i.contains(l) {
                            num = num + 1;
                        }
                        if !neigh_j.contains(l){
                            num = num + 1;
                        }
                        if!neigh_k.contains(l)){
                            num = num + 1;
                        }
                        if num = 1{
                            count = count + 1;
                        }
                    } */
                }
            }
        }
    }

    return count;
}

fn choose(n: i32, k: i32) -> i32 {
    let mut x = 1;
    let mut y = 1;
    for i in 0..k {
        x = x * (n - i);
    }
    for i in 1..k + 1 {
        y = y * i;
    }
    return x / y;
}
fn num_k_stars(g: &mut Graph, k: i32) -> i32 {
    let mut count = 0;
    for i in g.V.clone().iter() {
        let neigh = get_neighbors(g, *i).len();
        if neigh >= k as usize {
            count = count + choose(neigh as i32, k);
        }
    }
    return count;
}

fn noisy_num_k_stars(g: &mut Graph, k: i32, epsilon: f64) -> f64 {
    let stars = num_k_stars(g, k) as f64;
    let n = g.V.len();
    let noise = noise(choose((n - 1) as i32, k - 1) as f64 / epsilon);
    return stars + noise;
}

fn noise(b: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let mut u: f64 = rng.gen();
    u = u - 0.5;
    let mut su = -1.;
    let mut au = -u;
    if u > 0. {
        su = 1.;
        au = u;
    }
    let x = -b * su * (1. - 2. * u).ln();
    return x;
}

fn load_from_file(g: & mut Graph, filepath: & Path ){
    let open_result = File::open(filepath);
    match open_result {
        Ok(file) => {
            let lines = BufReader::new(file).lines();
            for x in lines {
                match x {
                    Ok(l) => {
                        //println!("{}", l);
                        let parts: Vec<&str> = l.split(",").collect();
                        if parts[0] == "from" {
                            continue;
                        }
                        let v1: i32 = i32::from_str(parts[0]).unwrap();
                        let v2: i32 = i32::from_str(parts[1]).unwrap();
                        //println!("Got the edge {} - {}", v1, v2);
                        add_edge(g, (v1,v2));
                    }
                    Err(e) => panic!("Error reading line: {}", e),
                }

            }
        }
        Err(e) => panic!("Unable to open file: {}", e),
    }
}
fn min(a: i32, b:i32) -> i32{
    if a < b{
        return a;
    }
    return b;
}
fn private_k_stars(g: & mut Graph, e: f64, dmax: i32) -> f64 {
    let mut tot = 0.;
    let  delta = choose(d,k-1) as f64;
    let n = g.V.len();
    for i in 0..n{
        let deg_i = get_neighbors(g,i as i32).len();
        let mut di = min(dmax, deg_i as i32);
        r = choose(di,k);
        ri = r + noise(delta/e);
        tot = tot + ri;
    }
    return tot;
}
fn main() {
    const N: i32 = 10000;
    const K: i32 = 10;
    const P: f64 = 0.01;

    println!("Initializing graph data structure.");
    let mut g = Graph {
        V: Default::default(),
        E: Default::default(),
    };

    let twitch_en = "D:/odrive/Amazon Cloud Drive/Diya/Research/DataSets/twitch/twitch/ENGB/twitch_en.csv";
    load_from_file(& mut g, Path::new(twitch_en));
    println!("Number of vertices = {}", g.V.len());
    //return;

    /*for i in 0..4 {
        add_vertex(& mut g,i);
    }
    let edges = vec![(0,1),(1,2),(2,3),(0,2),(1,3),(0,3),(4,0),(4,1),(4,2)];
    for e in edges {
        add_edge(& mut g, e);
    }

    //println!("{:?}",g.E);
    for t in 0.. 4 {
        println!("{}: {:?}", t, get_neighbors(&mut g, t));
    }
    */


    /* println!("Adding {} vertices.", N);

    for i in 0..N {
        add_vertex(&mut g, i);
    }

    println!("Making regular with K = {}", K);

    make_regular(&mut g, 10);

    println!("Rewiring with probability P = {}", P);
    rewire(&mut g, P);

    //println!("{:?}",g.E);
    println!(
        "Average shortest path from 0 = {}",
        avg_shortest_from_vertex(&mut g, 0)
    );
    */

    println!("Number of 3-stars");
    println!("Number of 3-stars = {}", num_k_stars(&mut g, 3));

    println!("Number of triangles_1");
    println!("Number of triangles_1 = {}", num_triangles_1(&mut g));

    println!("Number of 4-cliques");
    println!("Number of 4-cliques = {}", num_4_cliques(&mut g));

    println!("Number of near 4 cliques");
    println!("Number of near 4 cliques = {}", num_near_4_cliques(& mut g));

    //println!("{}", noise(0.1));

    /*let sensitivity = 10.0;
    let epsilon = 0.1;
    let noise_amt = noise(sensitivity/epsilon);

    println!("For sensitivity = {} and epsilon = {}, add noise = {}", sensitivity, epsilon, noise_amt);
    */
}
