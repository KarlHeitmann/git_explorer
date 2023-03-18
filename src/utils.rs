use git2::Oid;

pub fn short_id(id: Oid) -> String {
    let id = id.to_string();
    unsafe {
        format!("{}", id.get_unchecked(0..7))
    }
}


