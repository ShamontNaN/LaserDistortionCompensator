use dxf::Drawing;

pub fn read(path:String)-> dxf::DxfResult<Drawing> {
    let drawing = Drawing::load_file(path)?;
    Ok(drawing)
} 

