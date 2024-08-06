use std::collections::HashSet;

pub fn run() {



    fn find_subsets(set: HashSet<&str>) -> Vec<HashSet<&str>> {
        let mut visited: Vec<HashSet<&str>> = Vec::new();
        visited.push(set.clone());

        recursive_find_subset(&set, &mut visited);
        visited
    }

    fn recursive_find_subset<'a>(set: &HashSet<&'a str>, visited: &mut Vec<HashSet<&'a str>>) {
        if set.is_empty() {
            return;
        }

        for v in set.iter() {
            let mut subset: HashSet<&'a str> = set.clone();
            if subset.remove(v) {
                if !visited.contains(&subset) {
                    recursive_find_subset(&subset, visited);
                    visited.push(subset);
                }
            }
        }
    }



    let mut set = HashSet::new();
    set.insert("a");
    set.insert("b");
    set.insert("c");
    set.insert("d");
    set.insert("e");
    let subsets = find_subsets(set);
    dbg!(&subsets);

}