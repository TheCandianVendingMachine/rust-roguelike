/*
    A roguelike game created for a fun exercise
    Copyright (C) 2023  Bailey Danyluk

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

pub struct SparseSet<T> {
    // Index of element in sparse set represents the object. Value is the pointer
    sparse: Vec<usize>,
    // Index of element in dense set represents the T, value is the object
    dense: Vec<usize>,
    dense_objects: Vec<T>,
    tombstone: usize
}

impl<T> SparseSet<T> {
    pub fn new(length: usize) -> SparseSet<T> {
        let tombstone = length;

        let mut sparse = Vec::new();
        sparse.resize(length + 1, tombstone);

        SparseSet {
            sparse,
            dense: Vec::new(),
            dense_objects: Vec::new(),
            tombstone
        }
    }

    pub fn push(&mut self, element_id: usize, element: T) -> &mut T {
        if !self.contains(element_id) {
            let pos = self.dense.len();
            self.dense.push(element_id);
            self.dense_objects.push(element);
            self.sparse[element_id] = pos;
        }
        self.get_mut(element_id).unwrap()
    }

    pub fn remove(&mut self, element: usize) -> (usize, Option<T>) {
        if !self.contains(element) {
            return (self.tombstone, None)
        }

        let size = self.dense.len() - 1;
        let last = *self.dense.last().unwrap();

        self.dense.swap(size, self.sparse[element]);
        self.dense_objects.swap(size, self.sparse[element]);

        self.sparse.swap(last, element);
        self.sparse[element] = self.tombstone;

        (self.dense.pop().unwrap(), Some(self.dense_objects.pop().unwrap()))
    }

    pub fn contains(&self, element: usize) -> bool{
        element < self.tombstone &&
            self.sparse[element] < self.dense.len() && 
            self.sparse[element] != self.tombstone
    }

    pub fn clear(&mut self) {
        self.dense.clear();
        self.dense_objects.clear();
        self.sparse = self.sparse.iter_mut()
            .map(|_i| -> usize { self.tombstone }).collect();
    }

    pub fn get(&self, element: usize) -> Option<&T> {
        if !self.contains(element) {
            return None
        }
        Some(&self.dense_objects[self.sparse[element]])
    }

    pub fn get_mut(&mut self, element: usize) -> Option<&mut T> {
        if !self.contains(element) {
            return None
        }
        Some(&mut self.dense_objects[self.sparse[element]])
    }

    pub fn get_all_elements(&self) -> Vec<usize> {
        self.sparse.iter().filter(|s| { **s != self.tombstone }).copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SPARSE_SET_TEST_SIZE: usize = 100;

    #[test]
    fn test_push() {
        let mut set = SparseSet::new(SPARSE_SET_TEST_SIZE);
        for i in 0..SPARSE_SET_TEST_SIZE {
            set.push(i, 2*i);
            assert_eq!(set.dense[i], i);
            assert_eq!(set.dense_objects[i], 2*i);
        }
    }

    #[test]
    fn test_remove() {
        let mut set = SparseSet::new(SPARSE_SET_TEST_SIZE);
        for i in 0..SPARSE_SET_TEST_SIZE {
            set.push(i, i);
        }

        for i in (SPARSE_SET_TEST_SIZE/2)..(SPARSE_SET_TEST_SIZE) {
            assert_eq!(set.remove(i), (i, Some(i)));
        }

        assert_eq!(set.dense.len(), SPARSE_SET_TEST_SIZE/2);
        assert_eq!(set.remove(SPARSE_SET_TEST_SIZE + 1), (set.tombstone, None));
    }

    #[test]
    fn test_contains() {
        let mut set = SparseSet::new(SPARSE_SET_TEST_SIZE);
        for i in 0..SPARSE_SET_TEST_SIZE/2 {
            set.push(2 * i, 4 * i);
        }

        assert_eq!(set.contains(1), false);
        assert_eq!(set.contains(98), true);
        assert_eq!(set.contains(SPARSE_SET_TEST_SIZE + 1), false);
    }

    #[test]
    fn test_get() {
        let mut set = SparseSet::new(SPARSE_SET_TEST_SIZE);
        for i in 0..SPARSE_SET_TEST_SIZE {
            set.push(i, 3 * i);
        }

        for i in 0..SPARSE_SET_TEST_SIZE {
            assert_eq!(*set.get(i).unwrap(), 3 * i);
        }

        for i in 0..SPARSE_SET_TEST_SIZE {
            *set.get_mut(i).unwrap() *= 2;
        }

        for i in 0..SPARSE_SET_TEST_SIZE {
            assert_eq!(*set.get(i).unwrap(), i * 6);
        }
    }
}
