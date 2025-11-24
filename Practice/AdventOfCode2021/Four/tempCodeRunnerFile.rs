// Getting the data
    let raw_data: String = read_contents();
    let mut data: Vec<&str> = get_words(&raw_data);