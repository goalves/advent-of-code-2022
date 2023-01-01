#[derive(Debug)]
pub enum Command {
    NoOp,
    Add(i32),
}

impl Command {
    pub fn cycles_to_finish(&self) -> usize {
        match self {
            Command::NoOp => 1,
            Command::Add(_) => 2,
        }
    }
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        if value == "noop" {
            Command::NoOp
        } else {
            let input: Vec<&str> = value.split(' ').collect();
            Command::Add(input[1].parse::<i32>().unwrap())
        }
    }
}

#[derive(Debug)]
pub struct Machine {
    x_register: i32,
    commands: Vec<Command>,
    signal_strenghts: Vec<i32>,
}

impl Machine {
    pub fn new(commands: Vec<Command>) -> Self {
        Self {
            x_register: 1,
            commands,
            signal_strenghts: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let mut what_finishes_on_each_cycle: Vec<Option<&Command>> = vec![];

        for command in &self.commands {
            match command {
                Command::NoOp => what_finishes_on_each_cycle.push(Some(command)),
                Command::Add(_) => {
                    what_finishes_on_each_cycle.push(None);
                    what_finishes_on_each_cycle.push(Some(command));
                }
            }
        }

        let mut current_cycle = 0;
        let mut crt_index = 0;
        let mut sprite_index = 0i32;

        while current_cycle < what_finishes_on_each_cycle.len() {
            current_cycle += 1;

            if (current_cycle + 20) % 40 == 0 {
                self.signal_strenghts
                    .push((current_cycle as i32) * self.x_register);
            }

            draw_crt(&mut crt_index, &mut sprite_index);

            if let Some(Some(Command::Add(value))) =
                what_finishes_on_each_cycle.get(current_cycle - 1)
            {
                self.x_register += value;
                sprite_index = self.x_register;
            }
        }

        println!();
    }
}

fn draw_crt(crt_index: &mut i32, sprite_index: &mut i32) {
    if *crt_index == 40 {
        println!();
        *crt_index = 0;
    }

    if (*crt_index - *sprite_index).abs() < 2 {
        print!("#")
    } else {
        print!(".")
    }

    *crt_index += 1;
}

fn main() {
    let input = include_str!("../../inputs/day10");
    // let input = include_str!("../../inputs/test/day10");
    let commands: Vec<Command> = input.lines().map(Command::from).collect();
    let mut machine = Machine::new(commands);

    machine.run();

    println!("Signal strenghts: {:?}", machine.signal_strenghts);
}
