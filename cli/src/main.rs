use anyhow::Result;
use clap::Parser;
use iro::base24::{generate_palette, Base24Style};
use iro::{parse_colors, Args};

pub fn main() -> Result<()> {
    let args = Args::try_parse().unwrap();
    let mut img = image::open(args.path).unwrap().into_rgb8();
    let colors = generate_palette(parse_colors(&mut img), args.light_mode)?;
    let style = Base24Style {
        name: "Bebop".to_string(),
        author: "".to_string(),
        variant: "dark".to_string(),
        palette: colors,
    };
    println!("{}", serde_yaml::to_string(&style)?); 
    Ok(())
}
