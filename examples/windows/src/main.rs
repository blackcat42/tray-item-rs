use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};

enum Message {
    TrayLeftClicked,
    TrayRightClicked,
    TrayDoubleClicked,
    TrayMiddleClicked,
    Quit,
    Green,
    Red,
}

fn main() {
    let (tx, rx) = mpsc::sync_channel(1);
    let mut tray = TrayItem::new(
        "Tray Example",
        IconSource::Resource("name-of-icon-in-rc-file"),
    )
    .unwrap();

    tray.inner_mut().set_leftclick_callback({
        let tx = tx.clone();
        move || {
            tx.send(Message::TrayLeftClicked).unwrap();
        }
    });

    tray.inner_mut().set_rightclick_callback({
        let tx = tx.clone();
        move || {
            tx.send(Message::TrayRightClicked).unwrap();
        }
    });

    tray.inner_mut().set_doubleclick_callback({
        let tx = tx.clone();
        move || {
            tx.send(Message::TrayDoubleClicked).unwrap();
        }
    });

    tray.inner_mut().set_middleclick_callback({
        let tx = tx.clone();
        move || {
            tx.send(Message::TrayMiddleClicked).unwrap();
        }
    });

    tray.add_label("Tray Label").unwrap();

    tray.add_menu_item("Hello", || {
        println!("Hello!");
    })
    .unwrap();

    tray.inner_mut().add_separator().unwrap();

    let red_tx = tx.clone();
    tray.add_menu_item("Red", move || {
        red_tx.send(Message::Red).unwrap();
    })
    .unwrap();

    let green_tx = tx.clone();
    tray.add_menu_item("Green", move || {
        green_tx.send(Message::Green).unwrap();
    })
    .unwrap();

    tray.inner_mut().add_separator().unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    let mut toggle = true;

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                break;
            }
            Ok(Message::Red) => {
                println!("Red");
                tray.set_icon(IconSource::Resource("another-name-from-rc-file"))
                    .unwrap();
            }
            Ok(Message::Green) => {
                println!("Green");
                tray.set_icon(IconSource::Resource("name-of-icon-in-rc-file"))
                    .unwrap()
            }
            Ok(Message::TrayLeftClicked) => {
                println!("TrayLeftClicked");
            }
            Ok(Message::TrayRightClicked) => {
                println!("TrayRightClicked");
            }
            Ok(Message::TrayDoubleClicked) => {
                println!("TrayDoubleClicked");
                if toggle {
                    tray.set_icon(IconSource::Resource("another-name-from-rc-file"))
                        .unwrap();
                } else {
                    tray.set_icon(IconSource::Resource("name-of-icon-in-rc-file"))
                        .unwrap();
                }
                toggle = !toggle;
            }
            Ok(Message::TrayMiddleClicked) => {
                println!("TrayMiddleClicked");
            }
            _ => {}
        }
    }
}
