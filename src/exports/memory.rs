use crate::*;

// #[inline(always)]
// pub fn aob_scan_single_threaded1(
//     pattern: &str,
//     data: &[u8],
//     return_on_first: bool,
// ) -> Result<Vec<usize>> {
//     let mut signature = Vec::<Option<u8>>::new();

//     for c in pattern.split_whitespace() {
//         if c == "?" || c == "??" {
//             signature.push(None);
//         } else {
//             signature.push(Some(u8::from_str_radix(c, 16)?))
//         }
//     }

//     let mut address_array: Vec<usize> = Vec::new();

//     let signature_len = signature.len();
//     for i in 0..data.len() {
//         if data[i..].len() >= signature_len {
//             let mut match_found = true;

//             for j in 0..signature_len {
//                 if signature[j] != None && Some(data[i + j]) != signature[j] {
//                     match_found = false;
//                     break;
//                 }
//             }

//             if match_found {
//                 address_array.push(i);
//                 if return_on_first {
//                     return Ok(address_array);
//                 }
//             }
//         }
//     }
//     address_array.sort();
//     Ok(address_array)
// }

#[inline(always)]
pub fn aob_scan_single_threaded(
    pattern: &str,
    data: &[u8],
    return_on_first: bool,
) -> Result<Vec<usize>> {
    let mut signature: Vec<u8> = vec![];
    let mut mask: Vec<bool> = vec![];

    for pair in pattern.split_whitespace() {
        if pair == "?" || pair == "??" {
            mask.push(false);
            signature.push(0);
        } else {
            mask.push(true);
            signature.push(u8::from_str_radix(pair, 16)?);
        }
    }

    let mut start_offset = mask.iter().take_while(|&&x| x == false).count();
    let end_offset = mask.iter().rev().take_while(|&&x| x == false).count();

    if start_offset != mask.len() {
        signature = signature[start_offset..signature.len() - end_offset].to_vec();
        mask = mask[start_offset..mask.len() - end_offset].to_vec();
    } else {
        start_offset = 0;
    }

    let first_byte = signature[0];
    let first_mask = mask[0];

    let mut address_array: Vec<usize> = Vec::new();

    for i in 0..data.len() - signature.len() {
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
            address_array.push(i - start_offset);
            if return_on_first {
                break;
            }
        }
    }
    address_array.sort();
    Ok(address_array)
}

#[inline(always)]
pub fn aob_scan_multi_threaded(
    pattern: &str,
    data: &[u8],
    return_on_first: bool,
    thread_count: u32,
) -> Result<Vec<usize>> {
    crate::memory::aob_scan_multi_threaded(pattern, data, return_on_first, thread_count as usize)
}
