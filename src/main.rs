use gtk::prelude::{ApplicationExt, ApplicationExtManual, *};
use gtk_layer_shell::{Edge, Layer, LayerShell};
use glib::source::timeout_add_seconds_local;
use sysinfo::{Components, System};

fn activate(application: &gtk::Application) {

    // Инициализация окна
    let window = gtk::ApplicationWindow::new(application);
    
    window.init_layer_shell();
    window.set_layer(Layer::Bottom);
    window.set_exclusive_zone(0);

    // Положение
    window.set_layer_shell_margin(Edge::Top, 10);
    window.set_layer_shell_margin(Edge::Left, 1160);

    // Делаем прозрачным
    window.set_app_paintable(true);

    // Ставим "якоря"
    let anchors = [
        (Edge::Left, true),
        (Edge::Right, false),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    // Настраиваем css провайдера
    let provider = gtk::CssProvider::new();

    let home = std::env::var("HOME").unwrap();
    provider.load_from_path(&format!("{}/.config/sgw/style.css", home)).unwrap();

    let screen = gtk::gdk::Screen::default().unwrap();

    gtk::StyleContext::add_provider_for_screen(&screen, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    // Добавляем текстовый label
    let label = gtk::Label::new(Some(""));
    label.set_markup("Приветствую!");

    // Добавляем и показываем
    window.add(&label);
    window.show_all();

    // Обновление информации в label
    let mut sys = System::new();
    sys.refresh_all();

    // Процессор
    let mut cpu_usage: u8 = 0;
    let mut cpu_temp: u8 = 0;

    // Память
    let mut memory_usage: i8 = 0;
    let memory_total: f32 = ((sys.total_memory() as f32 / (1024.0 * 1024.0 * 1024.0)) * 100.0).round() / 100.0; // Округление до 2 знака

    // Подкачка
    let mut swap_usage: i8 = 0;
    let swap_total: f32 = ((sys.total_swap() as f32 / (1024.0 * 1024.0 * 1024.0)) * 100.0).round() / 100.0; // Округление до 2 знака
    
    // Таймаут
    timeout_add_seconds_local(1, move | | {
        sys.refresh_all();

        cpu_usage = (sys.cpus().iter().fold(0.0,|acc, x| acc + x.cpu_usage()) / 4.0) as u8;
        cpu_temp = Components::new_with_refreshed_list().iter().collect::<Vec<_>>()[0].temperature() as u8;

        memory_usage = ((sys.used_memory() as f32 * 100.0) / (1024.0 * 1024.0 * 1024.0 * memory_total)) as i8;

        swap_usage = ((sys.used_swap() as f32 * 100.0) / (1024.0 * 1024.0 * 1024.0 * swap_total)) as i8;

        label.set_markup(&format!("<b>Процессор:</b> {}% ({}°)\n<b>Память:</b> {}% / {}GB\n<b>Подкачка:</b> {}% / {}GB", cpu_usage, cpu_temp, memory_usage, memory_total, swap_usage, swap_total));
        
        glib::ControlFlow::Continue
    });

}

fn main() {
    let application = gtk::Application::new(Some("sh.wmww.gtk-layer-example"), Default::default());

    application.connect_activate(|app| {
        activate(app);
    });

    application.run();
}