use qrcode::render::unicode;
use qrcode::QrCode;

pub fn generate_qr_code(text: &str) {
    let code = QrCode::new(text).unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Dark)
        .light_color(unicode::Dense1x2::Light)
        .build();
    println!("{}", image);
}
