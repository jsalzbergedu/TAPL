use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Lambda {
    Variable(String),
    Abstraction(String, Box<Self>),
    Application(Box<Self>, Box<Self>)
}

fn free<'a>(obj: Lambda) -> HashSet<String> {
    match obj {
        Lambda::Variable(s) => { let mut set = HashSet::new(); set.insert(s); set },
        Lambda::Abstraction(s, term) => { let mut set = free(*term); set.remove(&s); set },
        Lambda::Application(term1, term2) => { let mut set1 = free(*term1); let set2 = free(*term2); set1.extend(set2.iter().cloned()); set1 }
    }
}

fn alpha_reduce(obj: Lambda) -> Lambda {
    match obj {
        Lambda::Variable(s) => {Lambda::Variable(format!("{}{}", &s, "'"))},
        Lambda::Abstraction(s, term) => { Lambda::Abstraction(format!("{}{}", &s, "'"), Box::new(alpha_reduce(*term))) },
        Lambda::Application(term1, term2) => { Lambda::Application(Box::new(alpha_reduce(*term1)), Box::new(alpha_reduce(*term2))) }
    }
}

/// Replace all free occurrences of x with y in obj
fn substitute(obj: Lambda, x: &str, y: Lambda) -> Lambda {
    match obj {
        Lambda::Variable(ref s) => { if x.eq(s) { y } else { obj } },
        Lambda::Application(term1, term2) => Lambda::Application(Box::new(substitute(*term1, x, y.clone())), Box::new(substitute(*term2, x, y))),
        Lambda::Abstraction(ref s, ref term) => { if x.eq(s) { *term.clone() } else if !free(y.clone()).contains(s) { Lambda::Abstraction(s.to_owned(), Box::new(substitute(*term.clone(), x, y))) } else { substitute(alpha_reduce(obj.clone()), x, y) } }
    }
}

fn reduce(obj: Lambda) -> Lambda {
    match obj.clone() {
        Lambda::Variable(_) => obj,
        Lambda::Abstraction(_, _) => obj,
        Lambda::Application(term1, term2) => {
            match *term1 {
                Lambda::Variable(_) => obj,
                Lambda::Application(_, _) => Lambda::Application(Box::new(reduce(*term1)), Box::new(*term2)),
                Lambda::Abstraction(ref s, term) => substitute(*term, s, *term2)
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let id = Lambda::Application(Box::new(Lambda::Abstraction("x".to_owned(), Box::new(Lambda::Variable("x".to_owned())))), Box::new(Lambda::Variable("y".to_owned())));
    println!("old {:?}", id);
    println!("new {:?}", reduce(id));
}
