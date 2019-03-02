use element::{HostElement};

struct HostNode<H>
where
    H: HostElement,
{
    element: H,
}
