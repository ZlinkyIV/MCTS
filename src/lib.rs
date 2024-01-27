pub trait State {
    type Action;
    
    fn is_terminal(&self) -> bool;
    fn get_value(&self) -> i32;
    fn get_actions(&self) -> Vec<Self::Action>;
    fn get_next_state(&self, action: &Self::Action) -> Self;
}

mod search_tree {
    // use std::rc::Rc;
    // use std::cell::RefCell;
    // use std::thread::current;

    use crate::State;

    use ordered_float::OrderedFloat;

    struct Node<S: State> {
        state: S,
        action: S::Action,
        sum_evaluation: i32,
        num_visits: u32,
        children: Vec<Node<S>>,
        children_discovered: bool,
    }

    impl<S: State> Node<S> {
        fn new(state: S, action: S::Action) -> Self {
            Self {
                state,
                action,
                sum_evaluation: 0,
                num_visits: 0,
                children: Vec::new(),
                children_discovered: false,
            }
        }

        fn evaluation(&self) -> Option<i32> {
            self.sum_evaluation.checked_div(self.num_visits as i32)
        }

        fn add_visit(&mut self, score: i32) {
            self.sum_evaluation += score;
            self.num_visits += 1;
        }

        fn discover_children(&mut self) {
            self.children = self.state.get_actions()
                .iter()
                .map(|action| (action, self.state.get_next_state(action)))
                .map(|(action, state)| Node::new(state, *action))
                .collect();
            self.children_discovered = true;
        }

        fn get_best_action(&self) -> Option<S::Action> {
            self.children
                .iter()
                .max_by_key(|node| node.evaluation().unwrap_or(i32::MIN))
                .map(|node| node.action)
        }


        fn mcts<F>(&mut self, score_func: &F) -> i32
        where
            F: Fn(&Node<S>) -> f32,
        {
            match (self.children.len() != 0, self.num_visits == 0) {
                (true, _) => self.backprop_with_best(score_func),
                (false, true) => self.rollout(),
                (false, false) => {
                    self.discover_children();
                    self.backprop_with_best(score_func)
                },
            }
        }

        fn backprop_with_best<F>(&mut self, score_func: &F) -> i32
        where
            F: Fn(&Node<S>) -> f32,
        {
            let best_child = self.children
                .iter_mut()
                .max_by_key(|node| OrderedFloat(score_func(node)));

            match best_child {
                Some(child) => {
                    let value = child.mcts(score_func);     // Recurse with new node
                    self.add_visit(value);
                    value
                },
                None => self.rollout(),  // Terminal node -- just do rollout
            }
        }

        fn rollout(&mut self) -> i32 {
            let value = self.state.get_value();
            self.add_visit(value);
            value
        }

        
    }

    pub struct SearchTree<S: State> {
        root: Node<S>,
    }

    impl<S: State> SearchTree<S> {
        fn new(root_state: S) -> Self {
            Self {
                root: Node::new(root_state),
            }
        }

        fn update(&mut self) {
            
        }
    }
}