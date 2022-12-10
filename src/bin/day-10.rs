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

    pub fn run(&mut self) -> Vec<i32> {
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

        println!("what finishes: {:?}", what_finishes_on_each_cycle);

        let mut signal_strenghts = vec![];
        let mut current_cycle = 0;
        while current_cycle < what_finishes_on_each_cycle.len() {
            println!("running current cycle: {}", current_cycle);
            current_cycle += 1;
            if current_cycle == 20 || (current_cycle >= 20 && (current_cycle - 20) % 40 == 0) {
                println!(
                    "pushing: {}, {}, total: {} \n",
                    current_cycle,
                    self.x_register,
                    current_cycle as i32 * self.x_register
                );
                signal_strenghts.push((current_cycle as i32) * self.x_register);
            }

            if let Some(Some(Command::Add(value))) =
                what_finishes_on_each_cycle.get(current_cycle - 1)
            {
                println!("summing up {:?}, self reg: {}", value, self.x_register);
                self.x_register += value;
            }
        }

        signal_strenghts
    }
}

fn main() {
    let input = include_str!("../../day10_input");
    // let input = include_str!("../../test_inputs/day10_test");
    let commands: Vec<Command> = input.lines().map(Command::from).collect();
    let mut machine = Machine::new(commands);

    let signal_strenghts = machine.run();
    println!("{:?}", signal_strenghts.iter().sum::<i32>());
    println!("{:?}", machine);
}
