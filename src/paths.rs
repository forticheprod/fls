use rayon::prelude::*;
use core;

pub struct Paths{
    paths: Vec<String>,
}
pub struct PathsPacked(Paths);

impl Paths{
    pub fn len(&self)->usize{
        self.paths.len()
    }
    pub fn iter(&self)->core::slice::Iter<'_, String>{
        self.paths.iter()
    }
    pub fn par_iter(&self)->rayon::slice::Iter<'_, String>{
        self.paths.par_iter()
    }
    pub fn new(paths: Vec<String>)->Paths{
        Paths{ paths}
    }
    pub fn to_vec(&self)->Vec<String>{
        self.paths.clone()
    }
    pub fn join(&self, sep: &str)->String{
        self.paths.join(sep)
    }
}
