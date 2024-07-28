use web_sys::CanvasRenderingContext2d;

pub(crate) trait CanvasClear {
    fn clear(&self) -> Result<(), ()>;
}

impl CanvasClear for CanvasRenderingContext2d {
    fn clear(&self) -> Result<(), ()> {
        let canvas = self.canvas().ok_or(())?;
        self.clear_rect(-((canvas.width() / 2) as f64), -((canvas.height() / 2) as f64), canvas.width() as f64, canvas.height() as f64);
        Ok(())
    }
}
