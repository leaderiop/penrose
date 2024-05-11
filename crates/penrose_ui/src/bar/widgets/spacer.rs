//! Widgets for the penrose status bar
use crate::{
    bar::widgets::Widget,
    core::{Context, TextStyle},
    Result,
};
use penrose::{ pure::geometry::Rect, x::XConn, Color};

/// A simple workspace indicator for a status bar
#[derive(Clone, Debug, PartialEq)]
pub struct Spacer {
    require_draw: bool,
    bg: Option<Color>,
}

impl Spacer {
    /// Create a new spacer
    pub fn new(style: TextStyle) -> Spacer {
        Spacer {
            require_draw: true,
            bg: style.bg,
        }
    }
}

impl<X: XConn> Widget<X> for Spacer {
    fn draw(
        &mut self,
        ctx: &mut Context<'_>,
        _: usize,
        _: bool,
        w: u32,
        h: u32,
    ) -> Result<()> {
        if let Some(bg) = self.bg {
            ctx.fill_rect(Rect::new(0, 0, w, h), bg)?;
            self.require_draw = false;
        }
        Ok(())
    }

    fn is_greedy(&self) -> bool {
        true
    }
    fn current_extent(&mut self, _: &mut Context<'_>, h: u32) -> Result<(u32, u32)> {
        Ok((0, h))
    }

    fn require_draw(&self) -> bool {
        self.require_draw
    }
}
