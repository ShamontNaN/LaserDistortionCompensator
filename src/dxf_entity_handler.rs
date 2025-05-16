use dxf::entities::*;

trait entity_handler{
    fn correction(&self,xDimention:f64,yDimention:f64) -> Result<Ok,Err>;
}

impl entity_handler for Line{
    fn correction()
}