pub trait Widget {
    fn build(&self) -> &dyn Widget;
}

use core::fmt::Debug;
impl Debug for dyn Widget {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Core Widget")
    }
}

#[derive(Debug)]
pub struct Text {
    value: String,
    font_size: u32,
}

impl Widget for Text {
    fn build(&self) -> &dyn Widget {
        let value = &self.value;
        let font_size = &self.font_size;
        println!("{value} with {font_size}");
        return self;
    }
}

#[derive(Debug)]
pub struct Column<T: Widget> {
    children: Vec<T>,
}

impl<T> Widget for Column<T>
where
    T: Widget,
{
    fn build(&self) -> &dyn Widget {
        for child in &self.children {
            child.build();
        }
        return self;
    }
}

pub struct App<T: Widget> {
    widget_tree: T,
}

impl<T> App<T>
where
    T: Widget,
{
    fn run(&self) -> &dyn Widget {
        self.widget_tree.build()
    }
}

fn main() {
    let app = App::<Column<Text>> {
        widget_tree: Column {
            children: vec![
                Text {
                    font_size: 30,
                    value: String::from("Hello Rustter"),
                },
                Text {
                    font_size: 100,
                    value: String::from("THIS WILL BLAZINGLY FAST!!!"),
                },
            ],
        },
    };

    app.run();
}
