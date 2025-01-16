
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;

fn print_qr_half(qr: &QrCode)
{
    let size = qr.size();
    println!("{:█<1$}", "", (size + 4) as usize);
    for y in 0 .. size / 2 {
        print!("██");
        for x in 0 .. size {
            if qr.get_module(x, y * 2) && qr.get_module(x, y * 2 + 1) {
                print!(" ");
            } else if qr.get_module(x, y * 2) {
                print!("▄");
            } else if qr.get_module(x, y * 2 + 1) {
                print!("▀");
            } else {
                print!("█");
            }
        }
        println!("██");
    }
    if size % 2 != 0 {
        print!("██");
        for x in 0 .. size {
            if qr.get_module(x, size - 1) {
                print!("▄");
            } else {
                print!("█");
            }
        }
        println!("██");
    }
    println!("{:█<1$}", "", (size + 4) as usize);
}

pub fn print_text_as_qr(text: &str)
{
    let qr = QrCode::encode_text(text, QrCodeEcc::Medium).unwrap();
    print_qr_half(&qr);
}