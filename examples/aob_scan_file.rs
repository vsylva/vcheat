fn main() {
    scan_single_threaded();
    scan_multi_threaded();
}

fn scan_multi_threaded() {
    let data = std::fs::read(r"C:\Program Files\Microsoft VS Code\Code.exe").unwrap();

    let pattern = "5C ? 6D ??";

    let return_on_first = false;

    let now = std::time::Instant::now();
    let multi_array = vcheat::aob_scan_multi_threaded(
        pattern,
        &data,
        return_on_first,
        vcheat::get_logical_cpu_count(),
    )
    .unwrap();

    println!(
        "Elapsed time of a multi thread: {} millis",
        now.elapsed().as_millis()
    );
    println!("Length of the found address: {}", multi_array.len());
}

fn scan_single_threaded() {
    let data = std::fs::read(r"C:\Program Files\Microsoft VS Code\Code.exe").unwrap();

    let pattern = "5C ? 6D ??";

    let return_on_first = false;

    let now = std::time::Instant::now();
    let single_array = vcheat::aob_scan_single_threaded(pattern, &data, return_on_first).unwrap();

    println!(
        "Elapsed time of a single thread: {} millis",
        now.elapsed().as_millis()
    );
    println!("Length of the found address: {}", single_array.len());
}
