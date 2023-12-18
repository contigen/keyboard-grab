use rdev::{grab, Event, EventType, Key};

fn main() {
    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error)
    }
    let callback = |event: Event| -> Option<Event> {
        if let EventType::KeyPress(Key::CapsLock) = event.event_type {
            println!("Consuming and cancelling CapsLock");
            None // CapsLock is now effectively disabled
        } else {
            Some(event)
        }
    };
    // This will block.
}
