use crate::{
    makepad_derive_widget::*,
    makepad_draw::*,
    view::*,
    widget::*,
};

live_design!{
    SlidePanelBase = {{SlidePanel}} {}
}

#[derive(Live)]
pub struct SlidePanel {
    #[deref] frame: View,
    #[animator] animator: Animator,
    #[live] closed: f64,
    #[live] side: SlideSide,
    #[rust] next_frame: NextFrame
}

impl LiveHook for SlidePanel {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, SlidePanel)
    }
}

#[derive(Clone, WidgetAction)]
pub enum SlidePanelAction {
    None,
}

impl Widget for SlidePanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut WidgetScope)->WidgetActions {
        let mut actions = WidgetActions::new();
        //let uid = self.widget_uid();
        actions.extend(self.frame.handle_event(cx, event, scope));
        // lets handle mousedown, setfocus
        if self.animator_handle_event(cx, event).must_redraw() {
            self.frame.redraw(cx);
        }
        
        match event {
            Event::NextFrame(ne) if ne.set.contains(&self.next_frame) => {
                self.frame.redraw(cx);
            }
            _ => ()
        }
        actions
    }
    
    fn walk(&mut self, cx:&mut Cx) -> Walk {
        self.frame.walk(cx)
    }
    
    fn redraw(&mut self, cx: &mut Cx) {
        self.frame.redraw(cx)
    }
    
    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.frame.find_widgets(path, cached, results);
    }
    
    fn draw_walk_widget(&mut self, cx: &mut Cx2d, scope:&mut WidgetScope, mut walk: Walk) -> WidgetDraw {
        // ok lets set abs pos
        let rect = cx.peek_walk_turtle(walk);
        match self.side{
            SlideSide::Top=>{
                walk.abs_pos = Some(dvec2(0.0, -rect.size.y * self.closed));
            }
            SlideSide::Left=>{
                walk.abs_pos = Some(dvec2(-rect.size.x * self.closed, 0.0));
            }
        }
        self.frame.draw_walk_widget(cx, scope, walk)
    }
}

#[derive(Live, LiveHook)]
#[live_ignore]
pub enum SlideSide{
    #[pick] Left,
    Top
}

impl SlidePanel {

    pub fn open(&mut self, cx: &mut Cx) {
        self.frame.redraw(cx);
    }
    
    pub fn close(&mut self, cx: &mut Cx) {
        self.frame.redraw(cx);
    }
    
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.frame.redraw(cx);
    }
}

// ImGUI convenience API for Piano
#[derive(Clone, PartialEq, WidgetRef)]
pub struct SlidePanelRef(WidgetRef);

impl SlidePanelRef {
    pub fn close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, id!(closed.on))
        }
    }
    pub fn open(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, id!(closed.off))
        }
    }
    pub fn toggle(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            if inner.animator_in_state(cx, id!(closed.on)){
                inner.animator_play(cx, id!(closed.off))
            }
            else{
                inner.animator_play(cx, id!(closed.on))
            }
        }
    }
}

#[derive(Clone, WidgetSet)]
pub struct SlidePanelSet(WidgetSet);

impl SlidePanelSet {
    pub fn close(&self, cx: &mut Cx) {
        for item in self.iter() {
            item.close(cx);
        }
    }
    pub fn open(&self, cx: &mut Cx) {
        for item in self.iter() {
            item.open(cx);
        }
    }
}

