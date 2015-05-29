use fpa_aws;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let mut awg = fpga_awg::new();

        while (true) {
            toggle(awg.data, 1);
            thread::sleep_ms(arg1.parse::<u32>().unwrap());
        }
    }
}