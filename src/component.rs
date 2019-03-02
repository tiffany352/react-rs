use element::ElementBox;

pub trait Component<Props = (), State = ()> {
    fn render(&self, props: &Props, state: &State) -> ElementBox;
}