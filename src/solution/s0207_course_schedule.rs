use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut graph = vec![Vec::new(); num_courses];
        
        for prereq in prerequisites {
            let course = prereq[0] as usize;
            let prerequisite = prereq[1] as usize;
            graph[course].push(prerequisite);
        }
        
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();
        
        for course in 0..num_courses {
            if Self::has_cycle(course, &graph, &mut visited, &mut recursion_stack) {
                return false;
            }
        }
        
        true
    }
    
    fn has_cycle(course: usize,graph: &Vec<Vec<usize>>,visited: &mut HashSet<usize>,recursion_stack: &mut HashSet<usize>) -> bool {
        if recursion_stack.contains(&course) {
            return true;
        }
        
        if visited.contains(&course) {
            return false;
        }
        
        visited.insert(course);
        recursion_stack.insert(course);
        
        for &prereq in &graph[course] {
            if Self::has_cycle(prereq, graph, visited, recursion_stack) {
                return true;
            }
        }
        
        recursion_stack.remove(&course);
        
        false
    }
}