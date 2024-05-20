const TEXT: &str = "160111433b00035f536110435a380402561240555c526e1c0e431300091e4f04451d1d490d1c49010d000a0a4510111100000d434202081f0755034f13031600030d0204040e
050602061d07035f4e3553501400004c1e4f1f01451359540c5804110c1c47560a1415491b06454f0e45040816431b144f0f4900450d1501094c1b16550f0b4e151e03031b450b4e020c1a124f020a0a4d09071f16003a0e5011114501494e16551049021011114c291236520108541801174b03411e1d124554284e141a0a1804045241190d543c00075453020a044e134f540a174f1d080444084e01491a090b0a1b4103570740
000000000000001a49320017071704185941034504524b1b1d40500a0352441f021b0708034e4d0008451c40450101064f071d1000100201015003061b0b444c00020b1a16470a4e051a4e114f1f410e08040554154f064f410c1c00180c0010000b0f5216060605165515520e09560e00064514411304094c1d0c411507001a1b45064f570b11480d001d4c134f060047541b185c
0b07540c1d0d0b4800354f501d131309594150010011481a1b5f11090c0845124516121d0e0c411c030c45150a16541c0a0b0d43540c411b0956124f0609075513051816590026004c061c014502410d024506150545541c450110521a111758001d0607450d11091d00121d4f0541190b45491e02171a0d49020a534f
031a5410000a075f5438001210110a011c5350080a0048540e431445081d521345111c041f0245174a0006040002001b01094914490f0d53014e570214021d00160d151c57420a0d03040b4550020e1e1f001d071a56110359420041000c0b06000507164506151f104514521b02000b0145411e05521c1852100a52411a0054180a1e49140c54071d5511560201491b0944111a011b14090c0e41
0b4916060808001a542e0002101309050345500b00050d04005e030c071b4c1f111b161a4f01500a08490b0b451604520d0b1d1445060f531c48124f1305014c051f4c001100262d38490f0b4450061800004e001b451b1d594e45411d014e004801491b0b0602050d41041e0a4d53000d0c411c41111c184e130a0015014f03000c1148571d1c011c55034f12030d4e0b45150c5c
011b0d131b060d4f5233451e161b001f59411c090a0548104f431f0b48115505111d17000e02000a1e430d0d0b04115e4f190017480c14074855040a071f4448001a050110001b014c1a07024e5014094d0a1c541052110e54074541100601014e101a5c
0c06004316061b48002a4509065e45221654501c0a075f540c42190b165c
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

fn text_to_binary(string: &str) -> String {
    string
        .chars()
        .map(|c| format!("{:b}", c as u8))
        .collect::<Vec<String>>()
        .join("")
}

fn hex_to_bin(hex_string: &str) -> String {
    hex_string
        .chars()
        .map(|c| {
            let decimal = c.to_digit(16).unwrap();
            format!("{:04b}", decimal)
        })
        .collect::<String>()
}

fn binary_to_ascii(binary: String) -> String {
    binary
        .as_bytes()
        .chunks(8)
        .map(|chunk| {
            let byte = std::str::from_utf8(chunk).unwrap();
            u8::from_str_radix(byte, 2).unwrap() as char
        })
        .collect::<String>()
}

fn binary_to_text(binary: &str) -> String {
    let text = binary
        .as_bytes()
        .chunks(8)
        .map(|chunk| {
            let byte_str = std::str::from_utf8(chunk).unwrap();
            let byte = u8::from_str_radix(byte_str, 2).unwrap();
            byte as char
        })
        .collect::<String>();

    text
}

fn ascii_to_binary(text: &str) -> String {
    text.chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect::<Vec<String>>()
        .join("")
}

fn xor(b1: String, b2: String) -> String {
    let length = std::cmp::min(b1.len(), b2.len());
    let lenghth_of_0 = std::cmp::max(b1.len(), b2.len()) - length;
    let mut result = String::new();

    for i in 0..length {
        let c1 = b1.chars().nth(i).unwrap();
        let c2 = b2.chars().nth(i).unwrap();

        if c1 == c2 {
            result.push('0');
        } else {
            result.push('1');
        }
    }

    for i in 0..lenghth_of_0 {
        result.push('0');
    }

    result
}

fn xor_binary_string(str1: String, str2: String) -> String {
    let mut res = String::new();
    let len1 = str1.len();
    let len2 = str2.len();
    let max_len = std::cmp::max(len1, len2);

    let padded_str1 = if len1 < max_len {
        // println!("before padding: {}", str1);
        format!("{:0<width$}", str1, width = max_len)
    } else {
        str1.to_string()
    };

    // println!("after padding : {}", padded_str1);

    let padded_str2 = if len2 < max_len {
        format!("{:0<width$}", str2, width = max_len)
    } else {
        str2.to_string()
    };

    for i in 0..max_len {

        let c1 = padded_str1.chars().nth(i).unwrap();
        let c2 = padded_str2.chars().nth(i).unwrap();

        if c1 == c2 {
            res.push('0');
        } else {
            res.push('1');
        }
    }

    res
}

fn main() {
    let text = "The ";
    let binary_text = text_to_binary(text);
    println!("Binary representation of '{}': {}", text, binary_text);

    let lines = TEXT.split('\n');

    // Iterate over each line and print it
    for (i, line) in lines.clone().enumerate() {
        println!("Binary representation of line {}: {}", i, hex_to_bin(line))
    }

    let binary_str = "011110010110111101110101"; // Binary representation of "you"
    println!("ASCII text: {}", binary_to_ascii(binary_str.to_string()));

    let ascii = "you"; // Binary representation of "you"
    println!("ASCII text: {} , bin {}", ascii, ascii_to_binary(ascii));
    // println!("ASCII text: {} , bin {}", ascii, text_to_binary(ascii));

    println!("{}", xor(binary_str.to_string(), ascii_to_binary(ascii)));

    //We test line 1 XOR "The "
    let key = xor_binary_string(
        hex_to_bin(TEXT.lines().nth(0).unwrap()),
        text_to_binary("The "),
    );
    for (i, line) in lines.clone().enumerate() {
        println!(
            "line {} first charracters: {}",
            i,
            binary_to_ascii(xor_binary_string(key.clone(), hex_to_bin(line)))
        );
    }
}
