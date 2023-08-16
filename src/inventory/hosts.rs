// Jetporch
// Copyright (C) 2023 - Michael DeHaan <michael@michaeldehaan.net> + contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// long with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::{HashMap};
use crate::util::yaml::{blend_variables};
use std::sync::Arc;
use std::sync::Mutex;
use crate::inventory::groups::Group;
use std::sync::RwLock;

pub struct Host {
    pub name : String,
    pub variables : String,
    pub groups : HashMap<String,Arc<RwLock<Group>>>
}

impl Host {

    pub fn new(name: &String) -> Self {
        Self {
            name: name.clone(),
            variables: String::new(),
            groups: HashMap::new()
        }
    }

    // ==============================================================================================================
    // PUBLIC API - most code can use this
    // ==============================================================================================================
  
    pub fn get_groups(&self) -> HashMap<String, Arc<RwLock<Group>>> {
        let mut results : HashMap<String, Arc<RwLock<Group>>> = HashMap::new();
        for (k,v) in self.groups.iter() {
            results.insert(k.clone(), Arc::clone(&v));
        }
        return results;
    }

    pub fn get_group_names(&self) -> Vec<String> {
        return self.get_groups().iter().map(|(k,v)| k.clone()).collect();
    }

    pub fn add_group(&mut self, name: &String, group: Arc<RwLock<Group>>) {
        self.groups.insert(name.clone(), Arc::clone(&group));
    }

    pub fn get_ancestor_groups(&self) -> HashMap<String, Arc<RwLock<Group>>> {

        let mut results : HashMap<String, Arc<RwLock<Group>>> = HashMap::new();
        for (k,v) in self.get_groups().into_iter() {
            results.insert(k, Arc::clone(&v));
            for (k2,v2) in v.read().unwrap().get_ancestor_groups().into_iter() { 
                results.insert(k2.clone(), Arc::clone(&v2)); 
            }
        }
        return results;
    }

    pub fn get_ancestor_group_names(&self) -> Vec<String> {
        return self.get_ancestor_groups().iter().map(|(k,v)| k.clone()).collect();
    }

    pub fn get_variables(&self) -> String {
        return self.variables.clone();
    }

    pub fn set_variables(&mut self, yaml_string: &String) {
        self.variables.clear();
        self.variables.push_str(&yaml_string.clone());
    }

    pub fn get_blended_variables(&self) -> String {
        let mut blended = String::from("");
        for (_k,ancestor) in self.get_ancestor_groups().into_iter() {
            let theirs = ancestor.read().unwrap().get_variables();
            blended = blend_variables(&theirs.clone(), &blended.clone());
        }
        let mine = self.get_variables();
        return blend_variables(&mine.clone(), &blended.clone());
    }

}
