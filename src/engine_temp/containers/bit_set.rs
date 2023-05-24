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
use std::mem;
use std::ops::{
    BitAnd, BitOr, BitXor,
    BitAndAssign, BitOrAssign, BitXorAssign
};

type InnerBit = u128;

/// A bitset of arbitrary length
/// Internally allocates a u128 for each allocation
pub struct BitSet {
    bits: Vec<InnerBit>
}

impl BitSet {
    const INNER_BIT_LEN: usize = mem::size_of::<InnerBit>();

    pub fn new(byte_count: usize) -> BitSet {
        let mut bits = Vec::new();
        bits.resize((byte_count as f64 / BitSet::INNER_BIT_LEN as f64) as usize, 0);
        BitSet {
            bits 
        }
    }

    const fn get_index(index: usize, byte_count: usize) -> (usize, usize) {
        let inner_bit_index = index / byte_count;
        let sub_index = index - inner_bit_index * BitSet::INNER_BIT_LEN;

        (inner_bit_index, sub_index)
    }

    pub fn flip(&mut self, bit_index: usize) {
        let (inner_bit_index, sub_index) = BitSet::get_index(bit_index, self.bits.len());
        if inner_bit_index >= self.bits.len() {
            panic!("Attempting to set bit at index outside of range of bitset!")
        }
        self.bits[inner_bit_index] ^= 1 << sub_index;
    }

    /// set the bit to 1 at index
    pub fn set(&mut self, bit_index: usize) {
        let (inner_bit_index, sub_index) = BitSet::get_index(bit_index, self.bits.len());
        if inner_bit_index >= self.bits.len() {
            panic!("Attempting to set bit at index outside of range of bitset!")
        }
        self.bits[inner_bit_index] |= 1 << sub_index;
    }

    pub fn unset(&mut self, bit_index: usize) {
        let (inner_bit_index, sub_index) = BitSet::get_index(bit_index, self.bits.len());
        if inner_bit_index >= self.bits.len() {
            panic!("Attempting to set bit at index outside of range of bitset!")
        }
        self.bits[inner_bit_index] &= !(1 << sub_index);
    }
}

impl BitAnd for BitSet {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let min_size = self.bits.len().min(rhs.bits.len());
        let mut new_set = BitSet::new(min_size);

        // For each "byte" in the bitset, AND the two together and put into the new bitset
        for i in 0..min_size {
            let lhs_value = self.bits[i];
            let rhs_value = rhs.bits[i];
        
            new_set.bits[i] = lhs_value & rhs_value;
        }

        new_set
    }
}

impl BitOr for BitSet {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let max_size = self.bits.len().max(rhs.bits.len());
        let min_size = self.bits.len().min(rhs.bits.len());
        let mut new_set = BitSet::new(max_size);

        for i in 0..max_size {
            if i >= min_size {
                new_set.bits[i] = if i < self.bits.len() {
                    self.bits[i]
                } else {
                    rhs.bits[i]
                }
            } else {
                let lhs_value = self.bits[i];
                let rhs_value = rhs.bits[i];
            
                new_set.bits[i] = lhs_value | rhs_value;
            }
        }

        new_set
    }
}

impl BitXor for BitSet {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        let max_size = self.bits.len().max(rhs.bits.len());
        let min_size = self.bits.len().min(rhs.bits.len());
        let mut new_set = BitSet::new(max_size);

        for i in 0..max_size {
            if i >= min_size {
                new_set.bits[i] = if i < self.bits.len() {
                    self.bits[i]
                } else {
                    rhs.bits[i]
                }
            } else {
                let lhs_value = self.bits[i];
                let rhs_value = rhs.bits[i];
            
                new_set.bits[i] = lhs_value ^ rhs_value;
            }
        }

        new_set
    }
}
