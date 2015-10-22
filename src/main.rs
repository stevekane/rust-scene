extern crate regex;
extern crate cgmath;

use std::hash::Hash;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use cgmath::{Vector3, Vector2, Matrix4};

mod geometry;
mod loading;
mod parser;

use self::loading::*;
use self::geometry::*;
use self::parser::tokenizer::*;

#[derive(Debug)]
enum NodeData <'a> {
    Root,
    Group,
    Mesh(&'a Mesh)
}

#[derive(Debug)]
struct Node <'a> {
    data: NodeData<'a>,
    children: Vec<Node<'a>>,
    local_mat: Matrix4<f32>,
    world_mat: Matrix4<f32>,
}

impl <'a> Node <'a> {
    fn default(data: NodeData<'a>, children: Vec<Node<'a>>) -> Node<'a> {
        Node {
            data: data,
            children: children,
            local_mat: Matrix4::identity(),
            world_mat: Matrix4::identity(),
        } 
    }
}

trait CacheOfType <K, V> where K: Hash + Eq {
    fn from_tuples(vals: Vec<(K, V)>) -> HashMap<K, V>;
}

impl <K, V> CacheOfType <K, V> for HashMap <K, V> where K: Hash + Eq {
    fn from_tuples(vals: Vec<(K, V)>) -> HashMap<K, V> {
        let mut c = HashMap::new();

        for (key, val) in vals {
            c.insert(key, val);
        }
        c 
    }
}


fn main() {
    if let Ok(f) = File::open("assets/plane.kane") {
        let mut t = Tokenizer::from_reader(f).peekable();

        println!("{:?}", t.peek());
        for token in t {
            println!("{:?}", token);
            if token.is_err() { break }
        }
    }
}
