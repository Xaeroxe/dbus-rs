
use crate::{MethodErr};
use crate::stdimpl::PropCtx;

#[derive(Debug)]
pub struct Context {
    path: dbus::Path<'static>,
    interface: Option<dbus::strings::Interface<'static>>,
    method: dbus::strings::Member<'static>,
    message: dbus::Message,

    prop_ctx: Option<PropCtx>,
    reply: Option<dbus::Message>,
    send_extra: Vec<dbus::Message>,
}

impl Context {
    pub fn new(msg: dbus::Message) -> Option<Self> {
        if msg.msg_type() != dbus::MessageType::MethodCall { return None; }
        let p = msg.path()?.into_static();
        let i = msg.interface().map(|i| i.into_static());
        let m = msg.member()?.into_static();
        Some(Context {
            path: p,
            interface: i,
            method: m,
            message: msg,
            reply: None,
            prop_ctx: None,
            send_extra: vec!(),
        })
    }

    pub fn check<R, F: FnOnce(&mut Context) -> Result<R, MethodErr>>(&mut self, f: F) -> Result<R, ()> {
        f(self).map_err(|e| {
            if !self.message.get_no_reply() {
                self.reply = Some(e.to_message(&self.message))
            };
        })
    }

    pub fn do_reply<F: FnOnce(&mut dbus::Message)>(&mut self, f: F) {
        if self.message.get_no_reply() { return; }
        if self.reply.is_some() { return; }
        let mut msg = self.message.method_return();
        f(&mut msg);
        self.reply = Some(msg);
    }

    pub fn set_reply(&mut self, msg: Option<dbus::Message>, check_no_reply: bool, check_set: bool) {
        if check_no_reply && self.message.get_no_reply() { return; }
        if check_set && self.reply.is_some() { return; }
        self.reply = msg;
    }

    pub fn flush_messages<S: dbus::channel::Sender>(&mut self, conn: &S) -> Result<(), ()> {
        if let Some(msg) = self.reply.take() {
            conn.send(msg)?;
        }
        for msg in self.send_extra.drain(..) {
            conn.send(msg)?;
        }
        Ok(())
    }


    pub fn make_signal<'b, A, N>(&self, name: N, args: A) -> dbus::Message
    where A: dbus::arg::AppendAll, N: Into<dbus::strings::Member<'b>> {
        let mut msg = dbus::Message::signal(&self.path, self.interface.as_ref().unwrap(), &name.into());
        args.append(&mut dbus::arg::IterAppend::new(&mut msg));
        msg
    }

    /// Adds an extra message to send together with the message reply, e g, a custom signal.
    pub fn push_msg(&mut self, msg: dbus::Message) { self.send_extra.push(msg); }

    pub fn path(&self) -> &dbus::Path<'static> { &self.path }
    pub fn interface(&self) -> Option<&dbus::strings::Interface<'static>> { self.interface.as_ref() }
    pub fn method(&self) -> &dbus::strings::Member<'static> { &self.method }
    pub fn message(&self) -> &dbus::Message { &self.message }

    pub fn has_reply(&self) -> bool { self.reply.is_some() }

    pub (crate) fn take_prop_ctx(&mut self) -> PropCtx { self.prop_ctx.take().unwrap() }
    pub (crate) fn give_prop_ctx(&mut self, p: PropCtx) { self.prop_ctx = Some(p); }
    pub (crate) fn prop_ctx(&mut self) -> &mut PropCtx { self.prop_ctx.as_mut().unwrap() }

}
