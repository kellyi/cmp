// struct Context<'s>(&'s str);

// struct Parser<'c, 's: 'c> {
//     context: &'c Context<'s>,
// }

// impl<'c, 's> Parser<'c, 's> {
//     fn parse(&self) -> Result<(), &str> {
//         Err(&self.context.0[1..])
//     }
// }

// fn parse_context(context: Context) -> Result<(), &str> {
//     Parser {
//         context: &context,
//     }.parse()
// }

// fn main() {
//     println!("Hello, world!");
// }

trait Red {}

struct Ball<'a> {
    _diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

fn main() {
    let num = 5;

    let _obj = Box::new(
        Ball {
            _diameter: &num,
        }
    ) as Box<Red>;
}
