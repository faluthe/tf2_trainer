use std::{ffi::CStr, collections::HashMap};
use lazy_static::lazy_static;

use crate::{interfaces::INTERFACES, sdk::RecvTable};

pub fn get(table_name: &str, netvar_name: &str) -> Option<usize> {
    lazy_static! { static ref NETVARS: HashMap<usize, usize> = unsafe { dump() }; };
    NETVARS.get(&netvar_sum(table_name, netvar_name)).copied()
}

unsafe fn dump() -> HashMap<usize, usize> {
    println!("Dumping netvars...");
    let mut hashmap = HashMap::new();

    let mut node = INTERFACES.client.get_all_classes();

    while !node.is_null() {
        let recv_table = (*node).recv_table;
        let table_name = CStr::from_ptr((*recv_table).net_table_name)
            .to_str()
            .unwrap();

        dump_table(recv_table, table_name, &mut hashmap);

        node = (*node).next;
    }

    println!("Netvar map created!");
    hashmap
}

unsafe fn dump_table(recv_table: *mut RecvTable, table_name: &str, hashmap: &mut HashMap<usize, usize>) {
    for i in 0..(*recv_table).nprops {
        let i: isize = i.try_into().unwrap();
        let prop = ((*recv_table).props).offset(i);
        let netvar_name = CStr::from_ptr((*prop).var_name).to_str().unwrap();
        let offset: usize = (*prop).offset.try_into().unwrap();

        hashmap.insert(netvar_sum(table_name, netvar_name), offset);

        if !(*prop).data_table.is_null() {
            dump_table((*prop).data_table, table_name, hashmap);
        }
    }
}

fn netvar_sum(table_name: &str, netvar_name: &str) -> usize {
    let mut sum = 0;

    for (i, c) in table_name.chars().enumerate() {
        sum += c as usize * i;
    }
    for (i, c) in netvar_name.chars().enumerate() {
        sum += c as usize * i;
    }

    sum
}