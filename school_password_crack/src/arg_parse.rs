// My simple (and really bad) parser for the command line arguments

/// Parse the command line arguments as key-value pairs
/// ## Example
/// ```rust
/// // Get Run Args
/// let args: Vec<String> = env::args().collect();
///
/// // Get the value to --hello
/// let world = arg_parse::get_arg_value(&args, "--hello").unwrap();
///
/// // Check that the value is "world"
/// assert_eq!("world", world);
/// ```
pub fn get_arg_value<'a>(raw_args: &'a [String], arg: &'a str) -> Option<&'a str> {
    // Make a new vec of references to the args
    let mut args: Vec<&String> = Vec::new();
    for i in raw_args.iter() {
        args.push(&i);
    }

    // Get position of the arg
    let value = match args.iter().position(|&x| x == arg) {
        // Get the value of the arg
        Some(i) => i,
        None => return None,
    };

    // Check if value is in the args
    if args.len() < value + 1 {
        return None;
    }

    // Return the value if it does not start with - (--)
    let ret = args[value + 1];
    if ret.starts_with('-') {
        return None;
    }

    Some(args[value + 1])
}
