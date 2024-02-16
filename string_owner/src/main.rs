fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word_1(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let second_bytes = &s[i+1..].as_bytes();

            for (j, &item_2) in second_bytes.iter().enumerate() {
                if item_2 == b' ' {
                    return &s[i+1..j+i+1]
                } else {
                    return ""
                }
            }
        }
    }
    ""
}


fn second_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut space_indices = bytes.iter().enumerate().filter(|&(_, &b)| b == b' ').map(|(i, _)| i);

    if let Some(first_space_index) = space_indices.next() {
        if let Some(second_space_index) = space_indices.next() {
            &s[first_space_index + 1..second_space_index]
        } else {
            // 첫 번째 공백 이후로 두 번째 공백이 없다면, 첫 번째 공백부터 문자열 끝까지가 아닌,
            // 첫 번째 공백 바로 다음부터 문자열 끝까지를 반환
            if s.len() > first_space_index + 1 {
                &s[first_space_index + 1..]
            } else {
                // 두 번째 단어가 실제로 없다면 빈 문자열 반환
                ""
            }
        }
    } else {
        // 공백이 전혀 없다면 빈 문자열 반환
        ""
    }
}


fn main() {
    // let s1 = String::from("hello this is me");
    // let s2 = String::from("suhyun");

    // let s1_result = first_word(&s1);
    // let s2_result = first_word(&s2);

    // let s1_second_result = second_word(&s1);
    // let s2_second_result = second_word(&s2);
    
    // println!("{s1_result}, {s2_result}");
    // println!("{s1_second_result}, {s2_second_result}");

    let s = String::from("hello  world");
    
    let word1 = first_word(&s);
    let word2 = second_word_1(&s);
    let word3 = second_word_2(&s);
    
    println!("{word1}, {word2}, {word3}")

}