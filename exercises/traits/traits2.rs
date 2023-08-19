// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

// I AM DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String>{
    fn append_bar(mut self) -> Vec<String> {
        
        self.push("Bar".to_string());

        self

        /*
        let mut tmp:Vec<String> = Vec::new();

        for element in self.iter() {
            let mut tmpStr = String::new();
            tmpStr.push_str(&element);
            tmp.push(tmpStr.to_string());
        }
        tmp.push("Bar".to_string());
        tmp
        */

        //i was appending "Bar" to each element of the input vector
        /*
        for element in self.iter() {
            let mut tmpStr = String::new();
            tmpStr.push_str(&element);
            tmpStr.push_str("Bar");
            
            println!("{} --> {}",element,tmpStr);
            tmp.push(tmpStr.to_string());
        }
        tmp
        */
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
