use crate::*;

#[inline(always)]
pub(crate) fn aob_scan_multi_threaded(
    pattern: &str,
    data: &[u8],
    return_on_first: bool,
    thread_count: usize,
) -> Result<Vec<usize>> {
    if pattern.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Pattern cannot be empty",
        )));
    }

    let mut signature_bytes: Vec<u8> = vec![];
    let mut mask_bytes: Vec<bool> = vec![];

    for pair in pattern.split_whitespace() {
        if pair == "?" || pair == "??" {
            mask_bytes.push(false);
            signature_bytes.push(0);
        } else {
            mask_bytes.push(true);
            signature_bytes.push(u8::from_str_radix(pair, 16)?);
        }
    }

    let mut start_offset = mask_bytes.iter().take_while(|&&x| x == false).count();
    let end_offset = mask_bytes.iter().rev().take_while(|&&x| x == false).count();

    if start_offset != mask_bytes.len() {
        signature_bytes =
            signature_bytes[start_offset..signature_bytes.len() - end_offset].to_vec();
        mask_bytes = mask_bytes[start_offset..mask_bytes.len() - end_offset].to_vec();
    } else {
        start_offset = 0;
    }

    let signature = &signature_bytes;
    let mask = &mask_bytes;
    let data = &data;

    let finished = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let address_array = std::sync::Arc::new(std::sync::Mutex::new(Vec::<usize>::new()));

    if thread_count > 1 {
        let running_thread_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let found = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        std::thread::scope(|scope| {
            for tc in 0..thread_count {
                let range = {
                    let data_size = data.len();
                    let chunks = thread_count;
                    let overlap = signature.len() - 1;
                    let index = tc;
                    let chunk_size = data_size / chunks;
                    let remainder = data_size % chunks;

                    let start = index * chunk_size;

                    let mut end =
                        start + chunk_size + if index == chunks - 1 { remainder } else { 0 };

                    let start = start - if start >= overlap { overlap } else { 0 };

                    end = end
                        + if end < data_size - overlap {
                            overlap
                        } else {
                            0
                        };

                    (start, end)
                };

                let running_thread_count = running_thread_count.clone();
                let finished = finished.clone();
                let found = found.clone();

                running_thread_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let addres_array_clone = address_array.clone();
                scope.spawn(move || {
                    let data = &data[range.0..range.1];

                    if {
                        let length = data.len() - signature.len();
                        let finished = &finished;
                        let first_byte = signature[0];
                        let first_mask = mask[0];

                        let mut found = false;

                        for i in 0..length {
                            if finished.load(std::sync::atomic::Ordering::Relaxed) {
                                break;
                            }

                            if data[i] != first_byte && first_mask {
                                continue;
                            }

                            if {
                                let temp_data = &data[i..];
                                let mut temp_bool = true;
                                for (i, sig) in signature.iter().enumerate() {
                                    if !mask[i] {
                                        continue;
                                    }

                                    if temp_data[i] != *sig {
                                        temp_bool = false;
                                        break;
                                    }
                                }
                                temp_bool
                            } {
                                found = true;
                                std::ops::DerefMut::deref_mut(
                                    &mut addres_array_clone.lock().unwrap(),
                                )
                                .push(range.0 + i - start_offset);
                                if return_on_first {
                                    finished.store(true, std::sync::atomic::Ordering::Relaxed);
                                    break;
                                }
                            }
                        }

                        found
                    } {
                        found.store(true, std::sync::atomic::Ordering::SeqCst);
                    }

                    running_thread_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                });
            }
        });
        while running_thread_count.load(std::sync::atomic::Ordering::SeqCst) != 0 {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        found.load(std::sync::atomic::Ordering::SeqCst);
    } else {
        let length = data.len() - signature.len();
        let finished = &finished;
        let first_byte = signature[0];
        let first_mask = mask[0];

        for i in 0..length {
            if finished.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            if data[i] != first_byte && first_mask {
                continue;
            }

            if {
                let temp_data = &data[i..];
                let mut temp_bool = true;
                for (i, sig) in signature.iter().enumerate() {
                    if !mask[i] {
                        continue;
                    }

                    if temp_data[i] != *sig {
                        temp_bool = false;
                        break;
                    }
                }
                temp_bool
            } {
                address_array.lock().unwrap().push(i - start_offset);
                if return_on_first {
                    finished.store(true, std::sync::atomic::Ordering::Relaxed);
                    break;
                }
            }
        }
    }

    let mut address_array = address_array.lock().unwrap().to_vec();
    address_array.sort();
    Ok(address_array)
}
