use crate::proc::Proc;
use crate::prio;

use std::collections::LinkedList;
use std::cmp::Ordering;

use rand::prelude::*;

const TIMESLICE: u64 = 20;

#[derive (Debug)]
struct Rq {
    nr_running: usize,
    stair: LinkedList<Entity>,
}

#[derive (Debug)]
struct Entity {
    prio: u8,      // to sort
    timeslice: u64, // time it should run and stop (park)
    //start_time: u64,
    proc: Proc,
}

// impl PartialOrd for Entity {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         if self.prio == other.prio { Some(Ordering::Equal) }
//         else if self.prio > other.prio { Some(Ordering::Greater) }
//         else if self.prio < other.prio { Some(Ordering::Less) }
//         else { None }
//     }
// }
// impl PartialEq for Entity {
//     fn eq(&self, other: &Self) -> bool {
//         self.prio == other.prio && self.timeslice == other.timeslice
//     }
// }

impl Entity {
    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        let prio: u8 = rng.gen_range(prio::MAX_RR_PRIO..prio::MAX_PRIO); 
        Entity {
            prio,
            timeslice: TIMESLICE,
            proc: Proc::new(),
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
            nr_running: len,
            stair: ll,
        }
    }

    pub fn schedule(&mut self) {
        // TODO I don't find a sort() method in std, so use a rather stupid one
        
    }
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
