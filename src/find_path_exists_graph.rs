use std::collections::HashMap;

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut visited = vec![false; n as usize];
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut dfs = Vec::new();
    for e in edges {
        let k = e[0];
        let v = e[1];
        map.entry(k).or_insert(Vec::new()).push(v);
        map.entry(v).or_insert(Vec::new()).push(k);
    }
    dfs.push(source);
    while let Some(cn) = dfs.pop() {
        visited[cn as usize] = true;
        if cn == destination {
            return true;
        }
        for v in map.get(&cn).unwrap().to_vec() {
            if !visited[v as usize] {
                dfs.push(v);
            }
        }
    }
    return false;
}
