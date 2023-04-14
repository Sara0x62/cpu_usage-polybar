use sysinfo::{CpuExt, System, SystemExt};

#[derive(Default)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    // Convert rgb values to hex for polybar to read
    fn hex_values(&self) -> String {
        format!(
            "{:02X}{:02X}{:02X}",
            self.r as i32, self.g as i32, self.b as i32
        )
    }

    // New squared colors for the start and end colors
    fn new_squared(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: r * r,
            g: g * g,
            b: b * b,
        }
    }

    // Generated the gradient color, based on percentage
    fn new_from_gradient(start: &Color, end: &Color, percentage: &f32) -> Color {
        let percent = percentage / 100.0;

        Color {
            r: f32::sqrt(start.r * (1.0 - percent) + end.r * percent).floor(),
            g: f32::sqrt(start.g * (1.0 - percent) + end.g * percent).floor(),
            b: f32::sqrt(start.b * (1.0 - percent) + end.b * percent).floor(),
        }
    }
}

fn main() {
    let mut sys = System::new();

    // start and ending color of the gradient
    let green = Color::new_squared(91.0, 227.0, 114.0);
    let red = Color::new_squared(238.0, 73.0, 73.0);

    // stores the current color of the gradient depending on percentage
    let mut color = Color::default();

    // Loopy loop forever :)
    loop {
        sys.refresh_cpu();

        sys.cpus().iter().for_each(|cpu| {
            color = Color::new_from_gradient(&green, &red, &cpu.cpu_usage());
            print!(
                "%{{F#{}}}{:3.0}% %{{F-}}",
                color.hex_values(),
                &cpu.cpu_usage()
            );
        });

        // Newline print to push updated text to polybar
        print!("\n");

        // sleep 1 second to not use too many resources
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
