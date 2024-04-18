pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    let mut output = duration;
    if time_series.len() < 2 {
        return output;
    }
    for i in 0..=time_series.len() - 2 {
        let next = i + 1;
        let cur_poison_duration = time_series[i] + duration - 1;
        if cur_poison_duration >= time_series[next] {
            output += time_series[next] - time_series[i];
        } else {
            output += duration
        }
    }
    output
}
