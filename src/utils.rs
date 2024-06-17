pub fn minify_bufname(bufname: &String, buflist: &Vec<String>) -> String {
    let mut smallest_unique_bufname = "".to_string();

    for part in bufname.split('/').collect::<Vec<&str>>().iter().rev() {
        let mut matches = false;
        smallest_unique_bufname = format!("{}/{}", *part, smallest_unique_bufname);
        smallest_unique_bufname = smallest_unique_bufname
            .strip_suffix('/')
            .unwrap_or(&smallest_unique_bufname)
            .to_string();

        for other_bufname in buflist {
            if *other_bufname != *bufname {
                matches = format!("/{}", other_bufname).ends_with(&format!("/{}", smallest_unique_bufname));
            }
        }
        if !matches {
            break;
        }
    }

    smallest_unique_bufname
}
