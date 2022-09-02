fn main() {
    let mut result = [
        'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A',
    ];
    do_all_work(0, &mut result);
    // let result = generate_new_result("00000Y000C00000Y", "A");

    // print!("{}", result);
}

fn do_all_work(result_char_count: usize, result: &mut [char; 16]) -> Result<(), ()> {
    let list_of_characters = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    ];

    for char_of_all_character in list_of_characters.iter() {
        if result_char_count != 15 {
            result[result_char_count] = char_of_all_character.clone();
            do_all_work(result_char_count + 1, result)?;
        } else {
            result[result_char_count] = char_of_all_character.clone();
            let result_to_show: String = result.iter().collect();

            println!("{:?}", result_to_show);
        }
    }
    return Ok(());
}

// fn generate_new_result<'a>(word: &'a mut [char], list_of_characters: &[char]) -> &'a [char] {
//     // let mut position_of_all_character = 0;

//     // while true {
//     //     let char_to_use = list_of_characters[position_of_all_character];

//     //     if position_of_all_character == 36 {
//     //         position_of_all_character = 0;
//     //     }
//     //     position_of_all_character += 1;
//     // }

//     for char in word.iter() {
//         fn_algo(char)
//     }

//     let length = word.len();

//     let word_to_use = &word[0..(length - 1)];

//     let ll = word_to_use.len();

//     return word;
// }

// A A A A

//RESULT = """
// A 0 , RESULT = "0000"
// A 1 , RESULT = "0000"
// 0 2 , RESULT = "0000"
// 0 3 , RESULT = "0000"
// 0 3 , RESULT = "0001"
// 0 3 , RESULT = "0002"

//A 0 ""
//RESULT = "AAA"
//do_all_work(0, "A")
//  do_all_work(1, "A")
//      do_all_work(2, "AAD")
//          do_all_work(3, "AAA")
//          print("AAA")
//          print("AAB")
//          print("AAC")
//          print("AAD")
//      do_all_work(2, "ABD")
//      print("ABA")
//      print("ABB")
//      print("ABC")
