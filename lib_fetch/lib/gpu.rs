pub fn get_gpu(brand: bool) -> Vec<String> {
    let v: Vec<String> = Vec::with_capacity(2);
    use shells::*;
    let (_code, stdout, _stderr) = sh!("lspci | grep VGA | awk -F ':' '{{print $3}}'");
    let gpu = stdout[..].trim();
    let v = gpu
        .to_string()
        .lines()
        .map(|s| gpu_parse(s.to_string(), brand))
        .collect();
    v
}

pub fn gpu_parse(gpu: String, brand: bool) -> String {
    let v: Vec<&str> = Vec::with_capacity(8);
    let v: Vec<&str> = gpu.split_whitespace().collect();
    if v[0] == "Intel" {
        let gpu = v.join(" ");
        let gpu = str::replace(&gpu, "(Mobile)", "");
        let gpu = str::replace(&gpu, "Corporation ", "");
        if !brand {
            str::replace(&gpu, "Intel", "").trim().to_string()
        } else {
            gpu.to_string()
        }
    } else if v[0] == "NVIDIA" {
        let gpu: String = v.join(" ");
        let gpu: String = gpu.split('[').take(2).nth(1).unwrap_or("").to_string();
        let gpu: String = gpu.split(']').take(1).next().unwrap_or("").to_string();

        if !brand {
            gpu
        } else {
            "Nvidia ".to_string() + &gpu
        }
    } else {
        "Unknown".to_string()
    }
}
