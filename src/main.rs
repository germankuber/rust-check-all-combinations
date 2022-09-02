fn main() {
    let mut result = [
        'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A',
    ];
    do_all_work(0, &mut result);
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
