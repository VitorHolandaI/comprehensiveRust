#[derive(Debug)]
enum Floor {
    F0 = 0,
    F1 = 1,
    F2 = 2,
    F3 = 3,
    F4 = 4,
}

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    GoUp,
    GoDown,
    Stop,
    OpenDoor,
    GoToFloor(Floor),
    CloseDoor,
}

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::Stop
}
/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::OpenDoor
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::CloseDoor
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    match dir {
        Direction::Up => Event::GoUp,
        Direction::Down => Event::GoDown,
    }
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    match floor {
        0 => Event::GoToFloor(Floor::F0),
        1 => Event::GoToFloor(Floor::F1),
        2 => Event::GoToFloor(Floor::F2),
        3 => Event::GoToFloor(Floor::F3),
        4 => Event::GoToFloor(Floor::F4),
    }
    
}

fn main() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!(
        "The car has arrived on the ground floor: {:?}",
        car_arrived(0)
    );
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}
