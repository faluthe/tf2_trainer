use std::{ffi::CStr, collections::HashMap};
use lazy_static::lazy_static;

use crate::{interfaces::INTERFACES, sdk::RecvTable};

// Make netvar macro like in romulus

// lazy_static! {
//     pub static ref NETVARS: HashMap<usize, usize> = unsafe { dump() };
// }

// fn netvar_sum(table_name: &str, netvar_name: &str) -> usize {
//     let mut sum = 0;

//     for (i, c) in table_name.chars().enumerate() {
//         sum += c as usize * i;
//     }
//     for (i, c) in netvar_name.chars().enumerate() {
//         sum += c as usize * i;
//     }

//     println!("Sum of {}/{} is 0x{:#x}", table_name, netvar_name, sum);

//     sum
// }

// unsafe fn dump_table(recv_table: *mut RecvTable, table_name: &str, hashmap: &mut HashMap<usize, usize>) {
//     for i in 0..(*recv_table).nprops {
//         let i: isize = i.try_into().unwrap();
//         let prop = ((*recv_table).props).offset(i);
//         let netvar_name = CStr::from_ptr((*prop).var_name).to_str().unwrap();
//         let offset: usize = (*prop).offset.try_into().unwrap();

//         hashmap.insert(netvar_sum(table_name, netvar_name), offset);

//         if !(*prop).data_table.is_null() {
//             dump_table((*prop).data_table, table_name, hashmap);
//         }
//     }
// }

// pub unsafe fn dump() -> HashMap<usize, usize> {
//     let mut hashmap = HashMap::new();

//     let mut node = INTERFACES.client.get_all_classes();

//     while !node.is_null() {
//         let recv_table = (*node).recv_table;
//         let table_name = CStr::from_ptr((*recv_table).net_table_name)
//             .to_str()
//             .unwrap();

//         dump_table(recv_table, table_name, &mut hashmap);

//         node = (*node).next;
//     }

//     hashmap
// }

pub unsafe fn get(table_name: &str, netvar_name: &str) -> Option<usize> {
    let mut node = INTERFACES.client.get_all_classes();

    while !node.is_null() {
        let recv_table = (*node).recv_table;
        let node_tblname = {
            let p = (*recv_table).net_table_name;
            CStr::from_ptr(p)
        }.to_str().unwrap();

        if table_name == node_tblname {
            return check_table(recv_table, netvar_name)
        }

        node = (*node).next;
    }

    None
}

unsafe fn check_table(recv_table: *mut RecvTable, netvar_name: &str) -> Option<usize> {
    for i in 0..(*recv_table).nprops {
        let i: isize = i.try_into().unwrap();
        let prop = ((*recv_table).props).offset(i);
        let var_name = CStr::from_ptr((*prop).var_name).to_str().unwrap();
        
        if netvar_name == var_name {
            return Some((*prop).offset.try_into().unwrap())
        }

        if !(*prop).data_table.is_null() {
            match check_table((*prop).data_table, netvar_name) {
                Some(o) => {
                    let offset: usize = (*prop).offset.try_into().unwrap();
                    return Some(offset + o)
                },
                None => (),
            }

        }
    }

    None
}