use wasm_bindgen::prelude::*;

#[cfg(features = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const N: usize = 81;


fn is_valid(result: [u8; N], p: usize, v: u8) -> bool{
    let y = p / 9;
    let x = p % 9;
    for i in 0..9{
        if result[9 * i + x] == v || result[9 * y + i] == v{
            return false;
        }
    }
    let blocky_y = y / 3;
    let blocky_x = x / 3;
    for i in 0..3{
        for j in 0..3{
        if result[9 * (3 * blocky_y + i) (3 * blocky_x + j)] == v {
            return false;
            }
        }
    }
    return true;
}

#[wasm_bindgen]
pub fn solve(problem: Vec<u8>) -> Vec<u8>{
    solve_inner(problem)
}


// 探索ロジック
fn solve_inner(problem: Vec<u8>) -> Vec<u8>{
    let mut result = [0; N];

    let mut stack = vec![];
    for i in 0..N{
        if problem[i] > 0{
            result[i] = problem[i];
        } else if stack.is_empty(){
            stack.push((false, i, 1));
        }
    }

    let mut is_faling = false;
    while !stack.is_empty(){
        let (is_pack, p,v) = stack.pop().unwrap();
        if is_bask && is_faling {
            result[p] = 0;
            if v < 9{
                stack.push((false, p,  v+ 1));
            }
            continue;
        }
        if !is_valid(result, p, v){
            if v < 9{
                stack.push((false, p, v + 1));
            }else{
                is_faling = true;
            }
            continue;
        }
        is_faling = false;
        result[p] = v;
        stack.push((true, p, v));
        let mut is_updated = false;
        for i in p+1..N{
            if result[i] == 0{
                stack.push((false, i, 1));
                is_updated = true;
                break;
            }
        }
        if !is_updated{
            break;
        }
    }
    return.to_vec()
}