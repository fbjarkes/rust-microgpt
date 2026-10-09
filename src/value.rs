

struct Value<'a> {
    data: f32,
    grad: f32,

    children: Option<(&'a Value<'a>, &'a Value<'a>)>,
    local_grads: Option<(f32, f32)>,
}

impl<'a> Value<'a> {
    fn new(data: f32, children: Option<(&'a Value<'a>, &'a Value<'a>)>, local_grads: Option<(f32, f32)>) -> Self {
        Self {
            data,
            grad: 0.0,
            children,
            local_grads,
        }
    }

    fn mul(&'a self, other: &'a Value<'a>) -> Value<'a> {
        Value {
            data: self.data * other.data,
            grad: 0.0,
            children: Some((self, other)),
            local_grads: Some((other.data, self.data)),
        }
    }

    fn add(&'a self, other: &'a Value<'a>) -> Value<'a> {
        Value {
            data: self.data + other.data,
            grad: 0.0,
            children: Some((self, other)),
            local_grads: Some((1.0, 1.0)),
        }
    }

    fn backward(&mut self) {
        self.grad += 1.0;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_add() {
        let mut a = Value::new(2.0, None, None);
        let mut b = Value::new(3.0, None, None);
        let mut c = a.add(&b);
        assert!(c.children.is_some());
        assert_eq!(c.local_grads, Some((1.0, 1.0)));
    }

    #[test]
    fn test_mul() {
        let mut a = Value::new(2.0, None, None);
        let mut b = Value::new(3.0, None, None);
        let mut c = a.mul(&b);
        assert!(c.children.is_some());
        assert_eq!(c.local_grads, Some((3.0, 2.0)));
    }
    
    #[test]
    fn test_backward() {
        let mut a = Value::new(2.0, None, None);
        let mut b = Value::new(3.0, None, None);
     
        let mut c = a.mul(&b);
        assert!(c.children.is_some());
        assert_eq!(c.local_grads, Some((3.0, 2.0)));
        c.backward();
        //TODO:
    }   
        
}