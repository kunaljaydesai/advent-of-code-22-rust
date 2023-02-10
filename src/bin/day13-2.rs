use std::{cell::RefCell, cmp::Ordering, io::BufRead, rc::Rc};

enum Type {
    Vec,
    Int,
}

enum CorrectOrder {
    True,
    False,
    Continue,
}

struct VecInt {
    vec: Vec<VecIntRef>,
    integer: u32,
    val_type: Type,
}

type VecIntRef = Rc<RefCell<VecInt>>;

impl VecInt {
    pub fn to_str(&self) -> String {
        if matches!(self.val_type, Type::Vec) {
            let mut surrounding_array = "[".to_string();
            let mut idx = 0;
            loop {
                if idx >= self.vec.len() {
                    break;
                }
                let e = &*Rc::clone(&self.vec[idx]);
                surrounding_array += &e.borrow().to_str();
                idx += 1;
            }
            surrounding_array += "]";
            return surrounding_array;
        } else {
            return self.integer.to_string() + ",";
        }
    }

    pub fn from_str(s: String) -> Option<VecIntRef> {
        if s.len() == 0 {
            return None;
        }
        let mut stack: Vec<VecIntRef> = vec![];
        let mut parent: Option<VecIntRef> = None;
        let mut chars = s.chars();
        while let Some(char) = chars.next() {
            if char == '[' {
                let new_vec = Rc::new(RefCell::new(VecInt {
                    vec: Vec::new(),
                    integer: 0,
                    val_type: Type::Vec,
                }));
                if parent.is_none() {
                    parent = Some(Rc::clone(&new_vec));
                }
                let stack_len = stack.len();
                if stack_len > 0 {
                    let last_elem = &*stack[stack_len - 1];
                    last_elem.borrow_mut().vec.push(Rc::clone(&new_vec));
                }
                stack.push(Rc::clone(&new_vec));
            } else if char == ']' {
                stack.pop();
            } else if char.is_digit(10) {
                let mut digit_char = char.to_string();
                let mut should_pop_stack = false;
                while let Some(dc) = chars.next() {
                    if !dc.is_digit(10) {
                        if dc == ']' {
                            should_pop_stack = true;
                        }
                        break;
                    }
                    digit_char += &dc.to_string();
                }

                let stack_len = stack.len();
                let last_elem = &*stack[stack_len - 1];
                last_elem
                    .borrow_mut()
                    .vec
                    .push(Rc::new(RefCell::new(VecInt {
                        integer: digit_char.parse::<u32>().ok().unwrap(),
                        vec: vec![],
                        val_type: Type::Int,
                    })));

                if should_pop_stack {
                    stack.pop();
                }
            }
        }
        return parent;
    }

    fn is_in_right_order(left: Rc<RefCell<VecInt>>, right: Rc<RefCell<VecInt>>) -> CorrectOrder {
        let left_vec_int = (*left).borrow();
        let right_vec_int = (*right).borrow();

        // same types
        if matches!(left_vec_int.val_type, Type::Int) && matches!(right_vec_int.val_type, Type::Int)
        {
            if left_vec_int.integer < right_vec_int.integer {
                return CorrectOrder::True;
            } else if left_vec_int.integer == right_vec_int.integer {
                return CorrectOrder::Continue;
            } else {
                return CorrectOrder::False;
            }
        }

        if matches!(left_vec_int.val_type, Type::Vec) && matches!(right_vec_int.val_type, Type::Vec)
        {
            let mut idx = 0;
            while idx < left_vec_int.vec.len() && idx < right_vec_int.vec.len() {
                let order = VecInt::is_in_right_order(
                    Rc::clone(&left_vec_int.vec[idx]),
                    Rc::clone(&right_vec_int.vec[idx]),
                );
                if matches!(order, CorrectOrder::True) {
                    return CorrectOrder::True;
                }
                if matches!(order, CorrectOrder::False) {
                    return CorrectOrder::False;
                }
                idx += 1;
            }
            if left_vec_int.vec.len() < right_vec_int.vec.len() {
                return CorrectOrder::True;
            } else if left_vec_int.vec.len() == right_vec_int.vec.len() {
                return CorrectOrder::Continue;
            } else {
                return CorrectOrder::False;
            }
        }

        if matches!(left_vec_int.val_type, Type::Int) && matches!(right_vec_int.val_type, Type::Vec)
        {
            let wrapped_left = VecInt {
                vec: vec![Rc::clone(&left)],
                integer: 0,
                val_type: Type::Vec,
            };
            return VecInt::is_in_right_order(
                Rc::new(RefCell::new(wrapped_left)),
                Rc::clone(&right),
            );
        }

        if matches!(right_vec_int.val_type, Type::Int) && matches!(left_vec_int.val_type, Type::Vec)
        {
            let wrapped_right = VecInt {
                vec: vec![Rc::clone(&right)],
                integer: 0,
                val_type: Type::Vec,
            };
            return VecInt::is_in_right_order(
                Rc::clone(&left),
                Rc::new(RefCell::new(wrapped_right)),
            );
        }
        CorrectOrder::False
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day13.txt");
    let file = std::fs::File::open(path)?;
    let mut lines = std::io::BufReader::new(file).lines();
    let mut packets = vec![];
    while let Some(line) = lines.next() {
        let str = line.ok().unwrap();
        if str.len() == 0 {
            continue;
        }
        let vec_int = VecInt::from_str(str).unwrap();
        packets.push(vec_int);
    }
    packets.sort_by(|right, left| {
        let order = VecInt::is_in_right_order(Rc::clone(right), Rc::clone(left));
        if matches!(order, CorrectOrder::True) {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });
    let mut idx = 1;
    for packet in packets {
        let packet_b = (*packet).borrow();
        println!("idx {} packet {}", idx, packet_b.to_str());
        idx += 1;
    }
    Ok(())
}
