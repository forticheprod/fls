use rayon::prelude::*;
use core;

pub struct Paths{
    data: Vec<String>,
}


impl Paths{
    pub fn len(&self)->usize{
        self.data.len()
    }
    pub fn iter(&self)->core::slice::Iter<'_, String>{
        self.data.iter()
    }
    pub fn par_iter(&self)->rayon::slice::Iter<'_, String>{
        self.data.par_iter()
    }
    pub fn new(data: Vec<String>)->Paths{
        Paths{ data}
    }
    pub fn new_empty()->Paths{
        Paths{ data: Vec::new()}
    }
    pub fn to_vec(&self)->Vec<String>{
        self.data.clone()
    }
    pub fn join(&self, sep: &str)->String{
        self.data.join(sep)
    }
}

pub struct PathsPacked{
    paths: Paths,
    metadata: Paths
}

impl PathsPacked {
    pub fn new_empty()->PathsPacked{
        PathsPacked { paths: Paths::new_empty(), metadata: Paths::new_empty() }
    }
    pub fn push_paths(&mut self, path:String){
        self.paths.data.push(path)
    }
    pub fn push_metadata(&mut self, path:String){
        self.metadata.data.push(path)
    }
    pub fn join(&self, sep: &str)->String{
        let mut main_vec = self.paths.data.clone();
        main_vec.extend(self.metadata.data.clone());
        main_vec.join(sep)
    }
}