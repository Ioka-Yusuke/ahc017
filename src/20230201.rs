use proconio::input;
use std::time::Instant;

fn main() {
    // get start time
    let _start = Instant::now();

    // 全体値　最短
    let mut cnt = 0;

    // input
    input! {
        n: usize,
        m: usize,
        d: usize,
        _k: usize,
        uvw: [(usize, usize, usize); m],
        _xy: [[f64; 2]; n],
    }

    // root map
    let mut root_weight = vec![vec![1_000_000_000; n]; n];
    let mut short_roots = vec![vec![vec![]; n]; n];
    for (u, v, w) in &uvw {
        root_weight[(u-1) as usize][(v-1) as usize] = *w;
        root_weight[(v-1) as usize][(u-1) as usize] = *w;
    }
    // day_num
    let mut day_num = vec![0; d];
    // now_ans
    let mut now_ans = vec![0; m];
    for i in 0..m {
        let u = uvw[i].0;
        let v = uvw[i].1;

        let short_root = short_roots[(u-1) as usize][(v-1) as usize].clone();
        let mut random_seed = vec![];
        for i in 0..d {
            random_seed.push(i);
        }
        
        for root in short_root {
            random_seed.retain(|&x| x != root);
        }

        if random_seed.len() == 0 {
            for i in 0..d {
                random_seed.push(i);
            }
        } 

        
        let mut min_v = 100000000;
        for i in &random_seed {
            if day_num[*i] < min_v {
                min_v = day_num[*i];
            }
        }
        let mut day = 1_000_000_000;
        for i in &random_seed {
            if day_num[*i] == min_v {
                day = *i;
            }
        }
        day_num[day] += 1;
    
        let mut stop_days_u = vec![];
        let mut stop_days_v = vec![];
        for j in 0..n {
            if root_weight[(u-1) as usize][j] != 1_000_000_000 {
                stop_days_u.push(j);
            }
            
            if root_weight[(v-1) as usize][j] != 1_000_000_000 {
                stop_days_v.push(j);
            }
        } 

        let mut rw = vec![];
        for u_i in &stop_days_u {
            for v_i in &stop_days_v {
                if u_i == v_i {
                    let sum_w: usize = root_weight[*(u_i)][(u-1) as usize]+root_weight[*(u_i)][(v-1) as usize];
                    rw.push((u_i, sum_w));
                }
            }
        }


        if rw.len() != 0 {
            cnt += 1;
            rw.sort_by(|a, b| a.1.partial_cmp(&(b.1)).unwrap());
            let halfway_point: usize = *rw[0].0;
            short_roots[halfway_point][(u-1) as usize].push(day);
            short_roots[halfway_point][(v-1) as usize].push(day);
            short_roots[(u-1) as usize][halfway_point].push(day);
            short_roots[(v-1) as usize][halfway_point].push(day);
        }
        now_ans[i] = (day) as usize;
    }

    // loop
    // let mut cnt = 0;
    // loop {
    //     cnt += 1;
    //     // 乱数
    //     let random_way: usize = rand::thread_rng().gen_range(0, m);
    //     let random_u: usize = uvw[random_way].0 - 1;
    //     let random_v: usize = uvw[random_way].1 - 1;
    //     let random_day: usize = rand::thread_rng().gen_range(0, d);
    //     let now_day = now_ans[random_way];
    //     let fut_day = random_day;

    //     // fut_ans
    //     let mut fut_ans = now_ans.clone();
    //     fut_ans[random_way] = fut_day;

    //     // 合計StopRoot
    //     let mut now_stop_root = 0;
    //     let mut fut_stop_root = 0;
        
    //     for i in 0..n {
    //         now_stop_root += uvw_map[now_day][random_u][i] + uvw_map[now_day][random_v][i];
    //         fut_stop_root += uvw_map[fut_day][random_u][i] + uvw_map[fut_day][random_v][i];
    //     }
        
    //     if fut_stop_root < now_stop_root {
    //         now_ans[random_way] = fut_day;
    //         uvw_map[now_day][random_u][random_v] = 0;
    //         uvw_map[fut_day][random_u][random_v] = STOP_VALUE;
    //     }
    
    //     // get end time
    //     let end = start.elapsed();
    //     let end_sec = end.as_secs_f64();

    //     if cnt > 10000 {
    //         println!("{}", end_sec);
    //         break;
    //     }
    // }
    
    // output
    for i in &now_ans {
        print!("{} ", i+1)
    }
}
