use std::path::Path;
use vtracer::{convert_image_to_svg, Config};

fn main() {
    //当前路径 /Users/angcyo/project/rust/rust-vtracer-demo
    let current_path = std::env::current_dir().unwrap();
    println!("current_path:{}", current_path.display());

    let input_path = Path::new("tests/小孩.jpeg");
    let output_path = Path::new(".output/小孩.svg");
    convert_image_to_svg(input_path, output_path, Config::default()).unwrap();
    //println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use visioncortex::PathSimplifyMode;
    use vtracer::{convert_image_to_svg, ColorMode, Config, Hierarchical};

    #[test]
    fn test() {
        //当前路径 /Users/angcyo/project/rust/rust-vtracer-demo
        let current_path = std::env::current_dir().unwrap();
        println!("current_path:{}", current_path.display());

        //let input_path = Path::new("tests/小孩.jpeg");
        let input_path = Path::new("tests/老虎.jpeg");
        let output_path = Path::new(".output/老虎2.svg");
        //convert_image_to_svg(input_path, output_path, Config::default()).unwrap();
        convert_image_to_svg(
            input_path,
            output_path,
            Config {
                color_mode: ColorMode::Color,
                hierarchical: Hierarchical::Cutout, /*Hierarchical::Stacked*/
                mode: PathSimplifyMode::Polygon,    /*PathSimplifyMode::Spline*/
                filter_speckle: 4,
                color_precision: 6,
                layer_difference: 16,
                corner_threshold: 60,
                length_threshold: 4.0,
                splice_threshold: 45,
                max_iterations: 10,
                path_precision: Some(2),
                ..Config::default()
            },
        )
        .unwrap();
        //println!("Hello, world!");
    }
}
