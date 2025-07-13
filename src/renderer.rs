use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Corner {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}
impl Corner {
    pub const fn new(top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }
    pub const fn all(rounding: f32) -> Self {
        Self::new(rounding, rounding, rounding, rounding)
    }
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
}

pub struct Renderer<'a> {
    // TODO: In the future, the struct may be used to create WGPU instance.
    // For now, contain raylib draw handle
    pub handle: RaylibDrawHandle<'a>,
}
impl<'a> Renderer<'a> {
    pub const fn from_handle(handle: RaylibDrawHandle<'a>) -> Self {
        Self { handle }
    }
    pub fn draw_rectangle(
        &mut self,
        position: Vector2,
        size: Vector2,
        corner: Corner,
        color: Color,
    ) {
        if corner == Corner::ZERO {
            self.handle.draw_rectangle_v(position, size, color);
        } else {
            // max radius must be such that full rounding causes one or the both side to have no rectangles
            let max_radius = f32::min(size.x, size.y) / 2.0;
            let tl = f32::min(corner.top_left, max_radius);
            let tr = f32::min(corner.top_right, max_radius);
            let br = f32::min(corner.bottom_right, max_radius);
            let bl = f32::min(corner.bottom_left, max_radius);

            // draw arcs
            let corners = [
                Vector2::new(tl, tl),
                Vector2::new(size.x - tr, tr),
                Vector2::new(size.x - br, size.y - br),
                Vector2::new(bl, size.y - bl),
            ];
            let radiuses = [tl, tr, br, bl];
            for (i, (corner, radius)) in corners.into_iter().zip(radiuses).enumerate() {
                let start_angle = i as f32 * 90.0 - 180.0;
                let end_angle = start_angle + 90.0;
                self.handle.draw_circle_sector(
                    position + corner,
                    radius,
                    start_angle,
                    end_angle,
                    32,
                    color,
                );
            }

            // draw rectangles
            // rectangles reach till the center to ensure full coverage
            // array order top, right, bottom, left sides
            let starts = [
                Vector2::new(tl, 0.0),
                Vector2::new(size.x * 0.5, tr),
                Vector2::new(bl, size.y * 0.5),
                Vector2::new(0.0, tl),
            ];
            let ends = [
                Vector2::new(size.x - tr, size.y * 0.5),
                Vector2::new(size.x, size.y - br),
                Vector2::new(size.x - br, size.y),
                Vector2::new(size.x * 0.5, size.y - bl),
            ];
            for (start, end) in starts.into_iter().zip(ends) {
                self.handle
                    .draw_rectangle_v(position + start, end - start, color);
            }
        }
    }
}
