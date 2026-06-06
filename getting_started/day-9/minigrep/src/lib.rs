pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut vec = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            vec.push(line);
        }
    }

    vec
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
     
    }
}
