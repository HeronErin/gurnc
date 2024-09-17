use std::{borrow::Borrow, cell::{Ref, RefCell}, collections::HashMap, hash::Hash, rc::Rc};
pub enum ScopeDataVariety{

}

pub enum ScopeData{
    Scope{
        variety : ScopeDataVariety,
        parent : Option<Rc<RefCell<ScopeData>>>,
        children : HashMap<String, Rc<RefCell<ScopeData>>>
    },

    // Todo
    Function(),
    Constant(),
    Struct(),
    Enum(),
}

impl ScopeData{
    // Search until a node with a name is found
    pub fn symbol_search(mut current : Rc<RefCell<Self>>, findme : &String) -> Option<Rc<RefCell<Self>>>{
        loop{
            current = {
                let current = current.as_ref().borrow();
                let (parent, children) = match &*current{
                    Self::Scope{variety : _, parent, children} => (parent, children),
                    _ => return None
                };
                if let Some(found) = children.get(findme){
                    return Some(found.clone());
                }
                parent.clone()?
            };
        }
    }
}
