use std::io;

struct Game {
    score: i32,
    attempts: i32,
    random_num: i32,
}

impl Game {
    fn generate_random_number(&mut self) {
        println!("A new random number has been generated");
        let random_num: i32 = rand::random_range(0..=10);
        self.random_num = random_num;
    }

    fn inc_score(&mut self) {
        self.score += 1;
    }

    fn dec_score(&mut self) {
        self.score -= 1;
    }

    fn inc_attempts(&mut self) {
        self.attempts += 1;
    }

    fn reset_attempts(&mut self) {
        self.attempts = 0;
    }

    fn check_attempts(&mut self) {
        if self.attempts >= 3 {
            println!("You have made 3 attempts, the secert number was {}. Lets try again with a new number. ",self.random_num);
            self.dec_score();
            self.reset_attempts();
            println!("Your score is reduced by 1");
            self.generate_random_number();
        } else {
            self.inc_attempts();
        }
    }

    fn compare(&mut self, guess: i32) {
        if guess == self.random_num {
            println!("Good job the number was indeed {}", guess);
            self.inc_score();
            self.generate_random_number();
            self.reset_attempts();
        } else if guess > self.random_num {
            println!("-----------------------");
            println!("Guess a bit lower");
            self.check_attempts();
        } else if guess < self.random_num {
            println!("-----------------------");
            println!("Guess a bit higer");
            self.check_attempts();
        }
    }

    fn new() -> Game {
        Game {
            score: 0,
            attempts: 0,
            random_num: 0,
        }
    }
}

fn main() {
    let mut game = Game::new();

    loop {
        println!("Welcome to guessing game");
        println!("Press (p) to start and (q) to exit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "q" {
            break;
        } else {
            game.generate_random_number();
            loop {
                println!("Enter your guess.");
                println!("Current score: {}", game.score);
                println!("Curremt attempts:{}", game.attempts);
                let mut guess = String::new();
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Error in reading key");

                if guess.trim() == "q" {
                    break;
                }

                let guess = match guess.trim().parse() {
                    Ok(n) => n,
                    Err(_) => continue,
                };
                game.compare(guess);
            }
        }
    }
}
