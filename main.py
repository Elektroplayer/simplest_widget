import psutil
import gi

gi.require_version('Gtk', '3.0')
gi.require_version('GtkLayerShell', '0.1')

from gi.repository import Gtk, Gdk, GtkLayerShell, GLib
import cairo

def screen_changed(widget, old_screen, userdata=None):
    widget.set_visual(widget.get_screen().get_rgba_visual())

def expose_draw(widget, event, userdata=None):
    cr = Gdk.cairo_create(widget.get_window())

    cr.set_source_rgba(1.0, 1.0, 1.0, 0.0) 
    cr.set_operator(cairo.OPERATOR_SOURCE)
    cr.paint()

    return False

class SystemData:
    @staticmethod
    def cpu():
        return str(int(psutil.cpu_percent())) + "%"
    
    @staticmethod
    def cpu_temp():
        return str(int(psutil.sensors_temperatures()['dell_smm'][0].current)) + "°"
    
    @staticmethod
    def used_memory():
        return str(psutil.virtual_memory().percent) + "%"

    @staticmethod
    def total_memory():
        return str(round(psutil.virtual_memory().total/(1024*1024*1024),2)) + " GB"

    @staticmethod
    def total_swap():
        return str(round(psutil.swap_memory().total/(1024*1024*1024),2)) + " GB"
    
    @staticmethod
    def used_swap():
        return str(int(psutil.swap_memory().used/psutil.swap_memory().total)) + "%"


class TimeLabel(Gtk.Label):
    def __init__(self):
        Gtk.Label.__init__(self, "")

        GLib.timeout_add_seconds(1, self.updateTime)

        self.updateTime()

    def updateTime(self):
        text = f'<b>Процессор:</b> {SystemData.cpu()} ({SystemData.cpu_temp()})\n<b>Память:</b> {SystemData.used_memory()} / {SystemData.total_memory()}\n<b>Подкачка:</b> {SystemData.used_swap()} / {SystemData.total_swap()}'

        self.set_markup(text)

        return GLib.SOURCE_CONTINUE

if __name__ == "__main__":
    window = Gtk.Window()

    window.connect("delete-event", Gtk.main_quit)
    window.connect("draw", expose_draw)
    window.connect("screen-changed", screen_changed)

    GtkLayerShell.init_for_window(window)
    GtkLayerShell.set_layer(window, 1)
    GtkLayerShell.set_exclusive_zone(window, 0)

    screen = Gdk.Screen.get_default()
    provider = Gtk.CssProvider()
    style_context = Gtk.StyleContext()
    style_context.add_provider_for_screen(screen, provider, Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION)
    provider.load_from_path("./style.css")

    GtkLayerShell.set_anchor(window, GtkLayerShell.Edge.LEFT, True)
    GtkLayerShell.set_anchor(window, GtkLayerShell.Edge.TOP, True)

    GtkLayerShell.set_margin(window, GtkLayerShell.Edge.TOP, 10)
    GtkLayerShell.set_margin(window, GtkLayerShell.Edge.LEFT, 1160)

    vbox = Gtk.Box(orientation=Gtk.Orientation.VERTICAL)

    label = TimeLabel()
    label.set_justify(Gtk.Justification.LEFT)

    vbox.add(label)

    window.add(vbox)

    window.set_default_size(400, 400)
    window.set_app_paintable(True)
    window.set_decorated(False)

    screen_changed(window, None, None)

    window.show_all()
    Gtk.main()