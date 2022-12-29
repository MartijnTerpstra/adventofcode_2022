use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    compare_list_rows();
    find_divider_packets();
}

fn find_divider_packets() {
    let divider_key: usize = read_lines("input.txt")
        .iter()
        .filter(|e| !e.is_empty())
        .map(|e| (List::new(e), false))
        .chain(vec!["[[2]]", "[[6]]"].iter().map(|e| (List::new(e), true)))
        .sorted()
        .enumerate()
        .map(|(index, (_, is_divider))| if is_divider { index + 1 } else { 1 })
        .product();

    println!("Decoder key: {}", divider_key);
}

fn compare_list_rows() {
    let index_sum: usize = read_lines("input.txt")
        .iter()
        .chunks(3)
        .into_iter()
        .enumerate()
        .map(|(i, mut e)| {
            let l = List::new(e.next().unwrap());
            let r = List::new(e.next().unwrap());
            if l < r {
                i + 1
            } else {
                0
            }
        })
        .sum();

    println!("Sum of indices that compared correctly: {}", index_sum)
}

struct ListItem {
    elements: Vec<usize>,
    parent: usize,
}

enum ListNode {
    NumberItem(i32),
    ListItem(ListItem),
}

struct List {
    data: Vec<ListNode>,
}

impl List {
    fn new(row: &str) -> List {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\[|\]|\d+)").unwrap();
        }

        let mut current_node = usize::MAX;
        let mut data = Vec::new();

        for m in RE.captures_iter(row) {
            match &m[1] {
                "[" => {
                    let list_index = data.len();
                    data.push(ListNode::ListItem(ListItem {
                        elements: Vec::new(),
                        parent: current_node,
                    }));
                    if current_node != usize::MAX {
                        let elems = match &mut data[current_node] {
                            ListNode::ListItem(item) => &mut item.elements,
                            _ => panic!("Unknown type"),
                        };
                        elems.push(list_index);
                    }
                    current_node = data.len() - 1;
                }
                "]" => {
                    current_node = match &data[current_node] {
                        ListNode::ListItem(item) => item.parent,
                        _ => panic!("Unknown type"),
                    }
                }
                number => {
                    let number_index = data.len();
                    data.push(ListNode::NumberItem(number.parse().unwrap()));
                    let elems = match &mut data[current_node] {
                        ListNode::ListItem(item) => &mut item.elements,
                        _ => panic!("Unknown type"),
                    };
                    elems.push(number_index);
                }
            }
        }

        return List { data };
    }

    fn cmp_impl(
        l_nodes: &[ListNode],
        l_index: usize,
        r_nodes: &[ListNode],
        r_index: usize,
    ) -> Option<Ordering> {
        return match (&l_nodes[l_index], &r_nodes[r_index]) {
            (ListNode::ListItem(l), ListNode::ListItem(r)) => {
                List::cmp_impl_lists(l_nodes, r_nodes, &l.elements, &r.elements)
            }
            (ListNode::NumberItem(l), ListNode::NumberItem(r)) => {
                if l == r {
                    return None;
                } else {
                    return Some(l.cmp(r));
                }
            }
            (ListNode::NumberItem(_), ListNode::ListItem(r)) => {
                List::cmp_impl_lists(l_nodes, r_nodes, &vec![l_index], &r.elements)
            }
            (ListNode::ListItem(l), ListNode::NumberItem(_)) => {
                List::cmp_impl_lists(l_nodes, r_nodes, &l.elements, &vec![r_index])
            }
        };
    }

    fn cmp_impl_lists(
        l_nodes: &[ListNode],
        r_nodes: &[ListNode],
        l_elems: &[usize],
        r_elems: &[usize],
    ) -> Option<Ordering> {
        let min_len = l_elems.len().min(r_elems.len());

        for i in 0..min_len {
            let result = List::cmp_impl(l_nodes, l_elems[i], r_nodes, r_elems[i]);
            if result.is_some() {
                return result;
            }
        }

        if l_elems.len() != r_elems.len() {
            return Some(l_elems.len().cmp(&r_elems.len()));
        }

        return None;
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Eq for List {}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        return List::cmp_impl(&self.data, 0, &other.data, 0).unwrap_or(Ordering::Equal);
    }
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file)
        .lines()
        .filter_map(|e| e.ok())
        .collect_vec();
}
