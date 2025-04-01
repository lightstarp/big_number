
fn add(a: [u32; 64], b: [u32; 64]) -> [u32; 64] {
    
    let mut t64: [u64; 128] = [0; 128];
    for i in 0..64 {
        t64[i] = a[i] as u64 + b[i] as u64;
    }

    fix(t64)
}

fn mul(a: [u32; 64], b: [u32; 64]) -> [u32; 64] {
    
    let mut t64: [u64; 128] = [0; 128];
    for i in 0..64 {
        for j in 0..64 {
            t64[i+j] += a[i] as u64 * b[j] as u64;
        }
    }

    fix(t64)
}

fn fix(t64: [u64; 128]) -> [u32; 64] {

    let mut t32: [u32; 64] = [0; 64];
    let mut d: u32 = 0;
    let mut t: u64;

    for i in 0..64 {
        t = t64[i] + d as u64;
        t32[i] = t as u32;
        d = (t >> 32) as u32;
    }
    t32
}

fn d_add(a: Ratinal, b: Ratinal) -> Ratinal {
    let mut r = build_ratinal(1,1);
    r.dividend = b.divisor * a.dividend + a.divisor * b.dividend;
    r.divisor = a.divisor * b.divisor;
    r
}
fn d_sub(a: Ratinal, b: Ratinal) -> Ratinal {
    let mut r = build_ratinal(1,1);
    r.dividend = b.divisor * a.dividend - a.divisor * b.dividend;
    r.divisor = a.divisor * b.divisor;
    r
}
fn d_mul(a: Ratinal, b: Ratinal) -> Ratinal {
    let mut r = build_ratinal(1,1);
    r.dividend = a.dividend * b.dividend;
    r.divisor = a.divisor * b.divisor;
    r
}
fn d_div(a: Ratinal, b: Ratinal) -> Ratinal {
    let mut r = build_ratinal(1,1);
    r.dividend = a.dividend * b.divisor;
    r.divisor = a.divisor * b.dividend;
    r
}
fn d_reduce(a: Ratinal) -> Ratinal {
    if a.dividend % a.divisor == 0 {
        let r = build_ratinal(a.dividend / a.divisor,1);
        r
    }
    else
    {
        let mut r = build_ratinal(a.dividend, a.divisor);
        let mut i: i32 = 2;
        while i <= 100 {
            if r.dividend % i == 0 && r.divisor % i == 0 {
                r.dividend = r.dividend / i;
                r.divisor = r.divisor / i;
            }
            else
            {
                i += 1;
            }
        }
        r
    }
}

#[derive(Clone)]
struct Ratinal{
    dividend: i32,
    divisor: i32,
}

#[derive(Clone)]
struct Num{
    r: Ratinal,
    i: bool,
    s: String,
}

fn build_ratinal(dividend: i32,divisor: i32) -> Ratinal {
    Ratinal{
        dividend,
        divisor,
    }
}

fn build_num(r: Ratinal, i: bool, s: String) -> Num {
    Num{
        r,
        i,
        s,
    }
}

fn num_to_str(num: Vec<Num>) -> String {
    let mut tmp: String = format!("{}","");
    for i in 0..num.len() {
        if num[i].i {
            tmp = format!("{}{:10}", tmp, num[i].s);
        }
        else{
            tmp = format!("{}{:10}", tmp, "");
        }
    }
    tmp
}

fn to_str(num: Ratinal) -> String {
    if num.dividend % num.divisor == 0 {
        return format!("{}", num.dividend / num.divisor);
    }
    else
    {
        return format!("{}/{}", num.dividend, num.divisor);
    }
}

fn saiki(t: Vec<Num>){
    //print!("{:5}","Ok");
    let mut count: i32 = 0;
    let mut idx: usize = 0;
    for i in 0..t.len() {
        if t[i].i {
            idx = i;
            count += 1;
        }
    }
    if count == 1 {
        if t[idx].r.dividend == 10 && t[idx].r.divisor == 1 {
            println!("OK : {:10}",t[idx].s);
        }
    }
    else{
        for x in 0..t.len()-1 {
            if t[x].i {
            for y in x+1..t.len() {
                if t[y].i {
                for c in 0..=3 {
                    let mut clone: Vec<Num> = t.clone();
                    match c {
                        0 => {
                            clone[x].r = d_add(t[x].r.clone(),t[y].r.clone());
                            clone[y].i = false;
                            clone[x].s = format!("({}+{})",t[x].s,t[y].s);
                        }
                        1 => {
                            clone[x].r = d_sub(t[x].r.clone(),t[y].r.clone());
                            clone[y].i = false;
                            clone[x].s = format!("({}-{})",t[x].s,t[y].s);
                        }
                        2 => {
                            clone[x].r = d_mul(t[x].r.clone(),t[y].r.clone());
                            clone[y].i = false;
                            clone[x].s = format!("({}*{})",t[x].s,t[y].s);
                        }
                        3 => {
                            if t[y].r.dividend != 0 {
                                //println!("Div : {:10},{:10}",to_str(t[x].r.clone()),to_str(t[y].r.clone()));
                                clone[x].r = d_div(t[x].r.clone(),t[y].r.clone());
                                clone[y].i = false;
                                clone[x].s = format!("({}/{})",t[x].s,t[y].s);
                            }
                        }
                        _ => {}
                    }
                    if t[y].r.dividend != 0 {
                    clone[x].r = d_reduce(clone[x].r.clone());
                    saiki(clone.clone());
                    }
                }
                }
            }
            }
        }
    }
}

fn kaiseki(m: Vec<Ratinal>){/*後で変える*/
    
    let mut t: Vec<Num> = vec![build_num(build_ratinal(1,1),true,format!("{}","aaa")); m.len()];
    for i in 0..m.len(){
        t[i].r = m[i].clone();
        t[i].s = format!("{}",m[i].dividend.clone())
    }
    saiki(t.clone())
}

fn init() {
    let mut t: Vec<Ratinal> = vec![build_ratinal(0,1); 4];
    t[0] = build_ratinal(4,1);
    t[1] = build_ratinal(6,1);
    t[2] = build_ratinal(6,1);
    t[3] = build_ratinal(9,1);
    //t[5] = build_ratinal(7,1);
    //t[6] = build_ratinal(7,1);
    kaiseki(t);

    for x in 1..10 {
        for y in 1..10 {

            let mut a = build_ratinal(x,1);
            let mut b = build_ratinal(y,1);
            a = d_div(a,b);
            a = d_add(a,build_ratinal(1,1));
            a = d_reduce(a);
        
            //print!("{:5}",to_str(a));
        }
        //println!("")
    }
}

fn main() {
    println!("TEST");
    println!("TEST");
}   
