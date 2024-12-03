use crate::read_file;

#[derive(Debug)]
enum Tokens {
    Mul,
    OpenParen,
    CloseParen,
    Number(i32),
    Comma,
    Garbage,
    Enable,
    Disable,
}

pub fn day_three() {
    let input = read_file("src/dec_3/input.txt".to_string());
    let input = input.trim();

    let mut tokens = Vec::new();
    let mut characters = input.chars().peekable();

    while let Some(character) = characters.next() {
        if character.is_digit(10) {
            let mut digits = vec![character];

            // Greedily consume all digits
            while let Some(&character) = characters.peek() {
                if character.is_digit(10) {
                    digits.push(character);
                    characters.next();
                } else {
                    break;
                }
            }

            tokens.push(Tokens::Number(
                digits.iter().collect::<String>().parse().unwrap(),
            ));
        } else if character == '(' {
            tokens.push(Tokens::OpenParen);
        } else if character == ')' {
            tokens.push(Tokens::CloseParen);
        } else if character == ',' {
            tokens.push(Tokens::Comma);
        } else if character == 'm' {
            // Check next two
            let u = characters.peek();

            if u == Some(&'u') {
                // We don't do this in the same scope because we can't borrow characters mutably twice
                // We can safely advace at this point, because 'u' is not the first part of any other token
                // This allows us to call peek again to check for 'l'
                characters.next();
                let l = characters.peek();

                if l == Some(&'l') {
                    tokens.push(Tokens::Mul);
                    // Call next again to prevent 'l' from being added to garbage
                    characters.next();
                }
            }
        } else if character == 'd' {
            // Could be 'do' or 'don't'
            let o = characters.peek();

            if o == Some(&'o') {
                characters.next();

                let n = characters.peek();
                if n == Some(&'n') {
                    characters.next();
                    let a = characters.peek();
                    if a == Some(&'\'') {
                        characters.next();
                        let t = characters.peek();
                        if t == Some(&'t') {
                            tokens.push(Tokens::Disable);
                            characters.next();
                        }
                    }
                } else {
                    tokens.push(Tokens::Enable);
                }
            }
        } else {
            tokens.push(Tokens::Garbage);
        }
    }

    let mut part_one_sum = 0;
    let mut part_two_sum = 0;

    let mut enabled = true;

    let mut tokens = tokens.iter();

    while let Some(token) = tokens.next() {
        match token {
            Tokens::Enable => {
                // Must be proceeded by an open and then close paren to be valid
                if let Some(Tokens::OpenParen) = tokens.next() {
                    if let Some(Tokens::CloseParen) = tokens.next() {
                        enabled = true;
                    }
                }
            }
            Tokens::Disable => {
                // Must be proceeded by an open and then close paren to be valid
                if let Some(Tokens::OpenParen) = tokens.next() {
                    if let Some(Tokens::CloseParen) = tokens.next() {
                        enabled = false;
                    }
                }
            }
            Tokens::Mul => {
                if let Some(Tokens::OpenParen) = tokens.next() {
                    if let Some(Tokens::Number(value_one)) = tokens.next() {
                        if let Some(Tokens::Comma) = tokens.next() {
                            if let Some(Tokens::Number(value_two)) = tokens.next() {
                                if let Some(Tokens::CloseParen) = tokens.next() {
                                    part_one_sum += value_one * value_two;

                                    if enabled {
                                        part_two_sum += value_one * value_two;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    println!("Part one: {}", part_one_sum);
    println!("Part two: {}", part_two_sum);
}
