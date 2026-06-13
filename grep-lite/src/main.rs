fn main() {
    let search_term = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-house, and dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";

/** Standard Approach
    let mut line_num: usize = 1; 
    for line in quote.lines() {
        // .lines() demonstrates iterating line-by-line in a platform independent manner
        if line.contains(search_term) {
            // .contains demonstrates searching for text using the method syntax=
            println!("{}: {}", line_num, line); 
        }
        line_num += 1;
    }
*/
    
    // More ergonomic approach
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num: usize = i + 1;
            println!("{}: {}", line_num, line); 
        }
    }
}


