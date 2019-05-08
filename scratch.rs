use some_crate::{UiElement, ListBox};

#[derive(Clone)]
pub struct User {
    name: String,
    id: u64,
}

pub struct UserList {
    container: UiElement,
    entries: HashMap<u64, Child<UiElement, UserEntry>>,
    sort_order: Vec<u64>,
}

impl UserList {
    fn populate(&mut self, factory: &ChildFactory, users: &[User]) {
        self.sort_order = users.iter().map(|user| user.id).collect::<u64>();

        let mut marked: HashSet<u64> = HashSet::new();
        for user in users {
            self.entries.insert(user.id, factory.mount(user.Clone()));
            marked.insert(user.id);
        }

        self.entries.retain(|id, child| {
            !marked.contains(id)
        }
    }
}

impl Component<UiElement> for UserList {
    type Props = Vec<User>;
    type State = ();

    fn create(initial_props: &Self::Props, factory: &ChildFactory) -> (Self, Self::State) {
        let mut list = UserList {
            container: ListBox::new().into(),
            entries: HashMap::new()
        };
        list.populate(factory, &initial_props[..]);
        (list, ())
    }

    fn update(&mut self, ctx: UpdateContext<UiElement, Self>) {
        self.populate(ctx.factory, &ctx.props[..]);
    }

    fn render<'a>(&'a self) -> Element<'a, UiElement> {
        let mut children = vec![];

        for id in &self.sort_order {
            let child = self.entries.get(id).unwrap();
            children.push(child.render());
        }

        Element::new(&self.container, children)
    }
}