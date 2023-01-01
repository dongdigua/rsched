use crate::proc::Proc;
use crate::prio;

use std::collections::LinkedList;
//use std::cmp::Ordering;
//use std::time::Duration;

use rand::prelude::*;

const TIMESLICE: u64 = 20;

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
    pub fn new(prio: u8) -> Self {
        Entity {
            normal_prio: prio,
            prio,
            timeslice: TIMESLICE,
            proc: Proc::new(),
            runable: true,
        }
    }

    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        let prio: u8 = rng.gen_range(prio::MAX_RR_PRIO..prio::MAX_PRIO); 
        Entity::new(prio)
    }
}

#[derive (Debug)]
struct Rq {
    stair: LinkedList<Entity>,
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
        Rq { stair: ll }
    }

    pub fn schedule(&mut self) {
        // now first implement a single-cpu one
        let mut curr = self.stair.pop_front().unwrap();
        if self.stair.len() <= 1 {
            curr.timeslice += curr.timeslice;
            curr.normal_prio -= 1;
            curr.prio = curr.normal_prio;
        } else {
            let ll = &mut self.stair;
            curr.prio -= 1;
            insert_ab(ll, curr, InsertOption::After);
        }
    }

    pub fn run_one(&mut self) {
        let mut curr = self.stair.pop_front().unwrap();
        curr.runable = ! curr.proc.run(curr.timeslice);
        if curr.runable { self.stair.push_front(curr) };
    }

    pub fn insert(&mut self, entity: Entity) {
        let ll = &mut self.stair;
        insert_ab(ll, entity, InsertOption::Before);
    }
}

enum InsertOption { Before, After }

// after of before
// notice: it is a decrementl list
#[inline]
fn insert_ab(ll: &mut LinkedList<Entity>, entity: Entity, option: InsertOption) {
    let mut idx = 0;
    let mut iter = ll.iter();
    
    for i in 0..ll.len() {
        let cur = iter.next().unwrap(); // uhh, we don't have get()
        match option {
            InsertOption::Before =>
                if cur.prio <= entity.prio { idx = i ; break },
            InsertOption::After =>
                // 2, 2, 1 (1)
                if cur.prio <  entity.prio { idx = i ; break },
        };
    }
    if let option = InsertOption::After { if idx == 0 { idx = ll.len() }} // important

    // Returns everything after the given index,
    // including the index.
    let mut split = ll.split_off(idx);
    split.push_front(entity);

    ll.append(&mut split);
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
    println!("random: {:#?}", rq);
}

#[test]
fn insert_entity() {
    let mut entities = vec![
        Entity::new(103),
        Entity::new(102),
        Entity::new(102),
        Entity::new(101),
    ];
    let mut rq = Rq::new(&mut entities);
    rq.insert(Entity::new(101));
    rq.insert(Entity::new(102));
    println!("insert: {:#?}", rq);
}

#[test]
fn sched_entity() {
    let mut entities = vec![
        Entity::new(103),
        Entity::new(102),
        Entity::new(102),
        Entity::new(101),
    ];
    let mut rq = Rq::new(&mut entities);
    rq.schedule();
    rq.schedule();
    println!("schedule: {:#?}", rq);
}
