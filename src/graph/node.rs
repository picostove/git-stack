use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub commit: std::rc::Rc<crate::git::Commit>,
    pub branches: Vec<crate::git::Branch>,
    pub action: crate::graph::Action,
    pub pushable: bool,
    pub children: BTreeSet<git2::Oid>,
}

impl Node {
    pub fn new(commit: std::rc::Rc<crate::git::Commit>) -> Self {
        let branches = Vec::new();
        let children = BTreeSet::new();
        Self {
            commit,
            branches,
            action: crate::graph::Action::Pick,
            pushable: false,
            children,
        }
    }

    pub fn with_branches(mut self, possible_branches: &mut crate::git::Branches) -> Self {
        self.branches = possible_branches
            .remove(self.commit.id)
            .unwrap_or_else(Vec::new);
        self
    }

    pub fn update(&mut self, mut other: Self) {
        assert_eq!(self.commit.id, other.commit.id);

        let mut branches = Vec::new();
        std::mem::swap(&mut other.branches, &mut branches);
        self.branches.extend(branches);

        if other.action != crate::graph::Action::Pick {
            self.action = other.action;
        }

        if other.pushable {
            self.pushable = true;
        }

        self.children.extend(other.children);
    }
}
