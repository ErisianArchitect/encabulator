
/*
Nodes have a two-way relationship.
Root nodes hold connections to compound objects,
and compound objects hold connections to root nodes.
A compound object is an object that has has a single node that connects to the root.
Example:
    Root
    |---Compound Object
        |---Child 1
            |---Grandchild 1
            |---Grandchild 2
        |---Child 2
        |---Child 3
        |---Child 4

Nodes withing a compound object can be connected to each other so long as the path to the root for all
descendent objects passes through a single node bottleneck.


*/


use std::{collections::{HashMap, HashSet}, num::NonZeroUsize, ptr::NonNull, sync::{RwLock, atomic::AtomicUsize}};

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(Option<NonZeroUsize>);

impl Id {
    #[inline(always)]
    pub const fn null() -> Self {
        Self(None)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_null(self) -> bool {
        self.0.is_none()
    }
    
    #[must_use]
    #[inline(always)]
    fn next() -> Id {
        // start at 1 so that 0 ID can be reserved.
        static ID_ITER: AtomicUsize = AtomicUsize::new(1);
        let next_id = ID_ITER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        unsafe {
            ::core::mem::transmute(next_id)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Edge {
    parent: Id,
    child: Id,
}

impl Edge {
    #[must_use]
    #[inline(always)]
    pub const fn new(child: Id, parent: Id) -> Self {
        Self { child, parent }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn child(self) -> Id {
        self.child
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn parent(self) -> Id {
        self.parent
    }
}

struct EdgeHandler {
    edges: HashSet<Edge>
}

#[repr(C)]
struct Header {
    id: Id,
    parent_connections: RwLock<HashMap<Id, BidirectionalConnection>>,
    root_connections: RwLock<HashMap<Id, BidirectionalConnection>>,
}

struct BidirectionalConnection {
    connection: NonNull<Header>,
    
}

struct BidirectionalConnectionMap {
    map: RwLock<HashMap<Id, BidirectionalConnection>>,
}

#[repr(C)]
struct CompoundNode {
    header: Header,
    
}

struct NodeInner {
    root_connections: usize,
    
}

pub struct Node<> {
    
}