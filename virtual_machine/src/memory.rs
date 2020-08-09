use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::sync::{Arc, RwLock};

use crate::instruction_set::Immediate;
use crate::vm::Fault;
use crate::vm::Fault::{NotAVariable, SegmentationFault};

pub const MAX_MEM: usize = std::usize::MAX;
const MID: usize = MAX_MEM / 2;

#[derive(Debug, Clone)]
pub enum Scope {
    Global,
    Local,
}

#[derive(Clone)]
struct Variables
// A variable can exist for shorter than it's value, but it should not exist for longer than it's value
{
    mapping: HashMap<String, usize>,
}

impl Variables {
    fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct Memory {
    static_memory: Arc<RwLock<HashMap<String, Option<Immediate>>>>,
    memory: Vec<Option<Immediate>>,
    free_list: Vec<usize>,
    local_scope_stack: Vec<Variables>,
    heap: Vec<Box<Immediate>>
}

impl Memory {
    pub fn new() -> Self {
        Self {
            static_memory: Arc::new(RwLock::new(HashMap::new())),
            memory: vec![],
            free_list: vec![],
            local_scope_stack: vec![Variables::new()],
            heap: Vec::new()
        }
    }

    fn get_scope(&self) -> &Variables {
        self.local_scope_stack.last().unwrap()
    }

    fn get_scope_mut(&mut self) -> &mut Variables {
        self.local_scope_stack.last_mut().unwrap()
    }

    pub fn new_lower_scope(&mut self) {
        let new_scope = self.get_scope().clone();
        self.local_scope_stack.push(new_scope);
    }

    pub fn new_local_scope(&mut self) {
        let new_scope = Variables::new();
        self.local_scope_stack.push(new_scope);
    }

    pub fn exit_local_scope(&mut self) {
        self.local_scope_stack.pop().unwrap();
    }

    pub fn declare_variable(&mut self, name: &String, scope: &Scope) {
        let name = name.clone();
        match scope {
            Scope::Global => {
                let mut writer = self.static_memory.write().expect("Statics poisoned");
                if writer.contains_key(&name) {
                    panic!("Variable {} already declared", name);
                }

                writer.insert(name, None);
            }
            Scope::Local => {
                if self.get_scope().mapping.contains_key(&name) {
                    panic!("Variable {} already declared", name);
                }

                let pos = self.free_list.pop().unwrap_or(self.memory.len());

                if pos == self.memory.len() {
                    self.memory.push(None);
                }

                self.get_scope_mut().mapping.insert(name, pos);
            }
        }
    }

    pub fn set_variable(&mut self, name: &String, value: Immediate) -> Result<(), Fault> {
        if !self.get_scope().mapping.contains_key(name) {
            let mut writer = self.static_memory.write().expect("Statics poisoned");
            match writer.get_mut(name) {
                None => {
                    return Err(NotAVariable(name.to_string()));
                }
                Some(mem) => {
                    *mem = Some(value);
                    return Ok(());
                }
            }
        }

        if let Some(pos) = self.get_scope().mapping.get(name).map(|s| *s) {
            let variable = &mut self.memory[pos];
            *variable = Some(value);
        }

        Ok(())
    }

    pub fn get_variable(&self, name: &String) -> Result<Immediate, Fault> {
        if !self.get_scope().mapping.contains_key(name) {
            let mut reader = self.static_memory.read().expect("Statics poisoned");
            match reader.get(name) {
                None => {
                    return Err(NotAVariable(name.to_string()));
                }
                Some(Some(mem)) => {
                    return Ok(mem.clone());
                }
                Some(None) => {
                    return Err(SegmentationFault);
                }
            }
        }

        match self.get_scope().mapping.get(name) {
            None => unreachable!(),
            Some(pos) => {
                if let Some(imm) = self.memory.get(*pos) {
                    match imm {
                        None => Err(SegmentationFault),
                        Some(imm) => Ok(imm.clone()),
                    }
                } else {
                    panic!("{} is not a variable", name);
                }
            }
        }
    }
    pub fn get_variable_ref(&self, name: &String) -> Result<&Immediate, Fault> {
        if !self.get_scope().mapping.contains_key(name) {
            let mut reader = self.static_memory.read().expect("Statics poisoned");
            match reader.get(name) {
                None => {
                    return Err(NotAVariable(name.to_string()));
                }
                Some(Some(mem)) => {
                    return Ok(unsafe { &*(mem as *const Immediate) });
                }
                Some(None) => {
                    return Err(SegmentationFault);
                }
            }
        }

        match self.get_scope().mapping.get(name) {
            None => unreachable!(),
            Some(pos) => {
                if let Some(imm) = self.memory.get(*pos) {
                    match imm {
                        None => Err(SegmentationFault),
                        Some(imm) => Ok(imm),
                    }
                } else {
                    panic!("{} is not a variable", name);
                }
            }
        }
    }

    pub fn get_variable_mut(&mut self, name: &String) -> Result<&mut Immediate, Fault> {
        if !self.get_scope().mapping.contains_key(name) {
            let mut writer = self.static_memory.write().expect("Statics poisoned");
            match writer.get_mut(name) {
                None => {
                    return Err(NotAVariable(name.to_string()));
                }
                Some(Some(mem)) => {
                    return Ok(unsafe { &mut *(mem as *mut Immediate) });
                }
                Some(None) => {
                    return Err(SegmentationFault);
                }
            }
        }

        let option = self.get_scope().mapping.get(name).map(|pos| *pos);
        match option {
            None => unreachable!(),
            Some(pos) => {
                if let Some(imm) = self.memory.get_mut(pos) {
                    match imm {
                        None => Err(SegmentationFault),
                        Some(imm) => Ok(imm),
                    }
                } else {
                    panic!("{} is not a variable", name);
                }
            }
        }
    }

    pub fn heapify(&mut self, imm: Immediate) -> * mut Immediate {
        let mut boxed = Box::new(imm);
        let ptr = &mut *boxed as *mut Immediate;
        self.heap.push(boxed);
        ptr
    }

    pub fn collect_garbage(&mut self) {
        let used_memory = self
            .local_scope_stack
            .iter()
            .map(|vars| vars.mapping.values())
            .flatten()
            .map(|pos| *pos)
            .collect::<HashSet<usize>>();

        let mut unused: Vec<usize> = (0..self.memory.len())
            .into_iter()
            .filter(|pos| used_memory.contains(pos))
            .collect::<Vec<usize>>();

        unused.sort();
        self.free_list = unused;
    }
}
