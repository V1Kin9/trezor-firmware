use crate::ui::{
    component::{Child, Component, Event, EventCtx, Label, Pad},
    constant::screen,
    display::Icon,
    geometry::{Alignment, Insets, Point, Rect},
    model_tt::{
        bootloader::theme::{
            button_bld, button_bld_menu, BLD_BG, BUTTON_AREA_START, BUTTON_HEIGHT, CONTENT_PADDING,
            CORNER_BUTTON_AREA, MENU32, TEXT_NORMAL, TEXT_TITLE, TITLE_AREA, TITLE_Y_ADJUSTMENT,
        },
        component::{Button, ButtonMsg::Clicked},
        constant::WIDTH,
    },
};

#[repr(u32)]
#[derive(Copy, Clone, ToPrimitive)]
pub enum IntroMsg {
    Menu = 1,
    Host = 2,
}

pub struct Intro<'a> {
    bg: Pad,
    title: Child<Label<&'a str>>,
    menu: Child<Button<&'static str>>,
    host: Child<Button<&'static str>>,
    text: Child<Label<&'a str>>,
}

impl<'a> Intro<'a> {
    pub fn new(title: &'a str, content: &'a str) -> Self {
        Self {
            bg: Pad::with_background(BLD_BG).with_clear(),
            title: Child::new(Label::new(title, Alignment::Start, TEXT_TITLE)),
            menu: Child::new(
                Button::with_icon(Icon::new(MENU32))
                    .styled(button_bld_menu())
                    .with_expanded_touch_area(Insets::uniform(13)),
            ),
            host: Child::new(Button::with_text("INSTALL FIRMWARE").styled(button_bld())),
            text: Child::new(
                Label::new(content, Alignment::Start, TEXT_NORMAL)
                    .with_vertical_align(Alignment::Center),
            ),
        }
    }
}

impl<'a> Component for Intro<'a> {
    type Msg = IntroMsg;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.bg.place(screen());

        self.title.place(TITLE_AREA);
        let title_height = self.title.inner().area().height();
        self.title.place(Rect::new(
            Point::new(
                CONTENT_PADDING,
                TITLE_AREA.center().y - (title_height / 2) - TITLE_Y_ADJUSTMENT,
            ),
            Point::new(WIDTH - CONTENT_PADDING, BUTTON_AREA_START - CONTENT_PADDING),
        ));

        self.menu.place(CORNER_BUTTON_AREA);
        self.host.place(Rect::new(
            Point::new(CONTENT_PADDING, BUTTON_AREA_START),
            Point::new(WIDTH - CONTENT_PADDING, BUTTON_AREA_START + BUTTON_HEIGHT),
        ));
        self.text.place(Rect::new(
            Point::new(CONTENT_PADDING, TITLE_AREA.y1 + CONTENT_PADDING),
            Point::new(WIDTH - CONTENT_PADDING, BUTTON_AREA_START - CONTENT_PADDING),
        ));
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        if let Some(Clicked) = self.menu.event(ctx, event) {
            return Some(Self::Msg::Menu);
        };
        if let Some(Clicked) = self.host.event(ctx, event) {
            return Some(Self::Msg::Host);
        };
        None
    }

    fn paint(&mut self) {
        self.bg.paint();
        self.title.paint();
        self.text.paint();
        self.host.paint();
        self.menu.paint();
    }

    #[cfg(feature = "ui_bounds")]
    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.menu.bounds(sink);
    }
}
