use std::io;

// for use in enum example
enum PrimaryColor {
    red,
    yellow,
    blue,
}

// Create an enum for use with worker
enum WorkerType {
    cpu,
    gpu,
}

// Define a worker struct
struct Worker {
    name: String,
    id_number: i8,
    wtype: WorkerType,
    cores: u8,
    cores_in_use: u8,
    current_jobs: u8,
    jobs_computed: u8,
}

// Define a method for the worker struct
impl Worker {
    fn run_job(&mut self, job_cores: u8) {
        if (job_cores + self.cores_in_use) <= self.cores {
            println!("Starting a new job using {} cores", job_cores);
            self.cores_in_use = self.cores_in_use + job_cores;
            self.current_jobs = self.current_jobs + 1;
        } else {
             println!("Not enough cores available, currently using {} of {} availale cores and {} were requested.", self.cores_in_use, self.cores, job_cores);
        }
    }
}

fn main() {
    // simple function example
    helloworld();

    // passing a parameter and getting a result
    let cube_of_9 = cube(9);
    println!("The cube of 9 is {}", cube_of_9);

    // control flow
    tentotwenty(9);
    tentotwenty(10);
    tentotwenty(21);

    // loops
    testloop();
    testwhileloop();
    testforloop();

    // match statements
    let n = 4;
    match_num(n);
    match_tuple(1,3);

    // simple enum
    use_enum();

    // struct example
    let mut compute_instance = Worker {
        name: String::from("Worker 1"),
        id_number: 1,
        wtype: WorkerType::cpu,
        cores: 2,
        cores_in_use: 0,
        current_jobs: 0,
        jobs_computed: 0,
    };
    
    println!(
        "{} has {} cores",
        compute_instance.name,
        compute_instance.cores,
    );


    compute_instance.run_job(1);
    compute_instance.run_job(2);
}

fn helloworld() {
    println!("Hello, world!");
}

fn cube(num:i32) -> i32 {
    // Returns the number cubed
    num * num * num
}

fn tentotwenty(num:i32) {
    // Prints different statements to demonstrate control flow
    if num < 10 {
        println!("Number {} is less than 10.", num);
    } else if num >= 10 && num <= 20 {
        println!("Number {} is between 10 and 20 inclusive.", num);
    } else {
        println!("Number {} is greater than 20.", num);
    }
}

fn testloop(){
    loop {
        println!("Type 'break' to break out of the loop.");
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");

        if word.trim() == "break" {
            break;
        }
    }
    println!("Congrats, you typed and entered 'break'!");
}

fn testwhileloop() {
    let mut word = String::new();
    while word.trim() != "break" {
        println!("Type 'break' to break out of the while loop.");
        word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
    }
    println!("Congrats, you typed and entered 'break' to exit the while loop!");
}

fn testforloop() {
    // 1..11 is create a range between 1 (inclusive) and 7 (exclusive), so 1,2,3,4,5,6
    for i in 1..7 {
        println!("Now on number {}", i);
    }    
}

fn match_num(num:i32) {
    // Matches on a num
    match num {
        1 => println!("The number is one"),
        2 => println!("The number is two"),
        3 => println!("The number is three"),
        4 => println!("The number is four"),
        _ => println!("The number is not 1-4 but something else"),
    }
}

fn match_tuple(n1:i32, n2:i32) {
    match (n1, n2) {
        (1,1) => println!("Two ones"),
        (3,_) | (_,3) => {
            println!("At least one three!");
            println!("Nice work.");
        },
        _ => println!("Didn't match, try again"),
    }
}

fn use_enum() {
    // code
    let color = PrimaryColor::red;
    color_something(color);
}

fn color_something(color: PrimaryColor) {
    println!("Do something with the color");
    match color {
        PrimaryColor::red => println!("Hex code for red is #FF0000"),
        PrimaryColor::yellow => println!("Hex code for yellow is #FFFF00"),
        PrimaryColor::blue => println!("Hex code for blue is #0000FF"),
    }
}