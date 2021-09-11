// ANCHOR: all
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};

// ANCHOR: icons
const ICON_LIST: &[&str] = &[
    "bookmark-new-symbolic",
    "edit-copy-symbolic",
    "edit-cut-symbolic",
    "edit-find-symbolic",
    "starred-symbolic",
    "system-run-symbolic",
    "emoji-objects-symbolic",
    "emoji-nature-symbolic",
    "display-brightness-symbolic",
];

fn random_icon_name() -> &'static str {
    let index: usize = rand::random::<usize>() % ICON_LIST.len();
    ICON_LIST[index]
}
// ANCHOR_END: icons

// ANCHOR: msg
enum AppMsg {
    UpdateFirst,
    UpdateSecond,
}
// ANCHOR_END: msg

// The track proc macro allows to easily track changes to different
// fields of the model
// ANCHOR: model
#[tracker::track]
struct AppModel {
    first_icon: &'static str,
    second_icon: &'static str,
    identical: bool,
}
// ANCHOR_END: model

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

// ANCHOR: app_update
impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        // reset tracker value of the model
        self.reset();

        match msg {
            AppMsg::UpdateFirst => {
                self.set_first_icon(random_icon_name());
            }
            AppMsg::UpdateSecond => {
                self.set_second_icon(random_icon_name());
            }
        }
        self.set_identical(self.first_icon == self.second_icon);

        true
    }
}
// ANCHOR_END: app_update

// ANCHOR: widgets
#[relm4_macros::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = gtk::ApplicationWindow {
        		// ANCHOR: track1
            set_class_active: track!(model.changed(AppModel::identical()),
                "identical", model.identical),
            // ANCHOR_END: track1
            set_child = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_all: 10,
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    append = &gtk::Image {
                        set_pixel_size: 50,
                        // ANCHOR: track2
                        set_icon_name: track!(model.changed(AppModel::first_icon()),
                            Some(model.first_icon)),
                        // ANCHOR_END: track2
                    },
                    append = &gtk::Button {
                        set_label: "New random image",
                        connect_clicked(sender) => move |_| {
                            send!(sender, AppMsg::UpdateFirst);
                        }
                    }
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    append = &gtk::Image {
                        set_pixel_size: 50,
                        set_icon_name: track!(model.changed(AppModel::second_icon()),
                            Some(model.second_icon)),
                    },
                    append = &gtk::Button {
                        set_label: "New random image",
                        connect_clicked(sender) => move |_| {
                            send!(sender, AppMsg::UpdateSecond);
                        }
                    }
                },
            }
        }
    }

		// ANCHOR: post_init
    fn post_init() {
        relm4::set_global_css(b".identical { background: #00ad5c; }");
    }
		// ANCHOR_END: post_init
}
// ANCHOR_END: widgets

// ANCHOR: main
fn main() {
    let model = AppModel {
        first_icon: random_icon_name(),
        second_icon: random_icon_name(),
        identical: false,
        tracker: 0,
    };
    let relm = RelmApp::new(model);
    relm.run();
}
// ANCHOR_END: main
// ANCHOR_END: all
