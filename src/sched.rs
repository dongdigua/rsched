use crate::proc::Proc;
use crate::prio;

use std::collections::LinkedList;
//use std::cmp::Ordering;
//use std::time::Duration;

use rand::prelude::*;

const TIMESLICE: u64 = 20;

#[derive (Debug)]
struct Rq {
    nr: usize,
    // TODO: remove nr, because, why not
    stair: LinkedList<Entity>,
}

#[derive (Debug)]
struct Entity {
    normal_prio: u8, // change if it reached bottom
    prio: u8,        // to sort
    timeslice: u64,  // time it should run and stop (park)
    //start_time: u64,
    proc: Proc,
    runable: bool,
}

impl Entity {
    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        let prio: u8 = rng.gen_range(prio::MAX_RR_PRIO..prio::MAX_PRIO); 
        Entity {
            normal_prio: prio,
            prio,
            timeslice: TIMESLICE,
            proc: Proc::new(),
            runable: true,
        }
    }
}

impl Rq {
    pub fn new(entities: &mut Vec<Entity>) -> Self {
        let mut ll = LinkedList::new();
        entities.sort_by(|a, b| b.prio.partial_cmp(&a.prio).unwrap());

        let len = entities.len();
        for _i in 1..=len {
            let ent = entities.pop().unwrap();
            ll.push_front(ent);
        }

        drop(entities);
        Rq {
            nr: len,
            stair: ll,
        }
    }

    pub fn schedule(&mut self) {
        // now first implement a single-cpu one
        let mut curr = self.stair.pop_front().unwrap();
        if self.nr == 1 {
            curr.timeslice += curr.timeslice;
            curr.normal_prio -= 1;
            curr.prio = curr.normal_prio;
        } else {
            let ll = &mut self.stair;
            curr.prio -= 1;
            let mut i_prio = 0;

            // well, many of linkedlist function are nightly, but no
            ll.iter()
                .for_each(|i| {
                    if i.prio < curr.prio { i_prio = 1 }
                });
            // for i in 0..self.nr {
            //     if curr.prio < ll[i] { i_prio = i }
            // };
            // Returns everything after the given index,
            // including the index.
            let mut split = ll.split_off(i_prio);
            split.push_front(curr);

            ll.append(&mut split);
        }
    }

    pub fn run_one(&mut self) {
        let mut curr = self.stair.pop_front().unwrap();
        curr.runable = ! curr.proc.run(curr.timeslice);
        if curr.runable { self.stair.push_front(curr) };
    }

    pub fn insert() {}
}

#[cfg(test)]
use super::*;

#[test]
fn sort_random_entity() {
    let mut entities: Vec<Entity> =
        (1..5)
        .into_iter()
        .map(|_|  Entity::new_random() )
        .collect();
    let rq = Rq::new(&mut entities);
    println!("{:#?}", rq);
}
