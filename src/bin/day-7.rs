fn main() {
    let input = include_str!("../../day7_input");

    // let input = include_str!("../../test_inputs/day7_test");

    let mut folder_sizes = vec![("/", 0)];
    let mut final_folder_sizes = vec![];

    for line in input.lines() {
        println!("folder_sizes_calc: {:?}", folder_sizes);
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line == "$ cd .." {
            let len = folder_sizes.len();
            let val = folder_sizes.remove(len - 1);
            folder_sizes[len - 2].1 += val.1;

            final_folder_sizes.push(val);

            continue;
        }

        let split: Vec<&str> = line.split(" ").collect();

        if split[0] == "dir" {
            continue;
        }

        if let Ok(current_file_value) = split[0].parse::<u32>() {
            let len = folder_sizes.len();

            let (_, folder_value) = folder_sizes.get_mut(len - 1).unwrap();
            *folder_value += current_file_value;
            continue;
        } else {
            folder_sizes.push((split[2], 0));
        }
    }

    println!("folder sizes: {:?}", folder_sizes);

    while folder_sizes.len() > 0 {
        let len = folder_sizes.len();
        let folder = folder_sizes.remove(len - 1);

        if len > 1 {
            let (_, to_update) = folder_sizes.get_mut(len - 2).unwrap();
            *to_update += folder.1;
        }

        final_folder_sizes.push(folder);
    }

    let sum_total_size: u32 = final_folder_sizes
        .iter()
        .map(|x| {
            println!("{:?}", x);
            x
        })
        .filter(|(_, size)| size < &100000)
        .map(|(_, size)| size)
        .sum();

    println!("{:?}", sum_total_size);

    let total_used_size = final_folder_sizes[final_folder_sizes.len() - 1].1.clone();
    let total_size = 70000000;
    let needed_unused = 30000000 - (total_size - total_used_size);

    let folder_to_delete_size = final_folder_sizes
        .into_iter()
        .filter(|(_, val)| val >= &needed_unused)
        .map(|(_, val)| val)
        .min()
        .unwrap();

    println!("{:?}", folder_to_delete_size)
}
