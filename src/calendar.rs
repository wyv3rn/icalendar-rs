use crate::components::*;

use std::fmt;
use std::ops::Deref;
use std::convert::Into;


#[derive(Debug)]
pub enum CalendarElement{
    Todo(Todo),
    Event(Event)
}

impl Into<CalendarElement> for Event {
    fn into(self) -> CalendarElement {
        CalendarElement::Event(self)
    }
}

impl Into<CalendarElement> for Todo {
    fn into(self) -> CalendarElement {
        CalendarElement::Todo(self)
    }
}

impl CalendarElement {
    fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        match *self {
            CalendarElement::Todo(ref todo)   => todo.fmt_write(out),
            CalendarElement::Event(ref event) => event.fmt_write(out)
        }
    }
}

/// Represents a calendar
///
/// You can `.add()` `Component`s to this.
#[derive(Default,Debug)]
pub struct Calendar {
    components: Vec<CalendarElement>
}

impl Calendar {

    /// Creates a new Calendar.
    pub fn new() -> Self {
        Default::default()
    }

    #[deprecated(note="Use .push() instead")]
    #[doc(hidden)]
    pub fn add<T:Into<CalendarElement>>(&mut self, component:T) -> &mut Self {
        self.push(component)
    }

    /// Moves all the elements of other into Self, leaving other empty.
    pub fn append(&mut self, other: &mut Calendar) {
        self.components.append(&mut other.components);
    }

    /// Extends this `Calendar` with the contends of another.
    pub fn extend<T, U>(&mut self, other: T)
        where T: IntoIterator<Item=U>,
        U: Into<CalendarElement>
    {
        self.components.extend(other.into_iter().map(|x| x.into()));
    }

    /// Appends an element to the back of the `Calendar`.
    pub fn push<T:Into<CalendarElement>>(&mut self, component:T) -> &mut Self {
        self.components.push(component.into());
        self
    }

    /// Writes `Calendar` into a `Writer` using `std::fmt`.
    fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        write_crlf!(out, "BEGIN:VCALENDAR")?;
        write_crlf!(out, "VERSION:2.0")?;
        write_crlf!(out, "PRODID:ICALENDAR-RS")?;
        write_crlf!(out, "CALSCALE:GREGORIAN")?;

        for component in &self.components {
            component.fmt_write(out)?;
        }
        write_crlf!(out, "END:VCALENDAR")?;
        Ok(())
    }

    /// Prints to stdout
    /// FIXME code repetition
    pub fn print(&self) -> Result<(), fmt::Error> {
        let mut out = String::new();
        self.fmt_write(&mut out)?;
        print_crlf!("{}", out);
        Ok(())
    }
}

impl ToString for Calendar {
    /// # panics
    fn to_string(&self) -> String {
        let mut out_string = String::new();
        self.fmt_write(&mut out_string).unwrap();
        out_string
    }
}

impl Deref for Calendar {
    type Target = [CalendarElement];

    fn deref(&self) -> &[CalendarElement]{
        self.components.deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calendar_extend_components() {
        let mut calendar = Calendar::new();
        let components = vec![
            CalendarElement::Event(Event::new()),
            CalendarElement::Event(Event::new()),
        ];
        calendar.extend(components);
        assert_eq!(calendar.components.len(), 2);
    }

    #[test]
    fn calendar_extend_events() {
        let mut calendar = Calendar::new();
        let events = vec![
            Event::new(),
            Event::new(),
        ];
        calendar.extend(events);
        assert_eq!(calendar.components.len(), 2);
    }
}
