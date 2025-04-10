use std::collections::{VecDeque, BinaryHeap};

pub fn topological_sort(n: usize, e: &Vec<(usize, usize)>) -> Option<Vec<usize>>{
    let mut edge = vec![Vec::new(); n];
    let mut cnt = vec![0; n];
    for &(u, v) in e{
        edge[u].push(v);
        cnt[v] += 1;
    }
    let mut res = Vec::with_capacity(n);
    let mut stack = VecDeque::new();
    for i in 0..n{
        if cnt[i]==0{
            stack.push_back(i);
        }
    }
    while let Some(p) = stack.pop_front(){
        res.push(p);
        for &nex in &edge[p]{
            cnt[nex] -= 1;
            if cnt[nex] == 0{
                stack.push_back(nex);
            }
        }
    }
    if res.len()==n{
        Some(res)
    } else {
        None
    }
}

pub fn lexical_topological_sort(n: usize, e: &Vec<(usize, usize)>) -> Option<Vec<usize>>{
    let mut edge = vec![Vec::new(); n];
    let mut cnt = vec![0; n];
    for &(u, v) in e{
        edge[u].push(v);
        cnt[v] += 1;
    }
    let mut res = Vec::with_capacity(n);
    let mut heap = BinaryHeap::new();
    for i in 0..n{
        if cnt[i]==0{
            heap.push(i);
        }
    }
    while let Some(p) = heap.pop(){
        res.push(p);
        for &nex in &edge[p]{
            cnt[nex] -= 1;
            if cnt[nex] == 0{
                heap.push(nex);
            }
        }
    }
    if res.len()==n{
        Some(res)
    } else {
        None
    }
}