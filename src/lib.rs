use std::error::Error;

pub fn run_application(args: &[String]) -> Result<(), Box<dyn Error>> {
    let _ = args;
    todo!("implement application logic")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "implement application logic")]
    fn application_panics() {
        run_application(&vec![]).unwrap();
    }
}
