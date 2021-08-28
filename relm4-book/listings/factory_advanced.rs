// ANCHOR: all
use gtk::glib::Sender;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use relm4::factory::{DynamicIndex, Factory, FactoryPrototype, FactoryVecDeque};
use relm4::*;

use std::rc::Rc;

#[derive(Debug)]
// ANCHOR: msg
enum AppMsg {
    AddFirst,
    RemoveLast,
    CountAt(Rc<DynamicIndex>),
    RemoveAt(Rc<DynamicIndex>),
    InsertBefore(Rc<DynamicIndex>),
    InsertAfter(Rc<DynamicIndex>),
}
// ANCHOR_END: msg

// ANCHOR: model
struct Counter {
    value: u8,
}

struct AppModel {
    counters: FactoryVecDeque<Counter>,
    received_messages: u8,
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
        match msg {
            AppMsg::AddFirst => {
                self.counters.push_front(Counter {
                    value: self.received_messages,
                });
            }
            AppMsg::RemoveLast => {
                self.counters.pop_back();
            }
            AppMsg::CountAt(index) => {
                if let Some(counter) = self.counters.get_mut(index.current_index()) {
                    counter.value = counter.value.wrapping_sub(1);
                }
            }
            AppMsg::RemoveAt(index) => {
                self.counters.remove(index.current_index());
            }
            AppMsg::InsertBefore(index) => {
                self.counters.insert(
                    index.current_index(),
                    Counter {
                        value: self.received_messages,
                    },
                );
            }
            AppMsg::InsertAfter(index) => {
                self.counters.insert(
                    index.current_index() + 1,
                    Counter {
                        value: self.received_messages,
                    },
                );
            }
        }
        self.received_messages += 1;
        true
    }
}
// ANCHOR_END: app_update

// ANCHOR: factory_widgets
struct FactoryWidgets {
    hbox: gtk::Box,
    counter_button: gtk::Button,
}
// ANCHOR_END: factory_widgets

impl FactoryPrototype for Counter {
    type Factory = FactoryVecDeque<Self>;
    type Widgets = FactoryWidgets;
    type Root = gtk::Box;
    type View = gtk::Box;
    type Msg = AppMsg;

    // ANCHOR: generate_start
    fn generate(&self, index: &Rc<DynamicIndex>, sender: Sender<AppMsg>) -> FactoryWidgets {
        let hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .build();

        let counter_button = gtk::Button::with_label(&self.value.to_string());
        let index: Rc<DynamicIndex> = index.clone();

        let remove_button = gtk::Button::with_label("Remove");
        let ins_above_button = gtk::Button::with_label("Add above");
        let ins_below_button = gtk::Button::with_label("Add below");
        // ANCHOR_END: generate_start

        // ANCHOR: append
        hbox.append(&counter_button);
        hbox.append(&remove_button);
        hbox.append(&ins_above_button);
        hbox.append(&ins_below_button);
        // ANCHOR_END: append

        // ANCHOR: connect
        {
            let sender = sender.clone();
            let index = index.clone();
            counter_button.connect_clicked(move |_| {
                send!(sender, AppMsg::CountAt(index.clone()));
            });
        }

        {
            let sender = sender.clone();
            let index = index.clone();
            remove_button.connect_clicked(move |_| {
                send!(sender, AppMsg::RemoveAt(index.clone()));
            });
        }

        {
            let sender = sender.clone();
            let index = index.clone();
            ins_above_button.connect_clicked(move |_| {
                send!(sender, AppMsg::InsertBefore(index.clone()));
            });
        }

        ins_below_button.connect_clicked(move |_| {
            send!(sender, AppMsg::InsertAfter(index.clone()));
        });
        // ANCHOR_END: connect

        // ANCHOR: return
        FactoryWidgets {
            hbox,
            counter_button,
        }
        // ANCHOR_END: return
    }

    fn position(&self, _index: &Rc<DynamicIndex>) {}

    fn update(&self, _index: &Rc<DynamicIndex>, widgets: &FactoryWidgets) {
        widgets.counter_button.set_label(&self.value.to_string());
    }
    
    fn get_root(widget: &FactoryWidgets) -> &gtk::Box {
        &widget.hbox
    }
}

struct AppWidgets {
    main: gtk::ApplicationWindow,
    gen_box: gtk::Box,
}

impl Widgets<AppModel, ()> for AppWidgets {
    type Root = gtk::ApplicationWindow;

    fn init_view(_model: &AppModel, _components: &(), sender: Sender<AppMsg>) -> Self {
        let main = gtk::ApplicationWindowBuilder::new()
            .default_width(300)
            .default_height(200)
            .build();
        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_end(5)
            .margin_top(5)
            .margin_start(5)
            .margin_bottom(5)
            .spacing(5)
            .build();

        let gen_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_end(5)
            .margin_top(5)
            .margin_start(5)
            .margin_bottom(5)
            .spacing(5)
            .build();

        let add = gtk::Button::with_label("Add");
        let remove = gtk::Button::with_label("Remove");

        main_box.append(&add);
        main_box.append(&remove);
        main_box.append(&gen_box);

        main.set_child(Some(&main_box));

        let cloned_sender = sender.clone();
        add.connect_clicked(move |_| {
            cloned_sender.send(AppMsg::AddFirst).unwrap();
        });

        remove.connect_clicked(move |_| {
            sender.send(AppMsg::RemoveLast).unwrap();
        });

        AppWidgets { main, gen_box }
    }

    fn view(&mut self, model: &AppModel, sender: Sender<AppMsg>) {
        model.counters.generate(&self.gen_box, sender);
    }

    fn root_widget(&self) -> gtk::ApplicationWindow {
        self.main.clone()
    }
}

fn main() {
    let model = AppModel {
        counters: FactoryVecDeque::new(),
        received_messages: 0,
    };

    let relm = RelmApp::new(model);
    relm.run();
}
// ANCHOR_END: all
