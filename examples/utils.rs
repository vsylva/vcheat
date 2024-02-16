fn main() {
    clipboard();
}

fn clipboard() {
    vcheat::system::set_clipboard_unicode_text("test").unwrap();
    let text = vcheat::system::get_clipboard_unicode_text().unwrap();
    println!("{text}");
}
