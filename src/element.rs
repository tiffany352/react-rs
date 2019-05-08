pub trait HostElement: 'static + Sized + PartialEq {}

enum ElementChildren<'a, H>
where
    H: HostElement,
{
    Owned(Vec<Element<'a, H>>),
    Borrowed(&'a [Element<'a, H>]),
}

pub enum Element<'a, H>
where
    H: HostElement,
{
    Host {
        element: &'a H,
        children: ElementChildren<'a, H>,
    },
    Fragment(ElementChildren<'a, H>),
}

impl<'a, H: HostElement> Element<'a, H> {
    pub fn new<Children>(elt: &'a H, children: Children) -> Element<'a, H>
    where
        Children: Into<ElementChildren<'a, H>>,
    {
        Element::Host {
            element: elt,
            children: children.into(),
        }
    }

    pub fn create_fragment<Children>(children: Children) -> Element<'a, H>
    where
        Children: Into<ElementChildren<'a, H>>,
    {
        Element::Fragment(children.into())
    }
}

impl<'a, H> From<Vec<Element<'a, H>>> for ElementChildren<'a, H>
where
    H: HostElement,
{
    fn from(vec: Vec<Element<'a, H>>) -> Self {
        ElementChildren::Owned(vec)
    }
}

impl<'a, H> From<&'a [Element<'a, H>]> for ElementChildren<'a, H>
where
    H: HostElement,
{
    fn from(arr: &'a [Element<'a, H>]) -> Self {
        ElementChildren::Borrowed(arr)
    }
}
