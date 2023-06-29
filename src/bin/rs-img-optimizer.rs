use clap::Parser;
use anyhow::Result;
use rs_img::ImgOptimizer;

fn main() -> Result<()> {
    
    let rs_img_optimizer = ImgOptimizer::parse();

    rs_img_optimizer.exec()?;
    
    Ok(())
}