//! Genenerational Indices
//!
//! This is a simple implementation of generational indices. It will serve
//! as the basis for you to implement your own Entity Component System. It uses
//! a Mutex so that it can be thread safe.
//!
//! Please see https://github.com/flajann2/gen-indices for more details.

#![warn(missing_docs)]

extern crate num;

use std::{result::Result,
          hash::Hash,
          vec::Vec,
          ops::AddAssign,
          marker::Copy,
          sync::Arc,
          sync::Mutex};

use num::{Num, zero, one};

/// GenIndex
///
/// This is the basic key for your indexes. It includes
/// a generational number so that if an entity is deleted
/// and a new one is entered, it won't confuse the new one with
/// the old one on lookups.
///
#[derive(Hash, Debug, PartialEq, Copy, Clone)]
pub struct GenIndex<I: Num + AddAssign + Copy,
                    G: Num + AddAssign + Copy> {
    index: I,
    generation: G
}

impl<I: Num + AddAssign + Copy,
     G: Num + AddAssign + Copy> GenIndex<I, G> {
    /// return the index number for GenIndex.
    pub fn get_index(&self) -> I { self.index }

    /// return the generation number for GenIndex.
    pub fn get_generation(&self) -> G { self.generation }
}

/// GenIndexEntitySet
/// This maintains the "state" for your entities. It is designed with
/// a Mutex, so that it is rendered thread safe.
///
/// Example:
///
/// ```
/// extern crate gen_indices;
/// extern crate num;
/// 
/// use gen_indices::*;
/// use num::{Num, zero, one};
///
/// let gi = GenIndexEntitySet::<u64, u64>::new();
///
/// // first index
/// let idx1 = gi.lock().unwrap().next_index();
/// println!("first: {:?}", idx1);
///
/// // second index
/// let idx2 = gi.lock().unwrap().next_index();
/// println!("first: {:?}", idx2);
///
/// // delete first index and then get next index
/// if let Err(e) = gi.lock().unwrap().delete_index(idx1) {
///     println!("Error: {}", e);
/// }
/// let idx3 = gi.lock().unwrap().next_index();
/// println!("first: {:?}", idx3);
/// ```
#[derive(Hash, Debug, PartialEq, Clone)]
pub struct GenIndexEntitySet<I: Num + AddAssign + Copy,
                             G: Num + AddAssign + Copy> {
    index_note: I,
    deleted: Vec<GenIndex<I, G>>,
}

impl<I: Num + AddAssign + Copy,
     G: Num + AddAssign + Copy> GenIndexEntitySet<I, G> {

    /// Create a new GenIndexEntitySet object, wrapped with
    /// a Mutex to allow for thread safety.
    pub fn new() -> Arc<Mutex<GenIndexEntitySet<I, G>>> {
        Arc::new(Mutex::new(GenIndexEntitySet {
            index_note: zero(),
            deleted: vec!{}
        }))
    }

    /// allocate and provide a "new" index. If an old
    /// index was deleted, that index is reused, with the
    /// generation number incremented, so that references
    /// to the deleted entity are not found.
    ///
    /// You are responsible for the corresponding maitenence in your
    /// ECS.
    pub fn next_index(&mut self) -> GenIndex<I, G> {
        if self.deleted.is_empty() {
            let g = GenIndex{index: self.index_note, generation: zero()};
            self.index_note += one();
            g
        } else {
            let mut oldidx = self.deleted.pop().unwrap();
            oldidx.generation += one();
            oldidx
        }
    }

    /// Delete an entity's index. You will be responsible for the cleanup
    /// in the corresponding ECS.
    pub fn delete_index(&mut self, gi: GenIndex<I, G>) -> Result<(), &'static str> {
        self.deleted.push(gi);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::thread::*;
    use std::vec::*;    
    use super::*;
    
    const THREADS: u32 = 100;
    
    #[test]
    fn test_gen_index_generation() {
        let gi = GenIndexEntitySet::<u64, u64>::new();
        let chk = GenIndex::<u64, u64> {index: zero(), generation: zero()};

        // first index
        let idx1 = gi.lock().unwrap().next_index();
        assert_eq!(chk, idx1);

        // second index
        let mut chk2 = chk.clone();
        chk2.index += 1;
        let idx2 = gi.lock().unwrap().next_index();
        assert_eq!(chk2, idx2);

        // delete first index and then get next index
        let mut chk3 = chk.clone();
        chk3.generation += 1;
        if let Err(e) = gi.lock().unwrap().delete_index(idx1) {
            println!("Error: {}", e);
        }
        let idx3 = gi.lock().unwrap().next_index();
        assert_eq!(chk3, idx3);        
    }

    #[test]
    fn test_multithreaded_index_generation() {
        // TODO: this test is to see if we get any seg faults-- since it
        // TODO: is asynchronous, it makes it difficult to test for anything
        // TODO: more specific. One may look at the output by uncommenting
        // TODO: the println!() below. Sucks, I know. I'll do something better
        // TODO: later.
        let gi = GenIndexEntitySet::<u64, u64>::new();
        let mut threads = Vec::new();

        for _ in 0..THREADS {
            let cgi = gi.clone();
            threads.push(spawn(move || {
                let idx = {
                    let mut gi = cgi.lock().unwrap();
                    gi.next_index()
                };
                //println!("thread_{}: {:?}", i, idx);
                if let Err(e) = cgi.lock().unwrap().delete_index(idx) {
                    println!("error: {:?}", e);
                }
            }));
        }
        
        for j in threads {
            if let Err(e) =  j.join() {
                println!("thead_error: {:?}", e);
            }
        }
    }   
}
