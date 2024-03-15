

pub const MAX_SIZE: usize = 30;
pub const DECIMAL_RADIX: u32 = 10;


pub fn make_response(words: Vec<String>) -> Vec<u8> {


    if words.first().expect("No first word in words") == "echo" {
        make_bulk_response(words.get(1).expect("No second word in words"))
    } else if words.first().expect("No first word in words") == "ping" {
        make_ping_response()
    } else {
        unimplemented!();
    }
}


fn make_bulk_response(word: &String) -> Vec<u8> {
    let mut response = Vec::new();
    response.push(b'$');
    response.push(char::from_digit(word.len() as u32, DECIMAL_RADIX).expect("Couldn't parse word length") as u8);
    response.push(b'\r');
    response.push(b'\n');
    response.extend(word.as_bytes());
    response.push(b'\r');
    response.push(b'\n');
    response
}
fn make_ping_response() -> Vec<u8> {
    let mut response = Vec::new();
    response.push(b'+');
    response.extend("PONG".as_bytes());
    response.push(b'\r');
    response.push(b'\n');
    response
}

pub fn parse_redis_command(&received_buffer: &[u8; MAX_SIZE]) -> Vec<String> {
    let first_byte: &u8 = received_buffer.get(0).expect("Couldn't get first byte");

    if *first_byte == b'*' {
        let words = parse_array(&received_buffer);
        words
    } else {
        unimplemented!();
    }
}

fn parse_array(&buffer: &[u8; MAX_SIZE]) -> Vec<String> {
    println!(
        "{:?}",
        char::from(*buffer.get(1).expect("Couldn't get array size"))
            .to_digit(DECIMAL_RADIX)
            .expect("Couldn't parse number of words") as usize
    );
    let number_of_words: usize = char::from(*buffer.get(1).expect("Couldn't get array size"))
        .to_digit(DECIMAL_RADIX)
        .expect("Couldn't parse number of words") as usize;
    let mut words = Vec::new();

    let mut length_word: usize;
    let mut i = 2;
    while (number_of_words as usize != words.len()) {
        while (i != buffer.len()) {
            let dollar = buffer.get(i).expect("Couldn't parse u8");
            if *dollar == b'$'  {
                let mut j = i + 1;

                while (j != buffer.len()) {
                    let digit_end = buffer.get(j).expect("Couldn't parse u8");
                    if *digit_end == b'\r' {
                        length_word = std::str::from_utf8(
                            &buffer.get(i + 1..j).expect("Couldn't get word length"),
                        )
                            .expect("Invalid UTF-8")
                            .parse::<usize>()
                            .expect("Failed to parse integer");
                        let word = std::str::from_utf8(
                            &buffer
                                .get(j + 2..j + 2 + length_word)
                                .expect("Couldn't get word"),
                        )
                        .expect("Couldn't parse word").to_string();
                        words.push(word);
                        i = j + 2 + length_word;
                        break;
                    }
                    j += 1;
                }
            }
            i += 1;
        }
    }
    words
}
