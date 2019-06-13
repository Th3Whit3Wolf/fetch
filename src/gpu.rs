#[cfg(not(feature = "vulkan"))]
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

#[cfg(not(feature = "vulkan"))]
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

#[cfg(feature = "vulkan")]
pub fn get_gpu(brand: bool) -> Vec<String> {
    let mut v: Vec<String> = Vec::with_capacity(2);
    use vulkano::instance::Instance;
    use vulkano::instance::InstanceExtensions;
    use vulkano::instance::PhysicalDevice;
    use vulkano::instance::PhysicalDeviceType;
    let instance = Instance::new(None, &InstanceExtensions::none(), None).unwrap();

    for physical_device in PhysicalDevice::enumerate(&instance) {
        v.push(gpu_parse(physical_device, brand));
    }

    fn gpu_parse(gpu: PhysicalDevice, brand: bool) -> String {
        let res: Vec<String> = gpu.name().split(' ').map(|s| s.to_string()).collect();

        let gpu_type = gpu.ty();
        if gpu_type == PhysicalDeviceType::IntegratedGpu {
            if &res[0] == "Intel(R)" {
                let gpu = res.join(" ");
                let gpu = str::replace(&gpu, "(R)", "").to_string();
                let gpu: String = gpu.split('(').take(1).next().unwrap_or("").to_string();
                if !brand {
                    str::replace(&gpu, "Intel ", "")
                } else {
                    gpu
                }
            } else {
                let gpu = res.join(" ");
                if !brand {
                    str::replace(&gpu, "AMD", "").to_string()
                } else {
                    gpu
                }
            }
        } else if gpu_type == PhysicalDeviceType::DiscreteGpu {
            if &res[0] == "GeForce" {
                if !brand {
                    res.join(" ")
                } else {
                    "Nvidia ".to_string() + &res.join(" ")
                }
            } else if !brand {
                res.join(" ")
            } else {
                "AMD ".to_string() + &res.join(" ")
            }
        } else {
            gpu.name()
        }
    }
    v
}



