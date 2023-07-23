use proconio::input;
use std::time::Instant;
// 提出予定

fn main() {
    // get start time
    let start = Instant::now();
    
    // input
    input! {
        n: usize,
        m: usize,
        d: usize,
        k: usize,
        uvw: [(usize, usize, usize); m],
        _xy: [[f64; 2]; n],
    }
    
    // 初期設定
    let mut root_weight = vec![vec![1_000_000_000; n]; n];
    let mut short_roots = vec![vec![vec![]; n]; n];
    let mut root_m = vec![vec![1_000_000_000; n]; n];

    // day_num
    let mut day_num = vec![0; d];
    // now_ans
    let mut now_ans = vec![1_000_000_000; m];
    
    // loop内
    const STOP_VALUE: usize = 1;
    let mut daily_uvw_map = vec![vec![vec![0; n]; n]; d];
    let mut conect_roots = vec![];

    for i in 0..uvw.len() {
        let u = uvw[i].0;
        let v = uvw[i].1;
        let w = uvw[i].2;

        root_weight[(u-1) as usize][(v-1) as usize] = w;
        root_weight[(v-1) as usize][(u-1) as usize] = w;
        root_m[(u-1) as usize][(v-1) as usize] = i;
        root_m[(v-1) as usize][(u-1) as usize] = i;
    }

    // kill dead end
    let mut dead_end_m = vec![];
    for i in 0..n {
        let mut root = vec![];
        for j in 0..n {
            if root_weight[(i) as usize][(j) as usize] != 1_000_000_000 {
                root.push([root_m[i][j], j]);
            }
        }

        if root.len() == 2 {
            let mut dead_end_point = vec![root[0][0], root[1][0]];
            let mut dead_end_num   = vec![root[0][1], root[1][1]];
            loop {
                if dead_end_num.len() == 0 {
                    dead_end_m.push(dead_end_point);
                    break;
                }

                let u_i  = dead_end_num.pop();
                let u_ii = u_i.unwrap();

                let mut c_root = vec![];
                for j in 0..n {
                    if root_weight[u_ii][j] != 1_000_000_000 {
                        c_root.push([root_m[u_ii][j], j]);
                    }
                }
                if c_root.len() == 2 {
                    for [r_m, d_point] in c_root {
                        let mut flag = 0;
                        for num in &dead_end_point {
                            if num == &r_m {
                                flag = 1;
                            }
                        }
                        if flag == 0 {
                            dead_end_point.push(r_m);
                            dead_end_num.push(d_point);
                        }
                    }
                }
            }
        }
    }

    for l in &dead_end_m {
        let mut num = 0;
        for dd in &*l {
            now_ans[*(dd)] = num;
            num += 1;
            num = num % k;
            day_num[*(dd)] += 1;
        }
    }
    
    // ２頂点最短
    for i in 0..m {
        let u = uvw[i].0;
        let v = uvw[i].1;

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

        if rw.len() != 0 && now_ans[i] == 1_000_000_000 {
            rw.sort_by(|a, b| a.1.partial_cmp(&(b.1)).unwrap());
            let halfway_point: usize = *rw[0].0;
            
            let short_root = short_roots[(u-1) as usize][(v-1) as usize].clone();
            let mut random_seed = vec![];
            for i in 0..d {
                random_seed.push(i);
            }
            
            for root in short_root {
                random_seed.retain(|&x| x != root);
            }

            if now_ans[(root_m[(halfway_point) as usize][(v-1) as usize]) as usize] != 1_000_000_000 {
                random_seed.retain(|&x| x != now_ans[(root_m[(halfway_point) as usize][(v-1) as usize]) as usize]);
            }

            if now_ans[(root_m[(halfway_point) as usize][(u-1) as usize]) as usize] != 1_000_000_000 {
                random_seed.retain(|&x| x != now_ans[(root_m[(halfway_point) as usize][(u-1) as usize]) as usize]);
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
            daily_uvw_map[(day) as usize][(u-1) as usize][(v-1) as usize] = STOP_VALUE;
            daily_uvw_map[(day) as usize][(v-1) as usize][(u-1) as usize] = STOP_VALUE;
            short_roots[halfway_point][(u-1) as usize].push(day);
            short_roots[halfway_point][(v-1) as usize].push(day);
            short_roots[(u-1) as usize][halfway_point].push(day);
            short_roots[(v-1) as usize][halfway_point].push(day);

            now_ans[i] = (day) as usize;

        } else if now_ans[i] == 1_000_000_000 {
            if now_ans[i] == 1_000_000_000 {
                conect_roots.push((u, v, i));
            }

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
            daily_uvw_map[(day) as usize][(u-1) as usize][(v-1) as usize] = STOP_VALUE;
            daily_uvw_map[(day) as usize][(v-1) as usize][(u-1) as usize] = STOP_VALUE;
            now_ans[i] = (day) as usize;
        }
    }

    // loop 山登り
    loop {
        // 乱数
        for (u, v, num) in &conect_roots {
            let mut nc_days_cnt = vec![];
            for nc_day in 0..d {
                let mut sum_stop_cnt = 0;
                let mut now_sum_stop_cnt = 0;
                for i in 0..n {
                    now_sum_stop_cnt += daily_uvw_map[(now_ans[(*num)]) as usize][(u-1) as usize][(i) as usize];
                    now_sum_stop_cnt += daily_uvw_map[(now_ans[(*num)]) as usize][(v-1) as usize][(i) as usize];
                }

                for i in 0..n {
                    sum_stop_cnt += daily_uvw_map[(nc_day) as usize][(u-1) as usize][(i) as usize];
                    sum_stop_cnt += daily_uvw_map[(nc_day) as usize][(v-1) as usize][(i) as usize];
                }

                if now_ans[(*num)] == nc_day {
                    sum_stop_cnt -= 2;
                }

                if sum_stop_cnt <= now_sum_stop_cnt {
                    nc_days_cnt.push((nc_day, sum_stop_cnt));
                }
            }

            nc_days_cnt.sort_by(|a, b| a.1.partial_cmp(&(b.1)).unwrap());
            let mut nc_days = vec![nc_days_cnt[0].0];
            for (a, b) in &nc_days_cnt {
                if *b == nc_days_cnt[0].1 {
                    nc_days.push(*a);
                }
            }

            let short_root = short_roots[(u-1) as usize][(v-1) as usize].clone();
            for day in &short_root {
                nc_days.retain(|&x| x != *day);
            }

            if nc_days.len() != 0 {
                let mut index = 1_000_000_000;
                let mut min_v = 1_000_000_000;
                for i in 0..nc_days.len() {
                    let dd = nc_days[i]; 
                    if day_num[dd] < min_v {
                        index = i;
                        min_v = day_num[dd];
                    }
                }
                if day_num[(nc_days[(index) as usize as usize])] < k {
                    day_num[(now_ans[*num]) as usize] -= 1;
                    day_num[(nc_days[(index) as usize]) as usize] += 1;
                    daily_uvw_map[(now_ans[*num]) as usize][(u-1) as usize][(v-1) as usize] = 0;
                    daily_uvw_map[(now_ans[*num]) as usize][(v-1) as usize][(u-1) as usize] = 0;
                    daily_uvw_map[(nc_days[(index) as usize]) as usize][(u-1) as usize][(v-1) as usize] = 1;
                    daily_uvw_map[(nc_days[(index) as usize]) as usize][(v-1) as usize][(u-1) as usize] = 1;
                    now_ans[*num] = (nc_days[(index) as usize]) as usize;
                }
            }
        }

        
        // get end time
        let end = start.elapsed();
        let end_sec = end.as_secs_f64();

        if end_sec > 3.5 {
            break;
        }
    }
    
    // output
    for i in &now_ans {
        print!("{} ", i+1);
    }
}
