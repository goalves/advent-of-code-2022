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
            return Command::NoOp;
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
}

impl Machine {
    pub fn new(commands: Vec<Command>) -> Self {
        Self {
            x_register: 1,
            commands,
        }
    }

    pub fn run(&mut self) {
        let mut what_finishes_on_each_cycle: Vec<Option<&Command>> = vec![];

        for command in &self.commands {
            match command {
                Command::NoOp => what_finishes_on_each_cycle.push(Some(&command)),
                Command::Add(_) => {
                    what_finishes_on_each_cycle.push(None);
                    what_finishes_on_each_cycle.push(Some(&command));
                }
            }
        }

        let mut current_cycle = 0;
        let mut crt_index = 0;
        let mut sprite_index = 0i32;

        while current_cycle < what_finishes_on_each_cycle.len() {
            current_cycle += 1;

            draw_crt(&mut crt_index, &mut sprite_index);

            if let Some(Some(Command::Add(value))) =
                what_finishes_on_each_cycle.get(current_cycle - 1)
            {
                self.x_register += value;
                sprite_index = self.x_register;
            }
        }
    }
}

fn draw_crt(crt_index: &mut i32, sprite_index: &mut i32) {
    if *crt_index == 40 {
        print!("\n");
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
    let input = include_str!("../../day10_input");
    // let input = include_str!("../../test_inputs/day10_test");
    let commands: Vec<Command> = input.lines().map(Command::from).collect();
    let mut machine = Machine::new(commands);

    machine.run();
}
