//! This module contains some standard interfaces and an easy way to call them.
//!
//! See the [D-Bus specification](https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces) for more information about these standard interfaces.
//! 
//! The code here was originally created by dbus-codegen.


// cargo run -- -d org.freedesktop.Notifications -p /org/freedesktop/Notifications -m none -c blocking -g -i "org.freedesktop.DBus"
// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

pub mod org_freedesktop_dbus {

#![allow(missing_docs)]

use crate as dbus;
use crate::arg;
use crate::blocking;

pub trait Properties {
    fn get<R0: for<'b> arg::Get<'b>>(&self, interface_name: &str, property_name: &str) -> Result<R0, dbus::Error>;
    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>, dbus::Error>;
    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: arg::Variant<I2>) -> Result<(), dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> Properties for blocking::Proxy<'a, C> {

    fn get<R0: for<'b> arg::Get<'b>>(&self, interface_name: &str, property_name: &str) -> Result<R0, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface_name, property_name, ))
            .map(|r: (arg::Variant<R0>,)| (r.0).0)
    }

    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface_name, ))
            .map(|r: (::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>,)| r.0)
    }

    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: arg::Variant<I2>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface_name, property_name, value, ))
    }
}

#[derive(Debug)]
pub struct PropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for PropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for PropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(PropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for PropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

pub trait Introspectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> Introspectable for blocking::Proxy<'a, C> {

    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .map(|r: (String,)| r.0)
    }
}

pub trait Peer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> Peer for blocking::Proxy<'a, C> {

    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .map(|r: (String,)| r.0)
    }
}

}

