mod style;
use style::{TableStyle, TABLE_STYLE_MAP};

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

type Coord = (i32, i32);

#[derive(Clone, Debug)]
pub struct TableItem {
    pub locate: Coord,
    pub content: Option<String>,
    pub content_len: i32,
    pub style: TableStyle,
}

impl TableItem {
    pub fn new(locate: Coord) -> TableItem {
        TableItem {
            locate,
            content: None,
            content_len: 0,
            style: TableStyle::default_style(),
        }
    }
    // pub fn
}

#[derive(Debug)]
pub struct SelectedTableItem<'t> {
    pub table_item: RefMut<'t, TableItem>,
}

impl<'t> SelectedTableItem<'t> {
    fn new(table_item: RefMut<'t, TableItem>) -> SelectedTableItem<'t> {
        SelectedTableItem { table_item }
    }

    pub fn edit_content(&mut self, new_content: String) {
        self.table_item.content = Some(new_content);
    }

    pub fn edit_style(&mut self, new_style: TableStyle) {
        self.table_item.style = new_style;
    }
}

#[derive(Debug)]
pub struct TableRow {
    pub id: i32,
    pub items: Vec<RefCell<TableItem>>,
}

impl TableRow {
    pub fn new(table: &mut Table, size: i32) -> Rc<TableRow> {
        let id = table.rows.len() as i32;
        let new_row = Rc::new(TableRow {
            id,
            items: {
                let mut items = Vec::with_capacity(size as usize);
                for n in 0..size {
                    items.push(RefCell::new(TableItem::new((id, n))));
                }
                items
            },
        });
        let refed_row = Rc::clone(&new_row);
        table.add_row(new_row);

        refed_row
    }
}

#[derive(Debug)]
pub struct Table {
    pub rows: Vec<Rc<TableRow>>,
    pub column_size: i32,
}

impl<'t> Table {
    pub fn new(initial_row_size: i32, initialize_col_size: i32) -> Table {
        let mut new_table = Table {
            rows: Vec::new(),
            column_size: initialize_col_size,
        };

        for _ in 0..initial_row_size {
            TableRow::new(&mut new_table, initialize_col_size);
        }

        new_table
    }

    pub fn add_row(&mut self, row: Rc<TableRow>) {
        self.rows.push(row);
    }

    pub fn select_item(&'t mut self, coordinate: Coord) -> Option<SelectedTableItem<'t>> {
        let row = self.rows.get(coordinate.0 as usize);
        if row.is_none() {
            return None;
        }

        let item = row.unwrap().items.get(coordinate.1 as usize);
        if item.is_none() {
            return None;
        }

        Some(SelectedTableItem::new(item.unwrap().borrow_mut()))
    }

    pub fn build_table_string(&self) {
        let a = TABLE_STYLE_MAP.get(&TableStyle::RegularBox).unwrap();
        let b = a.connect_diff_style.get(&TableStyle::BoldBox).unwrap();
        println!("{:?}", b.top_edge_conn_left);
    }
}

// impl fmt::Display for Table {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", table_string)
//     }
// }

#[test]
fn create_new_table() {
    let mut table = Table::new(3, 3);
    println!("{:?}", table);
    println!("{:?}", table.select_item((0, 2)));
    table
        .select_item((0, 2))
        .unwrap()
        .edit_content("hello".to_string());
    println!("{:?}", table.select_item((0, 2)));
    table.build_table_string();
}
