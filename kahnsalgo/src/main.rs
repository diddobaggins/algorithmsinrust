use std::collections::HashMap;
fn main() {
    // collaborated on by Justin, Joying, Shivani, and Muhammad
    /*
    Studying algorithms is fun by itself, but what are these algorithms useful for? What is exactly the point of sorting an acyclic graph?

    In a topological sort, nodes are sorted such that every "node" cannot be directed to by nodes placed after it. This logic resembles real world tasks, such as job scheduling.

    If you have a number of jobs with some order requirements, topoological sorting would give you the order in which you should do the jobs.

    For example, if you must eat before you brush you teeth, and cook before you eat, the schedule for your morning would look like cooking, eating, then teeth cleaning.

    Topological sort algorithms are directly applied in real world problems as well. It can be used to resolve dependencies, which is extremely important in programming.

    Furthermore, once a graph is sorted, it can allow for the computation of the shortest path in the graph, which has important implications for pathfinding. This is similar to how good number sorting algorithms are important because they enable searching.

    Although not immediately obvious, topological sorting plays an unnoticed role in our lives.
    */
    let mut l: Vec<usize> = Vec::new();

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    // graph.insert(1, vec![2]);
    // graph.insert(2, vec![3, 4, 5]);
    // graph.insert(3, vec![5]);
    // graph.insert(4, vec![5]);
    // graph.insert(5, vec![6]);
    // graph.insert(6, vec![]);
    // graph.insert(7, vec![4]);

    graph.insert(1, vec![2]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![4]);
    graph.insert(4, vec![]);
    let mut s = vec![];
    let mut values = vec![];

    for (_k, v) in &graph {
        for i in v {
        values.push(i);
        }
    }

    for &key in graph.keys() {
        if values.contains(&&key) == false {
            s.push(key);
        }
    }

    while s.len() != 0 {
        let n = s.remove(0);
        l.push(n);
        let values = graph[&n].clone();
        for value in values {
            let mut graph_update = graph[&n].clone();
            graph_update.retain(|&x| x != value);
            graph.insert(n, graph_update);
            let mut all = vec![];
            for (_k, v) in &graph {
                for i in v {
                all.push(i);
                }
            }
            let is_orphan = all.contains(&&value) == false;
            if is_orphan {
                s.push(value);
            }
        }
    }

    let mut len_of_map = 0;
    for (_k, v) in graph {
        len_of_map += v.len();
    }

    if len_of_map > 0 {
        println!("This is a cyclic graph!");
    }
    else {
        println!("Here is your sorted graph {:?}", l)
    }
}
