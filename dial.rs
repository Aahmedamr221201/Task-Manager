//use fltk::button::Button as FltkButton;
use fltk::{/*app*/ frame::Frame as FltkFrame, prelude::*, window::Window as FltkWindow, group::Group as FltkGroup};
use fltk::{enums::*, prelude::*};
use std::cell::RefCell;
use std::rc::Rc;
use fltk::widget_extends;
use fltk::draw;
use fltk::group::Flex as GroupFlex;
use fltk_extras::dial::*;
#[derive(Debug, Clone)]
pub struct MyDial {
    main_wid: FltkGroup,
    value: Rc<RefCell<i32>>,
    value_frame: FltkFrame,
}
widget_extends!(MyDial, FltkGroup, main_wid);
impl MyDial {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let value = Rc::from(RefCell::from(0));
        let mut main_wid = FltkGroup::new(x, y, w, h, None)
            .with_label(label)
            .with_align(Align::Top);
        let mut value_frame =
            FltkFrame::new(main_wid.x(), main_wid.y() + 80, main_wid.w(), 40, "0");
        value_frame.set_label_size(26);
        main_wid.end();
        let value_c = value.clone();
        main_wid.draw(move |w| {
            draw::set_draw_rgb_color(230, 230, 230);
            draw::draw_pie(w.x(), w.y(), w.w(), w.h(), 0., 180.);
            draw::set_draw_hex_color(0xb0bf1a);
            draw::draw_pie(
                w.x(),
                w.y(),
                w.w(),
                w.h(),
                (100 - *value_c.borrow()) as f64 * 1.8,
                180.,
            );
            draw::set_draw_color(Color::from_u32(0x797979));
            draw::draw_pie(
                w.x() - 50 + w.w() / 2,
                w.y() - 50 + w.h() / 2,
                100,
                100,
                0.,
                360.,
            );
            w.draw_children();
        });
        Self {
            main_wid,
            value,
            value_frame,
        }
    }
    pub fn value(&self) -> i32 {
        *self.value.borrow()
    }
    pub fn set_value(&mut self, val: i32) {
        *self.value.borrow_mut() = val;
        self.value_frame.set_label(&val.to_string());
        self.main_wid.redraw();
    }
}