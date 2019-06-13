pub fn get_kernel(shorthand: bool) -> String {
    let kernel = os_release().unwrap();
    kernel.to_string();
    if shorthand {
        "Kernel: ".to_string() + &kernel
    } else {
        let os_type = os_type().unwrap();
        "Kernel: ".to_string() + &os_type.to_string() + " " + &kernel
    }
}